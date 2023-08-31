use std::cmp::Ordering;

use proto95::{
    id::item_id::InventoryType,
    shared::{
        inventory::{CharEquipSlot, InventoryOperation},
        item::Item,
    },
};

use crate::services::{
    data::character::ItemStarterSet,
    helper::intentory::{
        data::{EquipInventory, EquipItemSlot, EquippedInventory, ShroomStackInventory},
        inv::InventoryExt,
        stack::StackOperationHandler,
    },
    item::{
        model::{EquipItem, StackItem},
        stats::{EquipStats, StatsExt},
    },
};

pub const EQUIPPED_CAP: usize = 96;
pub const INV_ITEM_CAP: usize = 180;

#[derive(Debug, Copy, Clone)]
pub enum InventorySlot {
    Slot(u16),
    EquippedSlot(CharEquipSlot),
}

impl InventorySlot {
    pub fn as_slot_index(&self) -> u16 {
        match self {
            InventorySlot::Slot(slot) => *slot + 1,
            InventorySlot::EquippedSlot(slot) => (-(*slot as i16)) as u16,
        }
    }

    pub fn as_slot(&self) -> usize {
        match self {
            InventorySlot::Slot(v) => *v as usize,
            InventorySlot::EquippedSlot(v) => *v as usize,
        }
    }
}

impl TryFrom<i16> for InventorySlot {
    type Error = anyhow::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(match value.cmp(&0) {
            Ordering::Greater => Self::Slot((value - 1) as u16),
            _ => Self::EquippedSlot(CharEquipSlot::try_from((-value) as u8)?),
        })
    }
}

#[derive(Debug)]
pub struct PendingOperations {
    pub ops: Vec<InventoryOperation>,
    inv_type: InventoryType,
}

impl StackOperationHandler for PendingOperations {
    type Item = StackItem;

    fn on_add_item(&mut self, slot: usize, item: &Self::Item) {
        self.ops.push(InventoryOperation::add(
            self.inv_type,
            slot as u16 + 1,
            Item::Stack(item.into()),
        ));
    }

    fn on_remove_item(&mut self, slot: usize) {
        self.ops
            .push(InventoryOperation::remove(self.inv_type, slot as u16 + 1));
    }

    fn on_update_item(&mut self, slot: usize, q: usize) {
        self.ops.push(InventoryOperation::update_quantity(
            self.inv_type,
            slot as u16 + 1,
            q as u16,
        ));
    }

    fn on_swap_item(&mut self, src: usize, dst: usize) {
        self.ops.push(InventoryOperation::mov(
            self.inv_type,
            src as u16 + 1,
            dst as u16 + 1,
        ));
    }
}

#[derive(Debug)]
pub struct InventorySet {
    pub equipped: EquippedInventory<EQUIPPED_CAP>,
    pub masked_equipped: EquippedInventory<EQUIPPED_CAP>,
    pub equip: EquipInventory<INV_ITEM_CAP>,
    pub use_: ShroomStackInventory<INV_ITEM_CAP>,
    pub misc: ShroomStackInventory<INV_ITEM_CAP>,
    pub etc: ShroomStackInventory<INV_ITEM_CAP>,
    pub cash: ShroomStackInventory<INV_ITEM_CAP>,
    pub pending_operations: PendingOperations,
}

impl InventorySet {
    pub fn with_default_slots() -> Self {
        const DEFAULT_SLOTS: usize = 48;
        Self {
            equipped: EquippedInventory::new(EQUIPPED_CAP),
            masked_equipped: EquippedInventory::new(EQUIPPED_CAP),
            equip: EquipInventory::new(DEFAULT_SLOTS),
            use_: ShroomStackInventory::new(DEFAULT_SLOTS),
            misc: ShroomStackInventory::new(DEFAULT_SLOTS),
            etc: ShroomStackInventory::new(DEFAULT_SLOTS),
            cash: ShroomStackInventory::new(DEFAULT_SLOTS),
            pending_operations: PendingOperations {
                ops: Vec::new(),
                inv_type: InventoryType::Etc,
            },
        }
    }

    pub fn fill_with_starter_set(&mut self, _starter_set: ItemStarterSet) {
        todo!()
    }

    pub fn get_stack_inventory_mut(
        &mut self,
        ty: InventoryType,
    ) -> anyhow::Result<&mut ShroomStackInventory<INV_ITEM_CAP>> {
        Ok(match ty {
            InventoryType::Cash => &mut self.cash,
            InventoryType::Consume => &mut self.use_,
            InventoryType::Install => &mut self.misc,
            InventoryType::Etc => &mut self.etc,
            _ => anyhow::bail!("Invalid stack inventory"),
        })
    }

    pub fn get_stack_inventory(
        &self,
        ty: InventoryType,
    ) -> anyhow::Result<&ShroomStackInventory<INV_ITEM_CAP>> {
        Ok(match ty {
            InventoryType::Cash => &self.cash,
            InventoryType::Consume => &self.use_,
            InventoryType::Install => &self.misc,
            InventoryType::Etc => &self.etc,
            _ => anyhow::bail!("Invalid stack inventory"),
        })
    }

    pub fn get_equipped_inventory_mut(
        &mut self,
        ty: InventoryType,
    ) -> anyhow::Result<&mut EquippedInventory<EQUIPPED_CAP>> {
        Ok(match ty {
            InventoryType::Equipped => &mut self.equipped,
            InventoryType::Equip => &mut self.equipped,
            _ => anyhow::bail!("Invalid equipped inventory"),
        })
    }

    pub fn get_equipped_inventory(
        &self,
        ty: InventoryType,
    ) -> anyhow::Result<&EquippedInventory<EQUIPPED_CAP>> {
        Ok(match ty {
            InventoryType::Equipped => &self.equipped,
            InventoryType::Equip => &self.equipped,
            _ => anyhow::bail!("Invalid equipped inventory"),
        })
    }

    pub fn slots(&self, ty: InventoryType) -> usize {
        if ty.is_stack() {
            self.get_stack_inventory(ty).unwrap().slots()
        } else {
            self.get_equipped_inventory(ty).unwrap().slots()
        }
    }
}

#[derive(Debug)]
pub struct CharInventory {
    pub invs: InventorySet,
    pub pending_operations: PendingOperations,
    pub recalc_eq_stats: bool,
}

impl CharInventory {
    pub fn from_inv_set(invs: InventorySet) -> Self {
        Self {
            invs,
            pending_operations: PendingOperations {
                ops: Vec::new(),
                inv_type: InventoryType::Etc,
            },
            recalc_eq_stats: false,
        }
    }

    pub fn get_equipped_stats(&self) -> EquipStats {
        EquipStats::sum(
            self.invs
                .equipped
                .items()
                .map(|item| item.item.stats.clone())
        )
    }

    pub fn slots(&self, ty: InventoryType) -> usize {
        self.invs.slots(ty)
    }

    pub fn try_add_equip(&mut self, item: EquipItem) -> anyhow::Result<usize> {
        let slot = self
            .invs
            .equip
            .try_insert(item.into())
            .map_err(|_| anyhow::format_err!("Insert failed"))?;

        let item = self.invs.equip.get(slot)?.item.as_ref().into();
        log::info!("Equip Item inserted in slot({slot}: {item:?}");

        self.pending_operations.ops.push(InventoryOperation::add(
            InventoryType::Equip,
            slot as u16 + 1,
            Item::Equip(item),
        ));

        Ok(slot)
    }

    pub fn try_add_stack_item(
        &mut self,
        item: StackItem,
        inv_type: InventoryType,
    ) -> anyhow::Result<usize> {
        let inv = self.invs.get_stack_inventory_mut(inv_type)?;
        let slot = inv
            .try_insert(item)
            .map_err(|_| anyhow::format_err!("Insert failed"))?;

        let item = inv.get(slot).expect("Item").into();
        log::info!("Stack Item inserted in slot({slot}: {item:?}");
        self.pending_operations.on_add_item(slot, item);

        Ok(slot)
    }

    pub fn equip_item(
        &mut self,
        eq_slot: usize,
        char_equip_slot: CharEquipSlot,
    ) -> anyhow::Result<()> {
        let equip = &mut self.invs.equip;
        let equipped = &mut self.invs.equipped;

        // Take the item from the equip
        let eq_item: EquipItemSlot = equip.take(eq_slot)?;

        // Put the item into the equipped slot
        let prev_item = equipped.replace(char_equip_slot, eq_item).expect("equip");

        // Put unequipped item back into the equip
        if let Some(item) = prev_item {
            equip.set_slot(eq_slot, item);
        };

        let dst = -(char_equip_slot as i16);
        // Add pending operation
        self.pending_operations.ops.push(InventoryOperation::mov(
            InventoryType::Equip,
            eq_slot as u16 + 1,
            dst as u16,
        ));

        self.recalc_eq_stats = true;

        Ok(())
    }

    pub fn unequip_item(
        &mut self,
        char_equip_slot: CharEquipSlot,
        eq_slot: Option<usize>,
    ) -> anyhow::Result<()> {
        let equip = &mut self.invs.equip;
        let equipped = &mut self.invs.equipped;

        // Either use the destination slot or create a free slot
        let eq_slot = eq_slot
            .or_else(|| equip.find_free_slot())
            .ok_or_else(|| anyhow::format_err!("No free slot"))?;

        // Ensure the eq slot is free
        if equip.get_slot(eq_slot)?.is_some() {
            anyhow::bail!("Slot is not free");
        }

        // Remove the equipped item
        let eq_item: EquipItemSlot = equipped
            .remove(char_equip_slot)
            .ok_or_else(|| anyhow::format_err!("Empty slot"))?;

        // Put the item into the free equip slot
        equip.set_slot(eq_slot, eq_item);

        let src = -(char_equip_slot as i16);
        // Add pending operation
        self.pending_operations.ops.push(InventoryOperation::mov(
            InventoryType::Equip,
            src as u16,
            eq_slot as u16 + 1,
        ));

        self.recalc_eq_stats = true;

        Ok(())
    }

    pub fn drop_stack_item(
        &mut self,
        inv_type: InventoryType,
        slot: InventorySlot,
        quantity: Option<usize>,
    ) -> anyhow::Result<StackItem> {
        let inv = self.invs.get_stack_inventory_mut(inv_type)?;
        let item = inv.take(slot.as_slot(), quantity, &mut self.pending_operations)?;
        Ok(item)
    }

    pub fn drop_equip_item(&mut self, slot: InventorySlot) -> anyhow::Result<EquipItemSlot> {
        Ok(match slot {
            InventorySlot::Slot(_) => {
                let item = self.invs.equip.take(slot.as_slot())?;
                self.pending_operations.ops.push(InventoryOperation::remove(
                    InventoryType::Equip,
                    slot.as_slot_index(),
                ));
                self.recalc_eq_stats = true;
                item
            }
            InventorySlot::EquippedSlot(eq_slot) => {
                let item = self
                    .invs
                    .equipped
                    .remove(eq_slot)
                    .ok_or_else(|| anyhow::format_err!("Invalid eq"))?;
                self.pending_operations.ops.push(InventoryOperation::remove(
                    InventoryType::Equip,
                    slot.as_slot_index(),
                ));
                self.recalc_eq_stats = true;
                item
            }
        })
    }

    pub fn move_item(
        &mut self,
        inv_type: InventoryType,
        src: InventorySlot,
        dst: InventorySlot,
        count: Option<usize>,
    ) -> anyhow::Result<()> {
        if inv_type.is_stack() {
            self.pending_operations.inv_type = inv_type;
            let inv = self.invs.get_stack_inventory_mut(inv_type)?;
            inv.move_slot(
                src.as_slot(),
                dst.as_slot(),
                count,
                &mut self.pending_operations,
            )?;
        } else {
            if inv_type != InventoryType::Equip {
                anyhow::bail!("Not equip");
            }
            match (src, dst) {
                // Unequip
                (InventorySlot::EquippedSlot(equip), InventorySlot::Slot(slot)) => {
                    self.unequip_item(equip, Some(slot as usize))?;
                }
                // Special case without pre-selected equip slot
                (
                    InventorySlot::EquippedSlot(equip),
                    InventorySlot::EquippedSlot(CharEquipSlot::Hat),
                ) => {
                    self.unequip_item(equip, None)?;
                }
                (InventorySlot::Slot(slot), InventorySlot::EquippedSlot(equip)) => {
                    self.equip_item(slot as usize, equip)?;
                }
                (InventorySlot::EquippedSlot(src_), InventorySlot::EquippedSlot(dst_)) => {
                    if !src_.can_swap(&dst_) {
                        anyhow::bail!("Unable to swap");
                    }

                    self.invs.equipped.swap(src_, dst_);
                    self.pending_operations.ops.push(InventoryOperation::mov(
                        inv_type,
                        src.as_slot_index(),
                        dst.as_slot_index(),
                    ));
                }
                (InventorySlot::Slot(_), InventorySlot::Slot(_)) => {
                    self.invs.equip.swap(src.as_slot(), dst.as_slot())?;
                    self.pending_operations.ops.push(InventoryOperation::mov(
                        inv_type,
                        src.as_slot_index(),
                        dst.as_slot_index(),
                    ));
                }
            }
        }

        Ok(())
    }
}
