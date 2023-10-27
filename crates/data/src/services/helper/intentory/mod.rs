pub mod data;
pub mod inv;
pub mod slot;
pub mod sorted_array_vec;
pub mod stack;

use std::fmt::Debug;

use thiserror::Error;

use self::sorted_array_vec::SortedItemKey;

#[derive(Error, Debug)]
pub enum InventoryError {
    #[error("Empty slot {0}")]
    EmptySlot(usize),
    #[error("Remove quantity({remove_quantity}) higher than quantity({quantity}) for slot {slot}")]
    RemoveTooMuch {
        remove_quantity: usize,
        quantity: usize,
        slot: usize,
    },
    #[error("Slot is out of range: {0}")]
    OutOfRange(usize),
    #[error("Inventory is full")]
    Full,
    #[error("Item {0} is one-of-a-kind and already exists in the inventory")]
    OneOfAKindConflict(u32),
    #[error("Item stacks left({left}) and right({right}) are not merge-able")]
    InvalidMergeId { left: u32, right: u32 },
}

type ItemId = u32;

pub trait InventoryItem {
    fn is_one_of_a_kind(&self) -> bool;
    fn id(&self) -> ItemId;
}

impl<T: InventoryItem> SortedItemKey for T {
    type K = ItemId;
    fn key(&self) -> ItemId {
        self.id()
    }
}

const MAX_CAP: usize = u8::MAX as usize;
