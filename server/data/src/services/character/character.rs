use std::ops::{Add, Div};

use crate::{entities, services::helper::intentory::inv::InventorySet};

#[derive(Debug, Clone)]
pub struct Character {
    pub model: entities::character::Model,
    pub inventory: InventorySet,
}

impl Character {
    pub fn new(model: entities::character::Model, inventory: InventorySet) -> Self {
        Self { model, inventory }
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
    }

    pub fn update_mp(&mut self, mp: i32) {
        self.model.mp = 0.max(self.model.mp.add(mp)).min(self.model.max_mp);
    }

    pub fn update_mesos(&mut self, mesos: i32) -> bool {
        if self.model.mesos + mesos < 0 {
            return false;
        }
        self.model.mesos = self.model.mesos.checked_add(mesos).unwrap_or(i32::MAX);
        true
    }
}
