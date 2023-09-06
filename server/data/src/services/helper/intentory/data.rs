use num_enum::TryFromPrimitive;
use proto95::{id::ItemId, shared::inventory::CharEquipSlot};

use crate::services::{
    helper::inv::{self, InvSlotIndex},
    item::model::{EquipItem, StackItem},
};

#[derive(Debug)]
pub struct EquipItemSlot {
    pub item_id: ItemId,
    pub item: Box<EquipItem>,
}

impl From<EquipItem> for EquipItemSlot {
    fn from(value: EquipItem) -> Self {
        Self {
            item_id: value.item_id,
            item: Box::new(value),
        }
    }
}

impl inv::InvItem for EquipItemSlot {
    type Id = ItemId;
    type SlotIndex = usize;
    fn id(&self) -> Self::Id {
        self.item_id
    }
}

#[derive(Debug)]
pub struct EquippedItemSlot(pub EquipItemSlot);

impl From<EquipItem> for EquippedItemSlot {
    fn from(value: EquipItem) -> Self {
        Self(value.into())
    }
}

impl From<EquipItemSlot> for EquippedItemSlot {
    fn from(value: EquipItemSlot) -> Self {
        Self(value)
    }
}


impl InvSlotIndex for CharEquipSlot {
    fn to_slot_index(&self) -> usize {
        *self as usize
    }

    fn from_slot_index(slot: usize) -> Self {
        CharEquipSlot::try_from_primitive(slot as u8).unwrap()
    }
}

impl inv::InvItem for EquippedItemSlot {
    type Id = ItemId;
    type SlotIndex = CharEquipSlot;
    fn id(&self) -> Self::Id {
        self.0.item_id
    }
}

/*
#[derive(Debug)]
pub struct EquippedInventory<const CAP: usize>(inv::Inv<EquipItemSlot, CAP>);

impl<const CAP: usize> EquippedInventory<CAP> {
    pub fn new(slots: usize) -> Self {
        Self(inv::Inv::new(slots))
    }

    pub fn iter(&self) -> impl Iterator<Item = (CharEquipSlot, &EquipItemSlot)> {
        self.0
            .item_with_slots()
            .map(|(slot, item)| (CharEquipSlot::try_from(slot as u8).unwrap(), item))
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut EquipItemSlot> {
        self.0.items_mut()
    }

    pub fn items(&self) -> impl Iterator<Item = &EquipItemSlot> {
        self.0.items()
    }

    pub fn items_with_slots(&self) -> impl Iterator<Item = (CharEquipSlot, &EquipItemSlot)> {
        self.0
            .item_with_slots()
            .map(|(slot, item)| (CharEquipSlot::try_from(slot as u8).unwrap(), item))
    }
}*/

pub type EquippedInventory<const CAP: usize> = inv::Inv<EquippedItemSlot, CAP>;
pub type EquipInventory<const CAP: usize> = inv::Inv<EquipItemSlot, CAP>;

pub type ShroomStackInventory<const CAP: usize> = inv::stack::StackInv<StackItem, CAP>;
