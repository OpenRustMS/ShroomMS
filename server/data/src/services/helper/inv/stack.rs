use super::{Inv, InvError, InvItem, InvResult};

pub trait StackInvEventHandler {
    type Item: StackItem;

    fn on_add(&mut self, item: &Self::Item, slot: usize);
    fn on_remove(&mut self, item: &Self::Item, slot: usize);
    fn on_quantity_change(&mut self, item: &Self::Item, slot: usize);
    fn on_swap(&mut self, slot_a: usize, slot_b: usize);
}

pub trait StackItem: InvItem + Sized {
    fn max_stack_size(&self) -> usize;

    fn get_quantity(&self) -> usize;
    fn set_quantity(&mut self, count: usize);

    fn split(&mut self, split_quantity: usize) -> InvResult<Self>;

    fn free_space(&self) -> usize {
        self.max_stack_size() - self.get_quantity()
    }

    fn sub_quantity(&mut self, delta: usize) -> InvResult<()> {
        self.set_quantity(
            self.get_quantity()
                .checked_sub(delta)
                .ok_or(InvError::SlotInsufficentSpace)?,
        );

        Ok(())
    }

    fn add_quantity(&mut self, delta: usize) -> InvResult<()> {
        if self.free_space() > delta {
            return Err(InvError::SlotFull);
        }

        self.set_quantity(self.get_quantity() + delta);

        Ok(())
    }

    fn merge_into(&mut self, other: &mut Self) -> usize {
        let free_space = other.free_space();
        let delta = free_space.min(self.get_quantity());
        other.add_quantity(delta).unwrap();
        self.sub_quantity(delta).unwrap();
        delta
    }
}

pub struct StackInv<Item: StackItem, const CAP: usize>(Inv<Item, CAP>);

impl<Item: StackItem, const CAP: usize> StackInv<Item, CAP> {
    fn clear_empty_slots(&mut self) {
        for slot in 0..self.0.len() {
            if let Ok(item) = self.0.get(slot) {
                if item.get_quantity() == 0 {
                    self.0.remove(slot).unwrap();
                }
            }
        }
    }

    pub fn empty() -> Self {
        Self(Inv::empty())
    }

    pub fn item_quantity_by_id(&self, id: Item::Id) -> usize {
        self.0.items_by_id(id).map(|i| i.get_quantity()).sum()
    }

    pub fn add(
        &mut self,
        mut stack: Item,
        mut handler: impl StackInvEventHandler<Item = Item>,
    ) -> InvResult<()> {
        // If the inventory is full we can't
        // add a new item after distribution
        if self.0.is_full() {
            return Err(InvError::Full);
        }

        // Filter empty item stacks
        if stack.get_quantity() == 0 {
            return Ok(());
        }

        // Attempt to distribute the stack into existing stacks
        for (slot, item) in self.0.item_slots_by_id_mut(stack.id()) {
            let free_space = item.free_space();
            if free_space > 0 {
                stack.merge_into(item);
                handler.on_quantity_change(item, slot);

                // If we merged all items into the inventory
                // we are done
                if stack.get_quantity() == 0 {
                    return Ok(());
                }
            }
        }

        // ... else-wise we need to add the rest
        let slot = self.0.add(stack)?;
        handler.on_add(self.0.get(slot).unwrap(), slot);
        Ok(())
    }

    /// Attempts to take `n` items from the given slot
    pub fn take_from_slot(&mut self, slot: usize, n: usize) -> InvResult<Item> {
        let item = self.0.get_mut(slot)?;
        let result = item.split(n)?;
        if item.get_quantity() == 0 {
            self.0.remove(slot)?;
        }
        Ok(result)
    }

    /// Attempts to take `n` items from the inventory
    /// of the given item id
    pub fn take_items(&mut self, id: Item::Id, mut n: usize) -> InvResult<()> {
        let q = self.item_quantity_by_id(id);
        // Check if we have items
        if q < n {
            return Err(InvError::InsufficentItems(n - q));
        }

        for item in self.0.items_by_id_mut(id) {
            let delta = item.get_quantity().min(n);
            n -= delta;
            item.sub_quantity(delta).unwrap();

            if n == 0 {
                break;
            }
        }

        self.clear_empty_slots();

        // Now we take n items out of the inventory

        todo!()
    }

    pub fn take(&mut self, slot: usize) -> InvResult<Item> {
        self.0.take(slot)
    }
}
