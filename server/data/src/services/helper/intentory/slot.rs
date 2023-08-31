pub trait InventorySlotIndex {
    fn from_index(ix: usize) -> Self;
    fn to_index(&self) -> usize;
}

impl InventorySlotIndex for usize {
    fn from_index(ix: usize) -> Self {
        ix
    }

    fn to_index(&self) -> usize {
        *self
    }
}