use std::ops::{Add, Div};

use proto95::shared::char::{CharStatFlags, CharStatPartial};
use shroom_net::packet::CondOption;

use crate::{entities::character::Model, services::helper::intentory::inv::InventorySet};

#[derive(Debug, Clone)]
pub struct Character {
    pub model: Model,
    pub inventory: InventorySet,
    char_stat_flags: CharStatFlags,
}

impl Character {
    pub fn new(model: Model, inventory: InventorySet) -> Self {
        Self {
            model,
            inventory,
            char_stat_flags: CharStatFlags::empty(),
        }
    }

    pub fn decrease_exp(&mut self, town: bool) {
        if self.model.exp <= 0 || self.model.exp >= 200 {
            return;
        }

        let reduction_rate = match town {
            true => 0.01,
            false => {
                let temp_rate = if self.model.job.eq(&3) { 0.08 } else { 0.2 };
                temp_rate.div((self.model.luk as f64).add(0.05))
            }
        };

        // set exp to the max of 0 or the current exp minus the next level xp times reduction rate
        // TODO: get next level xp
        self.model.exp = 0.max(self.model.exp - (self.model.exp as f64 * reduction_rate) as i32);
    }

    pub fn update_hp(&mut self, hp: i32) {
        self.model.hp = 0.max(self.model.hp.add(hp)).min(self.model.max_hp);
        self.char_stat_flags.insert(CharStatFlags::Hp);
    }

    pub fn update_mp(&mut self, mp: i32) {
        self.model.mp = 0.max(self.model.mp.add(mp)).min(self.model.max_mp);
        self.char_stat_flags.insert(CharStatFlags::Mp);
    }

    pub fn update_mesos(&mut self, mesos: i32) -> bool {
        if self.model.mesos + mesos < 0 {
            return false;
        }
        self.model.mesos = self.model.mesos.saturating_add(mesos);
        self.char_stat_flags.insert(CharStatFlags::Money);
        true
    }

    pub fn get_char_partial(&mut self) -> CharStatPartial {
        let mut stats = CharStatPartial::default();

        if self.char_stat_flags.contains(CharStatFlags::Hp) {
            stats.hp = CondOption(Some(self.model.hp as u32));
            self.char_stat_flags.remove(CharStatFlags::Hp);
        }
        if self.char_stat_flags.contains(CharStatFlags::Mp) {
            stats.mp = CondOption(Some(self.model.mp as u32));
            self.char_stat_flags.remove(CharStatFlags::Mp);
        }
        if self.char_stat_flags.contains(CharStatFlags::Money) {
            stats.money = CondOption(Some(self.model.mesos as u32));
            self.char_stat_flags.remove(CharStatFlags::Money);
        }

        stats
    }
}
