use num_enum::TryFromPrimitive;
use proto95::{id::ItemId, shared::inventory::CharEquipSlot};

use crate::services::item::model::{EquipItem, StackItem};

use super::{
    inv::{Inventory, InventoryExt},
    slot::InventorySlotIndex,
    stack::{StackInventory, StackInventoryItem},
    InventoryItem,
};

impl InventorySlotIndex for CharEquipSlot {
    fn from_index(ix: usize) -> Self {
        CharEquipSlot::try_from_primitive(ix as u8).unwrap()
    }

    fn to_index(&self) -> usize {
        *self as usize
    }
}

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

impl InventoryItem for EquipItemSlot {
    fn is_one_of_a_kind(&self) -> bool {
        false
    }

    fn id(&self) -> u32 {
        self.item_id.0
    }
}

#[derive(Debug)]
pub struct EquippedInventory<const CAP: usize>(Inventory<EquipItemSlot, CAP>);

impl<const CAP: usize> InventoryExt<CAP> for EquippedInventory<CAP> {
    type Slot = CharEquipSlot;

    type Item = EquipItemSlot;

    fn get_inner(&self) -> &Inventory<Self::Item, CAP> {
        &self.0
    }

    fn get_inner_mut(&mut self) -> &mut Inventory<Self::Item, CAP> {
        &mut self.0
    }
}

impl<const CAP: usize> EquippedInventory<CAP> {
    pub fn new(slots: usize) -> Self {
        Self(Inventory::new(slots))
    }

    pub fn iter(&self) -> impl Iterator<Item = (CharEquipSlot, &EquipItemSlot)> {
        self.0
            .items_with_slots()
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
            .items_with_slots()
            .map(|(slot, item)| (CharEquipSlot::try_from(slot as u8).unwrap(), item))
    }
}

pub type EquipInventory<const CAP: usize> = Inventory<EquipItemSlot, CAP>;

impl InventoryItem for StackItem {
    fn is_one_of_a_kind(&self) -> bool {
        false
    }

    fn id(&self) -> u32 {
        self.info.item_id.0
    }
}

impl StackInventoryItem for StackItem {
    fn quantity(&self) -> usize {
        self.quantity as usize
    }

    fn set_quantity(&mut self, quantity: usize) {
        self.quantity = quantity as u16;
        self.last_update += 1;
    }

    fn max_quantity(&self) -> usize {
        200
    }

    fn split(&mut self, quantity: usize) -> Result<Self, super::InventoryError> {
        self.adjust_quantity(-(quantity as isize))?;
        //TODO(!!!) this needs some work for checks
        Ok(StackItem::from_item_id(self.item_id, quantity as u16))
    }
}

pub type ShroomStackInventory<const CAP: usize> = StackInventory<StackItem, CAP>;
