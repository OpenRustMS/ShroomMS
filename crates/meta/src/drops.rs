use std::{collections::BTreeMap, ops::RangeInclusive};

use proto95::{
    game::{mob::MobId, reactor::ReactorId},
    id::ItemId,
};
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReactorDrop {
    #[serde(rename = "reactorId")]
    pub reactor_id: Option<u32>,
    #[serde(rename = "reactorAction")]
    pub reactor_action: Option<String>,
    #[serde(rename = "itemId")]
    pub item_id: u32,
    #[serde(rename = "chance")]
    pub chance: f32,
    #[serde(rename = "questId")]
    pub quest_id: Option<u32>,
}

pub type ReactorDropList = Vec<ReactorDrop>;

#[derive(Debug, Deserialize)]
pub struct MobDrop {
    #[serde(rename = "mobId")]
    pub mob_id: u32,
    #[serde(rename = "itemId")]
    pub item_id: u32,
    #[serde(rename = "minQ")]
    pub min_q: u16,
    #[serde(rename = "maxQ")]
    pub max_q: u16,
    #[serde(rename = "questId")]
    pub quest_id: u16,
    pub chance: f32,
}

pub type MobDropList = Vec<MobDrop>;

#[derive(Debug, Deserialize)]
pub struct NpcShopItem {
    #[serde(rename = "itemId")]
    pub item_id: u32,
    #[serde(rename = "itemPeriod")]
    pub item_period: u16,
    pub price: u32,
    pub position: u32,
}

#[derive(Debug, Deserialize)]
pub struct NpcShop {
    pub id: u32,
    pub items: Vec<NpcShopItem>,
}

pub type NpcShops = BTreeMap<u32, NpcShop>;

#[derive(Debug)]
pub struct DropEntry {
    pub item: ItemId,
    pub quantity: RangeInclusive<u32>,
    pub chance: f32,
}

impl DropEntry {
    pub fn get_with_rand(&self, rng: &mut impl Rng) -> Option<(ItemId, usize)> {
        if !rng.gen_bool(self.chance.into()) {
            return None;
        }

        Some((self.item, rng.gen_range(self.quantity.clone()) as usize))
    }
}

#[derive(Debug)]
pub struct DropPool {
    pub mob_drops: BTreeMap<MobId, Vec<DropEntry>>,
    pub reactor_drops: BTreeMap<ReactorId, Vec<DropEntry>>,
    pub money: u32,
    pub money_variance: u32,
}

impl DropPool {
    pub fn from_drop_lists(mob_list: MobDropList, reactor_list: ReactorDropList) -> Self {
        let mut mob_drops = BTreeMap::<MobId, Vec<DropEntry>>::new();
        for drop in mob_list {
            mob_drops.entry(drop.mob_id).or_default().push(DropEntry {
                item: ItemId(drop.item_id),
                quantity: (drop.min_q as u32)..=(drop.max_q as u32),
                chance: drop.chance,
            });
        }

        let mut reactor_drops = BTreeMap::<ReactorId, Vec<DropEntry>>::new();
        for drop in reactor_list {
            reactor_drops
                .entry(drop.reactor_id.unwrap_or(0))
                .or_default()
                .push(DropEntry {
                    item: ItemId(drop.item_id),
                    quantity: 1..=1,
                    chance: drop.chance,
                });
        }

        Self {
            mob_drops,
            reactor_drops,
            money: 50_000,
            money_variance: 49_999,
        }
    }

    pub fn get_drops_for_mob<R: Rng>(&self, mob_id: MobId, rng: &mut R) -> Vec<(ItemId, usize)> {
        self.mob_drops
            .get(&mob_id)
            .into_iter()
            .flat_map(|drops| {
                drops
                    .iter()
                    .filter_map(|drop| drop.get_with_rand(rng))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn get_reactor_drops<R: Rng>(
        &self,
        reactor_id: ReactorId,
        rng: &mut R,
    ) -> Vec<(ItemId, usize)> {
        self.reactor_drops
            .get(&reactor_id)
            .into_iter()
            .flat_map(|drops| {
                drops
                    .iter()
                    .filter_map(|drop| drop.get_with_rand(rng))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn get_money_drop<R: Rng>(&self, rng: &mut R) -> u32 {
        rng.gen_range((self.money - self.money_variance)..=self.money)
    }
}
