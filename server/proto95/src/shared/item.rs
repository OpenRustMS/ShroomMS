use shroom_pkt::{
    mark_shroom_bitflags, CondOption, ShroomExpirationTime, ShroomOption8, ShroomPacket,
    ShroomPacketEnum, ShroomTime,
};

use crate::id::ItemId;

use super::NameStr;

#[derive(Debug, ShroomPacket)]
pub struct ItemInfo {
    pub item_id: ItemId,
    pub cash_id: ShroomOption8<u64>,
    pub expiration: ShroomExpirationTime,
}

impl ItemInfo {
    pub fn is_rechargable(&self) -> bool {
        self.item_id.is_rechargable()
    }

    pub fn has_sn(&self) -> bool {
        self.cash_id.is_some()
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemFlags : u16 {
        const Protected = 0x01;
        const PreventSlipping = 0x02;
        const PreventColdness = 0x04;
        const Untradeable = 0x08;
        const ScissorsApplied = 0x10;
        const Sandbox = 0x40;
        const PetCome = 0x80;
        const AccountSharing = 0x100;
        const MergeUntradeable = 0x200;
    }
}
mark_shroom_bitflags!(ItemFlags);

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemBundleFlags : u16 {
        const Protected = 0x01;
        const TradingPossible = 0x02;
    }
}
mark_shroom_bitflags!(ItemBundleFlags);

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemPetFlags : u16 {
        const Protected = 0x01;
        const TradingPossible = 0x02;
    }
}
mark_shroom_bitflags!(ItemPetFlags);

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemEquipFlags : u16 {
        const Protected = 0x01;
        const PreventSlipping = 0x02;
        const SupportWarm = 0x04;
        const Binded = 0x08;
        //const TradingPossible = 0x1;
    }
}
mark_shroom_bitflags!(ItemEquipFlags);

#[derive(Debug, ShroomPacket)]
pub struct PetItemInfo {
    pub pet_name: NameStr,
    pub level: u8,
    pub tameness: u16,
    pub fullness: u8,                     /* repleteness */
    pub expiration: ShroomExpirationTime, /* dateDead */
    pub pet_attr: u16,                    /* PetAttribute  pet is only loaded when attr == 1*/
    pub skill: u16,
    pub remain_life: u32,
    pub attr: ItemPetFlags,
}
#[derive(Debug, ShroomPacket)]
pub struct EquipStats {
    pub str: u16,
    pub dex: u16,
    pub int: u16,
    pub luk: u16,
    pub hp: u16,
    pub mp: u16,
    pub watk: u16,
    pub matk: u16,
    pub wdef: u16,
    pub mdef: u16,
    pub accuracy: u16,
    pub avoid: u16,
    pub craft: u16,
    pub speed: u16,
    pub jump: u16,
}

#[derive(Debug, ShroomPacket)]
pub struct EquipAllStats {
    pub remaining_upgrade_slots: u8,
    pub upgrade_count: u8,
    pub stats: EquipStats,
    pub title: String, /* stitle */
    pub flags: ItemFlags,
}

#[derive(Debug, ShroomPacket)]
pub struct ItemPetData {
    pub info: ItemInfo,
    pub name: NameStr,
    pub level: u8,
    pub tameness: u16,
    pub fullness: u8,
    pub expiration: ShroomExpirationTime,
    pub attribute1: u16,
    pub skill: u16,
    pub remain_life: u32,
    pub attribute2: u16,
}

#[derive(Debug, ShroomPacket)]
pub struct ItemStackData {
    pub info: ItemInfo,
    pub quantity: u16, /* nNumber */
    pub title: String,
    pub flag: ItemFlags,
    #[pkt(check(field = "info", cond = "ItemInfo::is_rechargable"))]
    pub sn: CondOption<u64>,
}

#[derive(Debug, ShroomPacket)]
pub struct EquipItemInfo {
    pub info: ItemInfo,
    pub stats: EquipAllStats,
    pub lvl_up_ty: u8,
    pub lvl: u8,
    pub exp: u32,
    pub durability: i32,
    pub hammer_count: u32,
    pub grade: u8,
    pub stars: u8,
    pub options: [u16; 3],
    pub sockets: [u16; 2],
    #[pkt(check(field = "info", cond = "ItemInfo::has_sn"))]
    pub sn: CondOption<u64>,
    pub equipped_at: ShroomTime,
    pub prev_bonus_exp_rate: i32,
}

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum Item {
    Equip(EquipItemInfo) = 1,
    Stack(ItemStackData) = 2,
    Pet(ItemPetData) = 3,
    Equipped(()) = 255,
}
