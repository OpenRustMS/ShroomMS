pub mod index_map;
pub mod stack;

use index_map::IdIndexMap;

pub type InvResult<T> = Result<T, InvError>;

#[derive(Debug, PartialEq, Eq)]
pub enum InvError {
    UniqueConflict,
    Full,
    SlotFull,
    SlotInsufficentSpace,
    InvalidSlot(usize),
    EmptySlot(usize),
    InsufficentItems(usize),
}

pub trait InvItemId: Eq + std::hash::Hash + Copy + Clone + Default {
    fn is_unique(&self) -> bool;
}

pub trait InvItem {
    type Id: InvItemId;
    fn id(&self) -> Self::Id;
}

pub struct Inv<Item: InvItem, const CAP: usize> {
    slots: [Option<Item>; CAP],
    len: usize,
    ids: IdIndexMap<Item::Id>,
}

impl<Item: InvItem, const CAP: usize> Default for Inv<Item, CAP> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<Item: InvItem, const CAP: usize> Inv<Item, CAP> {
    pub fn empty() -> Self {
        Self {
            slots: array_init::array_init(|_| None),
            len: 0,
            ids: IdIndexMap::default(),
        }
    }

    fn check_slot(&self, slot: usize) -> InvResult<()> {
        if slot >= self.slots.len() {
            return Err(InvError::InvalidSlot(slot));
        }

        Ok(())
    }

    pub fn check_full(&self) -> InvResult<()> {
        if self.len == self.slots.len() {
            return Err(InvError::Full);
        }

        Ok(())
    }

    fn check_unique(&self, id: &Item::Id) -> InvResult<()> {
        if id.is_unique() && self.ids.contains_id(id) {
            return Err(InvError::UniqueConflict);
        }

        Ok(())
    }

    fn find_free_slot(&mut self) -> Option<usize> {
        self.slots.iter().position(|slot| slot.is_none())
    }

    pub fn add(&mut self, item: Item) -> InvResult<usize> {
        self.check_full()?;
        let free_slot = self.find_free_slot().ok_or(InvError::Full)?;
        self.set(free_slot, item)?;
        Ok(free_slot)
    }

    pub fn set(&mut self, slot: usize, item: Item) -> InvResult<()> {
        self.check_slot(slot)?;
        self.check_unique(&item.id())?;

        if let Some(slot_item) = self.slots[slot].as_ref() {
            // Elsewise remove the id link
            self.ids.remove(slot_item.id(), slot);
        } else {
            // If the slot was empty we increment the count
            self.len += 1;
        }

        // Insert the slot into the map
        self.ids.insert(item.id(), slot);
        self.slots[slot] = Some(item);
        Ok(())
    }

    pub fn remove(&mut self, slot: usize) -> InvResult<Option<Item>> {
        Ok(if let Some(item) = self.slots[slot].take() {
            self.ids.remove(item.id(), slot);
            self.len -= 1;
            Some(item)
        } else {
            None
        })
    }

    pub fn take(&mut self, slot: usize) -> InvResult<Item> {
        self.remove(slot)?.ok_or(InvError::EmptySlot(slot))
    }

    pub fn swap(&mut self, slot_a: usize, slot_b: usize) -> InvResult<()> {
        self.check_slot(slot_a)?;
        self.check_slot(slot_b)?;
        self.slots.swap(slot_a, slot_b);

        // Item was moved from b to a
        if let Some(slot_a_item) = self.slots[slot_a].as_ref() {
            self.ids.update(&slot_a_item.id(), slot_b, slot_a)?;
        }

        // Item was moved from a to b
        if let Some(slot_b_item) = self.slots[slot_b].as_ref() {
            self.ids.update(&slot_b_item.id(), slot_a, slot_b)?;
        }

        Ok(())
    }

    pub fn get(&self, slot: usize) -> InvResult<&Item> {
        self.check_slot(slot)?;
        self.slots[slot].as_ref().ok_or(InvError::EmptySlot(slot))
    }

    pub fn get_mut(&mut self, slot: usize) -> InvResult<&mut Item> {
        self.check_slot(slot)?;
        self.slots[slot].as_mut().ok_or(InvError::EmptySlot(slot))
    }

    pub fn item_slots_by_id(&self, id: Item::Id) -> impl Iterator<Item = (usize, &Item)> + '_ {
        self.ids
            .item_slots_iter(id, &self.slots)
            .map(|(i, s)| (i, s.as_ref().unwrap()))
    }

    pub fn items_by_id(&self, id: Item::Id) -> impl Iterator<Item = &Item> + '_ {
        self.ids
            .item_slots_iter(id, &self.slots)
            .map(|(_, s)| s.as_ref().unwrap())
    }

    pub fn item_slots_by_id_mut(
        &mut self,
        id: Item::Id,
    ) -> impl Iterator<Item = (usize, &mut Item)> + '_ {
        unsafe {
            self.ids
                .item_slots_iter_mut(id, &mut self.slots)
                .map(|(i, s)| (i, s.as_mut().unwrap()))
        }
    }

    pub fn items_by_id_mut(&mut self, id: Item::Id) -> impl Iterator<Item = &mut Item> + '_ {
        unsafe {
            self.ids
                .item_slots_iter_mut(id, &mut self.slots)
                .map(|(_, s)| s.as_mut().unwrap())
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.slots.len()
    }

    pub fn free_slots(&self) -> usize {
        self.capacity() - self.len
    }

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
}

impl<Item, const CAP: usize> FromIterator<(usize, Item)> for Inv<Item, CAP>
where
    Item: InvItem + Default,
{
    fn from_iter<T: IntoIterator<Item = (usize, Item)>>(iter: T) -> Self {
        let mut inv = Inv::default();
        for (slot, item) in iter {
            inv.set(slot, item).unwrap();
        }
        inv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub struct DummyItem(u32);

    impl DummyItem {
        pub fn new(id: u32) -> Self {
            Self(id)
        }
    }

    impl InvItemId for u32 {
        fn is_unique(&self) -> bool {
            self % 2 == 0
        }
    }
    impl InvItem for DummyItem {
        type Id = u32;

        fn id(&self) -> Self::Id {
            self.0
        }
    }

    pub type Inv2<Item> = Inv<Item, 2>;
    pub type Inv4<Item> = Inv<Item, 4>;

    #[test]
    fn add_item_unique() {
        let mut inv = Inv2::empty();
        inv.add(DummyItem::new(0)).unwrap();
        assert_eq!(inv.len(), 1);

        assert_eq!(inv.add(DummyItem::new(0)), Err(InvError::UniqueConflict));
        assert_eq!(inv.len(), 1);

        // Item should be inserted in the first slot
        assert!(inv.get(0).is_ok())
    }

    #[test]
    fn add_item_full() {
        let mut inv = Inv2::empty();
        inv.add(DummyItem::new(0)).unwrap();
        inv.add(DummyItem::new(1)).unwrap();
        assert_eq!(inv.len(), 2);

        assert_eq!(inv.add(DummyItem::new(2)), Err(InvError::Full));
        assert_eq!(inv.len(), 2);
    }

    #[test]
    fn remove_item() {
        let mut inv = Inv2::empty();
        inv.add(DummyItem::new(0)).unwrap();
        inv.add(DummyItem::new(1)).unwrap();

        assert_eq!(inv.len(), 2);
        assert_eq!(inv.remove(0).unwrap().unwrap().id(), 0);
        assert_eq!(inv.len(), 1);
        assert_eq!(inv.remove(1).unwrap().unwrap().id(), 1);
        assert_eq!(inv.len(), 0);
    }

    #[test]
    fn swap() {
        let mut inv = Inv4::empty();
        inv.add(DummyItem::new(0)).unwrap();
        inv.add(DummyItem::new(1)).unwrap();

        assert_eq!(inv.items_by_id(0).count(), 1);
        assert_eq!(inv.items_by_id(1).count(), 1);

        // Empty slot swap works
        inv.swap(2, 3).unwrap();
        // Invalid slot is an error
        assert!(inv.swap(3, 4).is_err());

        // Move the item to an empty slot
        inv.swap(0, 2).unwrap();
        // Item is now at slot 2
        assert_eq!(inv.get(2).unwrap().id(), 0);
        // Slot 0 is empty
        assert_eq!(inv.get(0).unwrap_err(), InvError::EmptySlot(0));
        // Still one item with id 0
        assert_eq!(inv.items_by_id(0).count(), 1);
        // But It's at index 2
        assert_eq!(inv.item_slots_by_id(0).next().unwrap().0, 2);

        // Move empty slot to an occupied slot
        inv.swap(0, 2).unwrap();
        // Item is now at slot 2
        assert_eq!(inv.get(0).unwrap().id(), 0);
        // Slot 0 is empty
        assert_eq!(inv.get(2).unwrap_err(), InvError::EmptySlot(2));
        // Still one item with id 0
        assert_eq!(inv.items_by_id(0).count(), 1);
        // But It's at index 2
        assert_eq!(inv.item_slots_by_id(0).next().unwrap().0, 0);

        // Swap first two occupied slots
        inv.swap(0, 1).unwrap();
        // Item is now at slot 2
        assert_eq!(inv.get(0).unwrap().id(), 1);
        assert_eq!(inv.get(1).unwrap().id(), 0);

        assert_eq!(inv.items_by_id(0).count(), 1);
        assert_eq!(inv.items_by_id(1).count(), 1);

        assert_eq!(inv.item_slots_by_id(0).next().unwrap().0, 1);
        assert_eq!(inv.item_slots_by_id(1).next().unwrap().0, 0);
    }

    #[test]
    fn items_by_id() {
        let mut inv = Inv2::empty();
        assert_eq!(inv.items_by_id(0).count(), 0);
        inv.add(DummyItem::new(0)).unwrap();
        assert_eq!(inv.items_by_id(0).count(), 1);
        inv.add(DummyItem::new(1)).unwrap();
        assert_eq!(inv.items_by_id(1).count(), 1);

        assert_eq!(inv.item_slots_by_id_mut(0).count(), 1);
        assert_eq!(inv.item_slots_by_id_mut(1).count(), 1);

        assert_eq!(inv.remove(0).unwrap().unwrap().id(), (0));
        assert_eq!(inv.items_by_id(0).count(), 0);
        assert_eq!(inv.items_by_id(1).count(), 1);
        assert_eq!(inv.remove(1).unwrap().unwrap().id(), (1));
        assert_eq!(inv.items_by_id(1).count(), 0);
    }
}
