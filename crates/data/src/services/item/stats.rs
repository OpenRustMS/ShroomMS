use std::ops::RangeInclusive;

use array_init::array_init;
use derive_more::{From, Into};
use enum_map::{Enum, EnumMap};
use rand::{distributions::uniform::SampleRange, Rng};

use meta::ItemMeta;

use crate::entities::equip_item;

#[derive(Debug, From, Into, PartialEq, Clone, Copy, Default)]
pub struct ItemStat(pub u16);

impl ItemStat {
    pub fn rnd_stat(mut rng: impl Rng, stat: u16) -> Self {
        if stat == 0 {
            return Self(0);
        }

        Self(rng.gen_range(stat.wrapping_sub(2)..=stat).max(1))
    }

    pub fn apply_chaos(&self, mut rng: impl Rng, range: impl SampleRange<i16>) -> Self {
        if self.0 == 0 {
            return Self(0);
        }

        let stat_diff = rng.gen_range(range);
        Self(self.0.saturating_add_signed(stat_diff))
    }
}

#[derive(Debug, Enum, Clone)]
pub enum EquipStat {
    Str,
    Dex,
    Int,
    Luk,
    Hp,
    Mp,
    WeaponAtk,
    MagicAtk,
    WeaponDef,
    MagicDef,
    Accuracy,
    Avoid,
    Craft,
    Speed,
    Jump,
}

pub type EquipStats = EnumMap<EquipStat, ItemStat>;

pub trait StatsExt {
    fn from_db_stats(v: &equip_item::Model) -> EquipStats;
    fn from_equip_meta(meta: ItemMeta) -> EquipStats;
    fn as_game_stats(&self) -> proto95::shared::item::EquipStats;
    fn from_game_stats(v: proto95::shared::item::EquipStats) -> EquipStats;

    fn apply_chaos_scroll(&mut self, rng: impl rand::Rng, range: RangeInclusive<i16>);
    fn add(&self, other: &Self) -> Self;
    fn sum(stats: impl Iterator<Item = Self>) -> Self;
}

impl StatsExt for EquipStats {
    fn add(&self, other: &Self) -> Self {
        let a = self.as_array();
        let b = other.as_array();

        Self::from_array(array_init(|i| ItemStat(a[i].0.saturating_add(b[i].0))))
    }

    fn sum(stats: impl Iterator<Item = Self>) -> Self {
        stats.fold(Self::default(), |acc, next| acc.add(&next))
    }

    fn from_db_stats(v: &equip_item::Model) -> EquipStats {
        enum_map::enum_map! {
            EquipStat::Str => ItemStat(v.str as u16),
            EquipStat::Dex => ItemStat(v.dex as u16),
            EquipStat::Int => ItemStat(v.int as u16),
            EquipStat::Luk => ItemStat(v.luk as u16),
            EquipStat::Hp => ItemStat(v.hp as u16),
            EquipStat::Mp => ItemStat(v.mp as u16),
            EquipStat::WeaponAtk => ItemStat(v.weapon_atk as u16),
            EquipStat::MagicAtk => ItemStat(v.magic_atk as u16),
            EquipStat::WeaponDef => ItemStat(v.weapon_def as u16),
            EquipStat::MagicDef => ItemStat(v.magic_def as u16),
            EquipStat::Accuracy => ItemStat(v.accuracy as u16),
            EquipStat::Avoid => ItemStat(v.avoid as u16),
            EquipStat::Craft => ItemStat(v.craft as u16),
            EquipStat::Speed => ItemStat(v.speed as u16),
            EquipStat::Jump => ItemStat(v.jump as u16)
        }
    }

    fn from_game_stats(v: proto95::shared::item::EquipStats) -> EquipStats {
        enum_map::enum_map! {
            EquipStat::Str => ItemStat(v.str),
            EquipStat::Dex => ItemStat(v.dex),
            EquipStat::Int => ItemStat(v.int),
            EquipStat::Luk => ItemStat(v.luk),
            EquipStat::Hp => ItemStat(v.hp),
            EquipStat::Mp => ItemStat(v.mp),
            EquipStat::WeaponAtk => ItemStat(v.watk),
            EquipStat::MagicAtk => ItemStat(v.matk),
            EquipStat::WeaponDef => ItemStat(v.wdef),
            EquipStat::MagicDef => ItemStat(v.mdef),
            EquipStat::Accuracy => ItemStat(v.accuracy),
            EquipStat::Avoid => ItemStat(v.avoid),
            EquipStat::Craft => ItemStat(v.craft),
            EquipStat::Speed => ItemStat(v.speed),
            EquipStat::Jump => ItemStat(v.jump)
        }
    }

    fn as_game_stats(&self) -> proto95::shared::item::EquipStats {
        proto95::shared::item::EquipStats {
            str: self[EquipStat::Str].0,
            dex: self[EquipStat::Dex].0,
            int: self[EquipStat::Int].0,
            luk: self[EquipStat::Luk].0,
            hp: self[EquipStat::Hp].0,
            mp: self[EquipStat::Mp].0,
            watk: self[EquipStat::WeaponAtk].0,
            matk: self[EquipStat::MagicAtk].0,
            wdef: self[EquipStat::WeaponDef].0,
            mdef: self[EquipStat::MagicDef].0,
            accuracy: self[EquipStat::Accuracy].0,
            avoid: self[EquipStat::Avoid].0,
            craft: self[EquipStat::Craft].0,
            speed: self[EquipStat::Speed].0,
            jump: self[EquipStat::Jump].0,
        }
    }

    fn from_equip_meta(meta: ItemMeta) -> EquipStats {
        enum_map::enum_map! {
            EquipStat::Str => ItemStat(meta.inc_str as u16),
            EquipStat::Dex => ItemStat(meta.inc_dex as u16),
            EquipStat::Int => ItemStat(meta.inc_int as u16),
            EquipStat::Luk => ItemStat(meta.inc_luk as u16),
            EquipStat::Hp => ItemStat(meta.inc_max_hp as u16),
            EquipStat::Mp => ItemStat(meta.inc_max_mp as u16),
            EquipStat::WeaponAtk => ItemStat(meta.inc_pad as u16),
            EquipStat::MagicAtk => ItemStat(meta.inc_mad as u16),
            EquipStat::WeaponDef => ItemStat(meta.inc_pdd as u16),
            EquipStat::MagicDef => ItemStat(meta.inc_mdd as u16),
            EquipStat::Accuracy => ItemStat(meta.inc_acc as u16),
            EquipStat::Avoid => ItemStat(meta.inc_eva as u16),
            EquipStat::Craft => ItemStat(meta.inc_craft as u16),
            EquipStat::Speed => ItemStat(meta.inc_speed as u16),
            EquipStat::Jump => ItemStat(meta.inc_jump as u16)
        }
    }

    fn apply_chaos_scroll(&mut self, mut rng: impl rand::Rng, range: RangeInclusive<i16>) {
        for stat in self.values_mut() {
            *stat = stat.apply_chaos(&mut rng, range.clone());
        }
    }
}
