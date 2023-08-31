pub mod inventory_set;

use std::ops::{Add, Div};

use either::Either;
use itertools::Itertools;
use proto95::{
    id::{job_id::JobId, FaceId, HairId, ItemId, MapId, Skin},
    shared::{
        char::{AvatarData, AvatarEquips, CharStat, CharStatPartial, PetIds},
        inventory::InventoryOperation,
        Gender,
    },
};
use shroom_net::packet::{CondOption, ShroomIndexList8};

use crate::entities::character::Model;

use self::inventory_set::{CharInventory, InventorySet};

use super::data::character::CharacterID;

#[derive(Debug, Clone, PartialEq)]
pub struct ClampedStat<T> {
    pub value: T,
    pub min: T,
    pub max: T,
}

#[tracker::track]
#[derive(Debug, Clone)]
pub struct TrackedCharStats {
    hp: u16,
    mp: u16,
    maxhp: u16,
    maxmp: u16,
    str: u16,
    dex: u16,
    int: u16,
    luk: u16,
    money: u32,
    exp: u32,
    job: JobId,
    ap: u16,
    sp: u16,
    fame: u16,
    level: u8,
    action_locked: bool,
}

impl From<&Model> for TrackedCharStats {
    fn from(value: &Model) -> Self {
        TrackedCharStats {
            hp: value.hp as u16,
            mp: value.mp as u16,
            maxhp: value.max_hp as u16,
            maxmp: value.max_mp as u16,
            str: value.str as u16,
            dex: value.dex as u16,
            int: value.int as u16,
            luk: value.luk as u16,
            money: value.mesos as u32,
            exp: value.exp as u32,
            job: JobId::try_from(value.job as u16).expect("Job"),
            ap: value.ap as u16,
            sp: value.sp as u16,
            fame: value.fame as u16,
            level: value.level as u8,
            tracker: 0,
            action_locked: true,
        }
    }
}

macro_rules! map_partial_stats {
    ($stats:expr, $update_stats:ident, $($stat:ident,)*) => {
        $(if $stats.changed(TrackedCharStats::$stat()) {
            $update_stats.$stat = CondOption(Some($stats.$stat.into()));
        })*
    };
}

#[derive(Debug)]
pub struct Character {
    //pub model: Model,
    pub id: CharacterID,
    pub name: String,
    pub gender: Gender,
    pub stats: TrackedCharStats,
    pub inventory: CharInventory,
    pub map_id: MapId,
    pub spawn_point: u8,
    pub skin: Skin,
    pub hair: HairId,
    pub face: FaceId,
    pub inv_size: [u8; 5],
}

impl Character {
    pub fn new(model: Model, inventory: InventorySet) -> Self {
        Self {
            id: model.id,
            stats: (&model).into(),
            inventory: CharInventory::from_inv_set(inventory),
            gender: (&model.gender).into(),
            name: model.name.clone(),
            map_id: MapId(model.map_id as u32),
            skin: Skin::try_from(model.skin as u8).expect("skin"),
            hair: HairId(model.hair as u32),
            face: FaceId(model.face as u32),
            inv_size: [
                model.equip_slots as u8,
                model.use_slots as u8,
                model.setup_slots as u8,
                model.etc_slots as u8,
                model.cash_slots as u8,
            ],
            spawn_point: model.spawn_point as u8,
        }
    }

    pub fn transfer_map(&mut self, map: MapId, sp: u8) {
        self.map_id = map;
        self.spawn_point = sp;
        self.get_stats_partial();
    }

    pub fn unlock_char(&mut self) {
        self.stats.set_action_locked(false)
    }

    pub fn decrease_exp(&mut self, town: bool) {
        if self.stats.exp <= 200 {
            return;
        }

        let reduction_rate = match town {
            true => 0.01,
            false => {
                let temp_rate = if self.stats.job.job_level() == 0 {
                    0.08
                } else {
                    0.2
                };
                temp_rate.div((self.stats.luk as f64).add(0.05))
            }
        };

        let delta = (self.stats.exp as f64 * reduction_rate) as i32;

        self.stats
            .set_exp(self.stats.exp.saturating_add_signed(delta));
    }

    pub fn update_hp(&mut self, d: i16) {
        let hp = self.stats.get_hp().saturating_add_signed(d);
        self.stats.set_hp(hp.clamp(0, self.stats.maxhp));
    }

    pub fn update_mp(&mut self, d: i16) {
        let mp = self.stats.get_mp().saturating_add_signed(d);
        self.stats.set_mp(mp.clamp(0, self.stats.maxmp));
    }

    pub fn update_mesos(&mut self, delta: i32) -> bool {
        self.stats.set_money(
            self.stats
                .money
                .saturating_add_signed(delta)
                .min(i32::MAX as u32),
        );
        // Return for drop
        true
    }

    pub fn stats_changed(&self) -> bool {
        self.stats.changed(TrackedCharStats::track_all())
    }

    pub fn get_map_id(&self) -> MapId {
        self.map_id
    }

    pub fn money(&self) -> u32 {
        self.stats.money
    }

    pub fn get_inv_slots(&self) -> [u8; 5] {
        self.inv_size
    }

    pub fn is_dead(&self) -> bool {
        self.stats.hp == 0
    }

    pub fn respawn(&mut self) {
        self.stats.set_hp(1);
        self.stats.set_mp(1);
    }

    pub fn get_all_stats(&self) -> CharStat {
        let (job_id, sub_job) = (self.stats.job, 0);

        CharStat {
            char_id: self.id as u32,
            skin_color: self.skin,
            face: self.face,
            hair: self.hair,
            level: self.stats.level,
            str: self.stats.str,
            dex: self.stats.dex,
            int: self.stats.int,
            luk: self.stats.luk,
            hp: self.stats.hp.into(),
            max_hp: self.stats.maxhp.into(),
            mp: self.stats.mp.into(),
            max_mp: self.stats.maxmp.into(),
            ap: self.stats.ap,
            sp: Either::Right(self.stats.sp).into(),
            exp: self.stats.exp as i32,
            fame: self.stats.fame,
            tmp_exp: 0,
            name: self.name.as_str().try_into().expect("Name"),
            gender: self.gender,
            pets: [0; 3],
            job_id,
            map_id: self.map_id,
            portal: 0,
            playtime: 0,
            sub_job,
        }
    }

    pub fn get_stats_partial(&mut self) -> CharStatPartial {
        let mut update_stats = CharStatPartial::default();

        map_partial_stats!(
            self.stats,
            update_stats,
            hp,
            maxhp,
            mp,
            maxmp,
            money,
            exp,
            job,
            str,
            dex,
            int,
            luk,
            ap,
            sp,
            fame,
            level,
        );

        self.stats.reset();
        self.stats.action_locked = true;

        update_stats
    }

    pub fn get_avatar_data(&self) -> AvatarData {
        AvatarData {
            gender: self.gender,
            skin: self.skin,
            mega: true,
            face: self.face,
            hair: self.hair,
            equips: AvatarEquips {
                equips: self
                    .inventory
                    .invs
                    .equipped
                    .items_with_slots()
                    .map(|(slot, item)| (slot as u8, item.item_id))
                    .collect_vec()
                    .into(),
                masked_equips: ShroomIndexList8::from(vec![]),
                weapon_sticker_id: ItemId(0),
            },
            pets: PetIds::default(),
        }
    }

    pub fn is_inventory_changed(&self) -> bool {
        !self.inventory.pending_operations.ops.is_empty()
    }

    pub fn get_inventory_ops(&mut self) -> Vec<InventoryOperation> {
        let mut ops = Vec::new();
        std::mem::swap(&mut ops, &mut self.inventory.pending_operations.ops);
        ops
    }
}
