use std::ops::{Deref, DerefMut};

use chrono::NaiveDateTime;

use proto95::{
    id::ItemId,
    shared::item::{self as proto_item},
};
use rand::{thread_rng, Rng};
use shroom_net::packet::proto::time::{ShroomExpirationTime, ShroomTime};

use crate::{
    entities::{equip_item, item_stack},
    proto_mapper::db_to_shroom_time,
    services::meta::meta_service::ItemMeta,
};

use super::stats::{EquipStats, StatsExt};

#[derive(Debug, Clone)]
pub struct ItemLevelInfo {
    pub level: u8,
    pub exp: u32,
}

#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub db_id: Option<i32>,
    pub item_id: ItemId,
    pub cash_id: Option<u64>,
    pub expiration: Option<NaiveDateTime>,
    pub owner: Option<String>,
    pub flags: proto_item::ItemFlags,
    pub last_update: u32,
}

impl ItemInfo {
    pub fn from_id(item_id: ItemId) -> Self {
        Self {
            db_id: None,
            item_id,
            cash_id: None,
            expiration: None,
            owner: None,
            flags: proto_item::ItemFlags::empty(),
            last_update: 0,
        }
    }

    pub fn is_expired(&self, now: NaiveDateTime) -> bool {
        match self.expiration {
            Some(t_exp) => t_exp <= now,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct EquipItem {
    pub info: ItemInfo,
    pub stats: EquipStats,
    pub upgrades: u8,
    pub slots: u8,
    pub hammers_used: u8,
    pub level_info: Option<ItemLevelInfo>,
}

impl From<equip_item::Model> for EquipItem {
    fn from(value: equip_item::Model) -> Self {
        let stats = EquipStats::from_db_stats(&value);
        let owner = if value.owner_tag.is_empty() {
            None
        } else {
            Some(value.owner_tag)
        };
        Self {
            info: ItemInfo {
                db_id: Some(value.id),
                item_id: ItemId(value.item_id as u32),
                cash_id: value.cash_id.map(|i| i as u64),
                expiration: value.expires_at,
                owner,
                flags: proto_item::ItemFlags::from_bits(value.flags as u16).unwrap(),
                last_update: 0,
            },
            hammers_used: value.vicious_hammers as u8,
            level_info: Some(ItemLevelInfo {
                level: value.item_level as u8, //TODO
                exp: value.item_exp as u32,
            }),
            slots: value.upgrade_slots as u8,
            upgrades: 0,
            stats,
        }
    }
}

impl Deref for EquipItem {
    type Target = ItemInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl DerefMut for EquipItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}

impl EquipItem {
    pub fn from_item_id(item_id: ItemId, meta: ItemMeta) -> Self {
        let mut rng = rand::thread_rng();
        let mut stats = EquipStats::from_equip_meta(meta);
        stats.apply_chaos_scroll(&mut rng, -2..=2);
        Self {
            info: ItemInfo::from_id(item_id),
            stats,
            slots: meta.slot_max as u8,
            hammers_used: 0,
            level_info: None,
            upgrades: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackItem {
    pub info: ItemInfo,
    pub quantity: u16,
}

impl Deref for StackItem {
    type Target = ItemInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl DerefMut for StackItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}

impl From<item_stack::Model> for StackItem {
    fn from(value: item_stack::Model) -> Self {
        Self {
            info: ItemInfo {
                db_id: Some(value.id),
                item_id: ItemId(value.item_id as u32),
                cash_id: value.cash_id.map(|i| i as u64),
                expiration: value.expires_at,
                owner: None,
                flags: proto_item::ItemFlags::from_bits(value.flags as u16).unwrap(), //TODO ::from(value.flags as u16),
                last_update: 0,
            },
            quantity: value.quantity as u16,
        }
    }
}

impl StackItem {
    pub fn from_item_id(item_id: ItemId, quantity: u16) -> Self {
        Self {
            info: ItemInfo::from_id(item_id),
            quantity,
        }
    }
}

impl From<&EquipItem> for proto_item::EquipItemInfo {
    fn from(value: &EquipItem) -> Self {
        proto_item::EquipItemInfo {
            info: proto_item::ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id.into(),
                expiration: value.expiration.map(db_to_shroom_time).into(),
            },
            stats: proto_item::EquipAllStats {
                remaining_upgrade_slots: value.slots,
                upgrade_count: value.upgrades,
                stats: value.stats.as_game_stats(),
                title: value.owner.clone().unwrap_or_default(),
                flags: value.flags,
            },
            equipped_at: ShroomTime::now(),
            lvl_up_ty: 0,
            lvl: value.level_info.as_ref().map(|l| l.level).unwrap_or(0),
            exp: value.level_info.as_ref().map(|l| l.exp).unwrap_or(0),
            durability: -1,
            hammer_count: value.hammers_used as u32,
            grade: 0,
            stars: 3,
            options: [0; 3],
            sockets: [0; 2],
            // TODO handle cashid/sn better
            sn: Some(value.db_id.unwrap_or(thread_rng().gen()) as u64).into(),
            prev_bonus_exp_rate: -1,
        }
    }
}

impl From<&StackItem> for proto_item::ItemStackData {
    fn from(value: &StackItem) -> Self {
        proto_item::ItemStackData {
            info: proto_item::ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id.into(),
                expiration: ShroomExpirationTime::never(),
            },
            quantity: value.quantity,
            title: value.owner.clone().unwrap_or(String::new()),
            flag: value.flags,
            sn: None.into(),
        }
    }
}
