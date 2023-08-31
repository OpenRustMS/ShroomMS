use std::fmt::Debug;

use crate::services::helper::intentory::MAX_CAP;

use super::{
    slot::InventorySlotIndex, sorted_array_vec::SortedArrayVec, InventoryError, InventoryItem,
    ItemId,
};

pub trait InventoryExt<const CAP: usize> {
    type Slot: InventorySlotIndex;
    type Item: InventoryItem + Debug;

    fn get_inner(&self) -> &Inventory<Self::Item, CAP>;
    fn get_inner_mut(&mut self) -> &mut Inventory<Self::Item, CAP>;

    fn len(&self) -> usize {
        self.get_inner().len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_not_empty(&self) -> bool {
        self.len() != 0
    }

    fn slots(&self) -> usize {
        self.get_inner().slots()
    }

    fn remove(&mut self, slot: Self::Slot) -> Option<Self::Item> {
        self.get_inner_mut().remove(slot.to_index()).unwrap()
    }

    fn swap(&mut self, slot_a: Self::Slot, slot_b: Self::Slot) {
        self.get_inner_mut()
            .swap(slot_a.to_index(), slot_b.to_index())
            .unwrap();
    }

    fn set(&mut self, slot: Self::Slot, item: Self::Item) {
        self.get_inner_mut().set_slot(slot.to_index(), item)
    }

    fn replace(
        &mut self,
        slot: Self::Slot,
        item: Self::Item,
    ) -> Result<Option<Self::Item>, InventoryError> {
        self.get_inner_mut().replace(slot.to_index(), item)
    }

    fn get_mut(&mut self, slot: Self::Slot) -> Option<&mut Self::Item> {
        self.get_inner_mut().get_mut(slot.to_index()).ok()
    }

    fn get(&self, slot: Self::Slot) -> Option<&Self::Item> {
        self.get_inner().get(slot.to_index()).ok()
    }

    fn try_insert(&mut self, item: Self::Item) -> Result<usize, Self::Item> {
        self.get_inner_mut().try_insert(item)
    }

    fn load(
        &mut self,
        items: impl Iterator<Item = (Self::Slot, Self::Item)>,
    ) -> Result<(), InventoryError> {
        for (slot, item) in items {
            self.set(slot, item);
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Inventory<Item, const CAP: usize> {
    pub(crate) slots: usize,
    pub(crate) items: SortedArrayVec<Item, CAP>,
    pub(crate) slot_mapping: [Option<u8>; CAP],
}

impl<const CAP: usize, Item> Debug for Inventory<Item, CAP>
where
    Item: Debug + InventoryItem,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.items_with_slots().map(|(_, item)| item))
            .finish()
    }
}

impl<const CAP: usize, Item> Inventory<Item, CAP>
where
    Item: InventoryItem,
{
    pub fn new(slots: usize) -> Self {
        assert!(
            CAP <= MAX_CAP,
            "Inventory Capacity {CAP} higher than the maximum CAP: {MAX_CAP}"
        );
        assert!(
            slots <= CAP,
            "Inventory slots({slots}) must be lower than the Capacity({CAP})"
        );

        Self {
            slots,
            slot_mapping: [None; CAP],
            items: SortedArrayVec::default(),
        }
    }

    /// Verifies that a slot is valid
    fn check_slot(&self, slot: usize) -> Result<(), InventoryError> {
        if slot >= self.slots {
            return Err(InventoryError::OutOfRange(slot));
        }

        Ok(())
    }

    /// Gets the mapped index for the slot
    fn get_mapped_slot(&self, slot: usize) -> Result<Option<usize>, InventoryError> {
        self.check_slot(slot)?;
        Ok(self.slot_mapping[slot].map(|s| s as usize))
    }

    fn update_add(&mut self, add_index: usize) {
        let add_index = add_index as u8;
        self.slot_mapping
            .iter_mut()
            .filter_map(|ix| ix.as_mut())
            .for_each(|ix| {
                if *ix >= add_index {
                    *ix += 1;
                }
            })
    }

    fn update_remove(&mut self, rm_index: usize) {
        let rm_index = rm_index as u8;
        self.slot_mapping
            .iter_mut()
            .filter_map(|ix| ix.as_mut())
            .for_each(|ix| {
                if *ix > rm_index {
                    *ix -= 1;
                }
            })
    }

    fn inner_add(&mut self, slot: usize, item: Item) {
        let ix = self.items.add(item);
        self.update_add(ix);
        self.slot_mapping[slot] = Some(ix as u8);
    }

    fn inner_remove(&mut self, slot: usize) -> Option<Item> {
        self.slot_mapping[slot].take().map(|ix| {
            self.update_remove(ix as usize);
            self.items.remove(ix as usize)
        })
    }

    /// Returns all items in the inventory
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// returns the amount of slots for this inventory
    pub fn slots(&self) -> usize {
        self.slots
    }

    /// Checks whether the inventory is full
    pub fn is_full(&self) -> bool {
        self.items.len() <= self.slots
    }

    /// Checks whether the inventory is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Finds the first free slot index in the inventory
    pub fn find_free_slot(&self) -> Option<usize> {
        self.slot_mapping
            .iter()
            .take(self.slots)
            .position(Option::is_none)
    }

    /// Gets a reference to the slot
    pub fn get_slot(&self, slot: usize) -> Result<Option<&Item>, InventoryError> {
        Ok(self.get_mapped_slot(slot)?.map(|ix| self.items.get(ix)))
    }

    /// Gets a reference to the slot
    pub fn get_mut_slot(&mut self, slot: usize) -> Result<Option<&mut Item>, InventoryError> {
        Ok(self.get_mapped_slot(slot)?.map(|ix| self.items.get_mut(ix)))
    }

    /// Returns a reference to the item at the given slot
    pub fn get(&self, slot: usize) -> Result<&Item, InventoryError> {
        self.get_slot(slot)?
            .ok_or(InventoryError::EmptySlot(slot))
    }

    /// Returns a mut reference to the item at the given slot
    pub fn get_mut(&mut self, slot: usize) -> Result<&mut Item, InventoryError> {
        self.get_mut_slot(slot)?
            .ok_or(InventoryError::EmptySlot(slot))
    }

    /// Attempts to insert an item into the inventory
    /// If item can be inserted, It will be create through `create_item`
    /// otherwise returns an error
    pub fn try_insert_with(
        &mut self,
        create_item: impl FnOnce() -> Item,
    ) -> Result<usize, InventoryError> {
        let Some(free_slot) = self.find_free_slot()  else {
            return Err(InventoryError::Full);
        };

        // TODO checking constraing maybe on item id
        let item = create_item();
        if !self.can_insert_item(&item) {
            return Err(InventoryError::Full);
        }

        self.inner_add(free_slot, item);
        Ok(free_slot)
    }

    /// Attempts to insert an item into the inventory
    /// Returns either the slot where the item was inserted
    /// or gives back the item in case the inventory was full
    pub fn try_insert(&mut self, item: Item) -> Result<usize, Item> {
        let Some(free_slot) = self.find_free_slot()  else {
            return Err(item);
        };

        if !self.can_insert_item(&item) {
            return Err(item);
        }

        self.inner_add(free_slot, item);
        Ok(free_slot)
    }

    /// Attempts to take an item out of the inventory otherwise errors
    pub fn take(&mut self, slot: usize) -> Result<Item, InventoryError> {
        self.check_slot(slot)?;
        self.inner_remove(slot)
            .ok_or(InventoryError::EmptySlot(slot))
    }

    /// Removes an item as the given slot
    pub fn remove(&mut self, slot: usize) -> Result<Option<Item>, InventoryError> {
        self.check_slot(slot)?;
        Ok(self.inner_remove(slot))
    }

    // Attempts to swap the slot with the other inventory
    pub fn swap_with_other_inv(
        &mut self,
        slot_a: usize,
        other: &mut Self,
        slot_other: usize,
    ) -> Result<(), InventoryError> {
        let item_a = self.get_slot(slot_a)?;
        if let Some(item) = item_a {
            if !other.can_insert_item(item) {
                return Err(InventoryError::OneOfAKindConflict(item.id()));
            }
        }

        let item_b = other.get_slot(slot_other)?;
        if let Some(item) = item_b {
            if !self.can_insert_item(item) {
                return Err(InventoryError::OneOfAKindConflict(item.id()));
            }
        }

        let item_a = self.remove(slot_a)?;
        let item_b = other.remove(slot_other)?;

        if let Some(item) = item_a {
            other.set_slot(slot_other, item);
        }

        if let Some(item) = item_b {
            self.set_slot(slot_a, item);
        }

        Ok(())
    }

    /// Swaps two slots
    pub fn swap(&mut self, slot_a: usize, slot_b: usize) -> Result<(), InventoryError> {
        self.check_slot(slot_a)?;
        self.check_slot(slot_b)?;

        self.slot_mapping.swap(slot_a, slot_b);
        Ok(())
    }

    /// Check if the inventory contains the specified Id
    pub fn contains_id(&self, id: ItemId) -> bool {
        self.items.contains_key(id)
    }

    /// Attempts to set the specified slot
    pub fn try_set_slot(&mut self, slot: usize, item: Item) -> Result<(), Item> {
        // Ensure the slot is valid
        if self.check_slot(slot).is_err() {
            return Err(item);
        }

        // Ensure the slot is empty
        if self.slot_mapping[slot].is_some() {
            return Err(item);
        }

        self.inner_add(slot, item);
        Ok(())
    }

    pub fn set_slot(&mut self, slot: usize, item: Item) {
        self.try_set_slot(slot, item)
            .map_err(|_| ())
            .expect("set slot");
    }

    pub fn replace(&mut self, slot: usize, item: Item) -> Result<Option<Item>, InventoryError> {
        let take = self.remove(slot)?;
        self.try_set_slot(slot, item)
            .map_err(|_| ())
            .expect("set slot");

        Ok(take)
    }

    pub fn can_insert_item(&self, item: &Item) -> bool {
        if item.is_one_of_a_kind() {
            !self.contains_id(item.id())
        } else {
            true
        }
    }

    pub fn find_first_shift_slot(&self, after_slot: usize) -> Option<usize> {
        self.slot_mapping
            .iter()
            .skip(after_slot)
            .position(|slot| slot.is_some())
    }

    /// Compacts the inventory, as in removing gaps
    pub fn compact(&mut self) {
        // Stores the last known position where an item was
        let mut last_shift_ix = 0;
        for gap in 0..CAP {
            //Check if this  is a gap
            if self.slot_mapping[gap].is_none() {
                let after_slot = last_shift_ix.max(gap + 1);
                //Find shift item
                let Some(shift_ix) = self.find_first_shift_slot(after_slot) else {
                    // No more items available
                    return;
                };

                //Indices are checked must work
                self.slot_mapping.swap(gap, shift_ix);
                last_shift_ix = shift_ix;
            }
        }
    }

    /// Sorts the inventory, by the item Id
    pub fn sort(&mut self) {
        // We know the underlying item array is already sorted
        // so sorting the inventory is as simple
        // as sorting the mappings by the index
        for slot in 0..self.slots {
            // If the index is set to a slot then we update it's slot
            let Some(ix) = self.slot_mapping[slot] else {
                continue
            };

            let ix = ix as usize;

            if ix != slot {
                self.slot_mapping.swap(slot, ix)
                //Log swap here
            }
        }
    }

    pub fn as_slice(&self) -> &[Item] {
        self.items.as_ref()
    }

    pub fn slot_index_of_item<'a>(&'a self, item: &'a Item) -> usize {
        let ix = self.items.index_of(item);
        self.slot_mapping
            .iter()
            .position(|slot| slot == &Some(ix as u8))
            .expect("Slot index")
    }

    /// Returns an iterator over all items with a specific id
    pub fn items_by_id(&self, id: ItemId) -> impl Iterator<Item = &Item> + '_ {
        self.items.items_by_key(id)
    }

    /// Returns an iterator over all items with a specific id
    pub fn items_with_slot_by_id(&self, id: ItemId) -> impl Iterator<Item = (usize, &Item)> + '_ {
        self.items.enumerate_items_by_key(id)
    }

    /// Returns an iterator over all items with a specific id
    pub fn items_with_slot_by_id_mut(
        &mut self,
        id: ItemId,
    ) -> impl Iterator<Item = (usize, &mut Item)> + '_ {
        self.items.enumerate_items_by_key_mut(id)
    }

    /// Returns an iterator over all items with a specific id
    pub fn items_by_id_mut(&mut self, id: ItemId) -> impl Iterator<Item = &mut Item> + '_ {
        self.items.items_by_key_mut(id)
    }

    /// An Iterator over all items
    pub fn items(&self) -> impl Iterator<Item = &Item> + '_ {
        self.items.iter()
    }

    ///A mutable Iterator over all items
    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut Item> + '_ {
        self.items.iter_mut()
    }

    /// An Iterator with all items and their slot
    pub fn items_with_slots(&self) -> impl Iterator<Item = (usize, &Item)> + '_ {
        self.slot_mapping
            .iter()
            .enumerate()
            .filter_map(|(slot, ix)| ix.map(|ix| (slot, self.items.get(ix as usize))))
    }
}

#[cfg(test)]
mod tests {
    use crate::services::helper::intentory::ItemId;

    use super::{Inventory, InventoryItem};

    impl InventoryItem for ItemId {
        fn is_one_of_a_kind(&self) -> bool {
            //Even IDs are one-of-a-kind
            self % 2 == 0
        }
        fn id(&self) -> ItemId {
            *self
        }
    }
    #[test]
    fn inventory_one_of_a_kind() {
        const SLOTS: usize = 4;
        let mut inv = Inventory::<u32, 8>::new(SLOTS);

        // Odd item id works
        inv.try_insert(1).unwrap();
        inv.try_insert(1).unwrap();

        // Even is one-of-a-kind
        inv.try_insert(2).unwrap();
        assert!(!inv.can_insert_item(&2));
        assert!(inv.try_insert(2).is_err());
    }

    #[test]
    fn test_insert() {
        const SLOTS: usize = 4;
        let mut inv = Inventory::<u32, 8>::new(SLOTS);

        for i in (1..=4).rev() {
            inv.try_insert(i).unwrap();
        }

        assert!(inv.is_full());
        itertools::assert_equal(inv.items().cloned(), [1, 2, 3, 4]);
        //assert!(inv.items.test_check_sorted());
    }
}
