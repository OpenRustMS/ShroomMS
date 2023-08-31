use super::{inv::Inventory, InventoryError, InventoryItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TakeResult<Item> {
    Complete(Item),
    Partial((usize, Item)),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackInsertResult {
    // Merged into the (slot, slot new quantity)
    Merged((usize, usize)),
    // Inserted into a new slot
    NewSlot(usize),
    // Partial inserted in old slot and added a new
    Partial {
        slot: usize,
        slot_quantity: usize,
        new_slot: usize,
        new_slot_quantity: usize,
    },
}

pub trait StackOperationHandler {
    type Item: StackInventoryItem;

    fn on_add_item(&mut self, slot: usize, item: &Self::Item);
    fn on_remove_item(&mut self, slot: usize);
    fn on_update_item(&mut self, slot: usize, q: usize);
    fn on_swap_item(&mut self, src: usize, dst: usize);
}

pub trait StackInventoryItem: InventoryItem + Sized {
    fn quantity(&self) -> usize;
    fn set_quantity(&mut self, quantity: usize);
    fn split(&mut self, quantity: usize) -> Result<Self, InventoryError>;

    fn adjust_quantity(&mut self, delta: isize) -> Result<usize, InventoryError> {
        let new_quantity = self
            .quantity()
            .checked_add_signed(delta)
            .ok_or(InventoryError::Full)?;
        if new_quantity > self.max_quantity() {
            //TODO ERROR
            return Err(InventoryError::Full);
        }

        self.set_quantity(new_quantity);
        Ok(self.quantity())
    }

    fn max_quantity(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.quantity() == 0
    }

    fn free_space(&self) -> usize {
        self.max_quantity() - self.quantity()
    }

    fn is_full(&self) -> bool {
        self.max_quantity() == self.quantity()
    }

    fn is_same_id(&self, other: &Self) -> bool {
        self.id() == other.id()
    }

    fn merge_from_n(&mut self, other: &mut Self, _n: usize) -> Result<usize, InventoryError> {
        if !self.is_same_id(other) {
            return Err(InventoryError::InvalidMergeId {
                left: self.id(),
                right: other.id(),
            });
        }

        let move_count = self.free_space().min(other.quantity());
        other.set_quantity(other.quantity() - move_count);
        self.set_quantity(self.quantity() + move_count);
        Ok(move_count)
    }

    fn merge_from(&mut self, other: &mut Self) -> Result<usize, InventoryError> {
        if !self.is_same_id(other) {
            return Err(InventoryError::InvalidMergeId {
                left: self.id(),
                right: other.id(),
            });
        }

        let move_count = self.free_space().min(other.quantity()) as isize;
        other.adjust_quantity(-move_count).unwrap();
        self.adjust_quantity(move_count).unwrap();
        Ok(move_count as usize)
    }

    fn merge_into(&mut self, other: &mut Self) -> Result<usize, InventoryError> {
        other.merge_from(self)
    }
}

#[derive(Clone, Debug)]
pub struct StackInventory<Item: StackInventoryItem, const CAP: usize>(Inventory<Item, CAP>);

impl<Item: StackInventoryItem, const CAP: usize> StackInventory<Item, CAP> {
    pub fn new(slots: usize) -> Self {
        Self(Inventory::new(slots))
    }

    pub fn swap(&mut self, src: usize, dst: usize) -> Result<(), InventoryError> {
        self.0.swap(src, dst)
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &Item)> {
        self.0.items_with_slots()
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut Item> {
        self.0.items_mut()
    }

    pub fn get(&self, slot: usize) -> Option<&Item> {
        self.0.get(slot).ok()
    }

    pub fn get_mut(&mut self, slot: usize) -> Option<&mut Item> {
        self.0.get_mut(slot).ok()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn set(&mut self, slot: usize, item: Item) {
        self.0.set_slot(slot, item)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn slots(&self) -> usize {
        self.0.slots()
    }

    pub fn try_insert(&mut self, item: Item) -> Result<usize, Item> {
        // TODO use stack merging
        self.0.try_insert(item)
    }
    fn items_with_free_space_mut(
        &mut self,
        id: u32,
    ) -> impl Iterator<Item = (usize, &mut Item)> + '_ {
        let mapping = &mut self.0.slot_mapping;
        let items = &mut self.0.items;

        // We know the iterator never changes
        // the array so we can safely assume to take the offerst
        items
            .enumerate_items_by_key_mut(id)
            .filter(|item| item.1.free_space() > 0)
            .map(move |item| {
                let slot_ix = mapping
                    .iter()
                    .position(|&x| x == Some(item.0 as u8))
                    .unwrap();

                (slot_ix, item.1)
            })
    }

    pub fn merged_insert(
        &mut self,
        mut item: Item,
        handler: &mut impl StackOperationHandler<Item = Item>,
    ) -> Result<(), InventoryError> {
        if !self.0.can_insert_item(&item) {
            //TODO error
            return Err(InventoryError::Full);
        }

        // Assertion is that there's atleast one slot free
        if self.0.is_full() {
            return Err(InventoryError::Full);
        }

        // Try to fill up non-filled slots
        // TODO this needs to be made simpler,
        // move this to the inventory struct
        // so the borrow checker can work better
        for (merge_slot, merge_item) in self.items_with_free_space_mut(item.id()) {
            item.merge_into(merge_item)?;

            if item.quantity() == 0 {
                return Ok(());
            }

            handler.on_update_item(merge_slot, merge_item.quantity());
        }

        // Check if we still have to insert items
        if item.quantity() > 0 {
            // We asserted, that we can insert the item
            let Ok(new_slot) = self.try_insert(item) else {
                return Err(InventoryError::Full)
            };

            handler.on_add_item(new_slot, self.get(new_slot).unwrap());
        }

        Ok(())
    }

    pub fn remove(&mut self, slot: usize) -> Result<Option<Item>, InventoryError> {
        self.0.remove(slot)
    }

    pub fn take(
        &mut self,
        slot: usize,
        quantity: Option<usize>,
        handler: &mut impl StackOperationHandler<Item = Item>
    ) -> Result<Item, InventoryError> {
        let item = self.0.get_mut(slot)?;

        let split = if let Some(q) = quantity {
            item.split(q)?
        } else {
            item.split(item.quantity())?
        };
        let q = item.quantity();
        Ok(if item.quantity() == 0 {
            // Remove the item
            let item = self.0.take(slot).unwrap();
            handler.on_remove_item(slot);
            item
        } else {
            handler.on_update_item(slot, q);
            split
        })
    }

    /// Moves a slot
    /// no quantity means the whole item is moved
    pub fn move_slot(
        &mut self,
        slot_src: usize,
        slot_dst: usize,
        quantity: Option<usize>,
        handler: &mut impl StackOperationHandler<Item = Item>,
    ) -> Result<(), InventoryError> {
        // Verify the src slot exists with that quantity
        let src = self.0.get_mut(slot_src)?;
        let src_id = src.id();

        let quantity = quantity.unwrap_or(src.quantity());
        // Ensure quantity exists in src slot
        if quantity > src.quantity() {
            //TODO
            return Err(InventoryError::EmptySlot(slot_src));
        }

        let complete = quantity == src.quantity();

        // Check if dst exists
        let Some(dst) = self.0.get_mut_slot(slot_dst)? else {
            // Dst is free so check if we remove or keep src
            if complete {
                self.0.swap(slot_src, slot_dst)?;
                handler.on_swap_item(slot_src, slot_dst);
            } else {
                let src = self
                    .0
                    .get_mut(slot_src)?;

                let split = src.split(quantity)?;
                let src_q = src.quantity();
                self.0.set_slot(slot_dst, split);

                handler.on_update_item(slot_src, src_q);
                handler.on_add_item(slot_dst, self.0.get(slot_dst).unwrap());
            }

            return Ok(());
        };

        // Ensure it's the same id
        if src_id != dst.id() {
            return Err(InventoryError::InvalidMergeId {
                left: src_id,
                right: dst.id(),
            });
        }
        let q = quantity as isize;
        let dst_q = dst.adjust_quantity(q)?;

        if complete {
            self.0.remove(slot_src)?;
            handler.on_update_item(slot_dst, dst_q);
            handler.on_remove_item(dst_q);
        } else {
            let src_q = self.0.get_mut(slot_src)?.adjust_quantity(-q).unwrap();
            handler.on_update_item(slot_src, src_q);
            handler.on_update_item(slot_dst, dst_q);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct MockItem(u32, usize);

    impl InventoryItem for MockItem {
        fn is_one_of_a_kind(&self) -> bool {
            false
        }

        fn id(&self) -> u32 {
            self.0
        }
    }

    impl StackInventoryItem for MockItem {
        fn quantity(&self) -> usize {
            self.1
        }

        fn set_quantity(&mut self, quantity: usize) {
            self.1 = quantity
        }

        fn max_quantity(&self) -> usize {
            255
        }

        fn split(&mut self, quantity: usize) -> Result<Self, InventoryError> {
            if quantity > self.quantity() {
                // TODO: check properly
                return Err(InventoryError::Full);
            }

            self.set_quantity(self.quantity() - quantity);
            Ok(Self(self.id(), quantity))
        }
    }

    pub struct DummyHandler;

    impl StackOperationHandler for DummyHandler {
        type Item = MockItem;

        fn on_add_item(&mut self, _slot: usize, _item: &Self::Item) {}

        fn on_remove_item(&mut self, _slot: usize) {}

        fn on_update_item(&mut self, _slot: usize, _q: usize) {}

        fn on_swap_item(&mut self, _src: usize, _dst: usize) {}
    }

    #[test]
    fn stack_move_free_slot() {
        let mut inv = StackInventory::<MockItem, 10>::new(10);
        inv.set(0, MockItem(1, 10));

        // Complete
        inv
            .move_slot(0, 1, Some(10), &mut DummyHandler)
            .expect("Move");
        assert!(inv.get(0).is_none());
        assert_eq!(inv.get(1).unwrap().quantity(), 10);

        // Move Partial back
        inv
            .move_slot(1, 0, Some(3), &mut DummyHandler)
            .expect("Move");
        assert_eq!(inv.get(0).unwrap().quantity(), 3);
        assert_eq!(inv.get(1).unwrap().quantity(), 7);
    }

    #[test]
    fn stack_move_complete() {
        let mut inv = StackInventory::<MockItem, 10>::new(10);
        inv.set(0, MockItem(1, 9));
        inv.set(1, MockItem(1, 1));

        // Complete
        inv
            .move_slot(0, 1, Some(9), &mut DummyHandler)
            .expect("Move");
        assert!(inv.get(0).is_none());
        assert_eq!(inv.get(1).unwrap().quantity(), 10);
    }

    #[test]
    fn stack_move_partial() {
        let mut inv = StackInventory::<MockItem, 10>::new(10);
        inv.set(0, MockItem(1, 5));
        inv.set(1, MockItem(1, 5));

        // Complete
        inv
            .move_slot(0, 1, Some(4), &mut DummyHandler)
            .expect("Move");
        assert_eq!(inv.get(0).unwrap().quantity(), 1);
        assert_eq!(inv.get(1).unwrap().quantity(), 9);

        inv
            .move_slot(1, 0, Some(8), &mut DummyHandler)
            .expect("Move");
        assert_eq!(inv.get(0).unwrap().quantity(), 9);
        assert_eq!(inv.get(1).unwrap().quantity(), 1);
    }

    #[test]
    fn simple_merge() {
        let mut l = MockItem(1, 10);
        let mut r = MockItem(1, 10);

        let merged = l.merge_from(&mut r).expect("Merge");
        assert_eq!(merged, 10);
        assert_eq!(l.quantity(), 20);
        assert_eq!(r.quantity(), 0);
    }

    #[test]
    fn simple_merge_rev() {
        let mut l = MockItem(1, 10);
        let mut r = MockItem(1, 5);

        let merged = r.merge_into(&mut l).expect("Merge");
        assert_eq!(merged, 5);
        assert_eq!(l.quantity(), 15);
        assert_eq!(r.quantity(), 0);
    }

    #[test]
    fn capped_merge() {
        let mut l = MockItem(1, 10);
        let mut almost_full = MockItem(1, 255 - 10);
        let merged = almost_full.merge_from(&mut l).expect("Merge");
        assert_eq!(merged, 10);

        assert_eq!(almost_full.quantity(), 255);
        assert_eq!(l.quantity(), 0);
    }

    #[test]
    fn empty_merge() {
        let mut l = MockItem(1, 10);
        let mut r = MockItem(1, 0);

        let merged = l.merge_from(&mut r).expect("Merge");
        assert_eq!(merged, 0);
        assert_eq!(l.quantity(), 10);
        assert_eq!(r.quantity(), 0);
    }
}
