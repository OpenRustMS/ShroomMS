use std::{ops::Add, time::Duration};

use proto95::{
    game::{
        drop::{
            DropEnterFieldResp, DropEnterType, DropId, DropLeaveFieldResp, DropLeaveType,
            DropOwner, DropType,
        },
        mob::MobId,
        ObjectId,
    },
    id::ItemId,
    shared::Vec2,
};

use shroom_pkt::ShroomExpirationTime;

use crate::services::{data::character::CharacterID, field::FieldRoomSet, meta::fh_tree::Foothold};

use super::{next_id, Pool, PoolItem, SimplePool};

#[derive(Debug)]
pub struct Drop {
    pub owner: DropOwner,
    pub pos: Vec2,
    pub start_pos: Vec2,
    pub value: DropTypeValue,
    pub quantity: usize,
}

#[derive(Debug)]
pub enum DropTypeValue {
    Mesos(u32),
    Item(ItemId),
}

#[derive(Debug)]
pub enum DropLeaveParam {
    TimeOut,
    ScreenScroll,
    UserPickup(u32),
    MobPickup(u32),
    Explode,
    PetPickup(u32),
    PassConvex,
    PetSkill,
}

impl PoolItem for Drop {
    type Id = ObjectId;

    type EnterPacket = DropEnterFieldResp;
    type LeavePacket = DropLeaveFieldResp;
    type LeaveParam = DropLeaveParam;

    fn get_id(&self) -> Self::Id {
        next_id()
    }

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket {
        let (drop_type, expiration) = match self.value {
            DropTypeValue::Item(item) => (
                DropType::Item(item),
                Some(ShroomExpirationTime::delay(chrono::Duration::seconds(60))),
            ),
            DropTypeValue::Mesos(mesos) => (DropType::Money(mesos), None),
        };

        let start_pos = (
            self.start_pos.add(Vec2::new(0, -20)),
            Duration::from_millis(100).into(),
        );

        DropEnterFieldResp {
            enter_type: DropEnterType::Create,
            id,
            drop_type,
            drop_owner: self.owner.clone(),
            pos: self.pos,
            src_id: 0,
            start_pos: Some(start_pos).into(),
            drop_expiration: expiration.into(),
            by_pet: false,
            u1_flag: false,
        }
    }

    fn get_leave_pkt(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeavePacket {
        let (leave_type, pickup_id) = match param {
            DropLeaveParam::Explode => (DropLeaveType::Explode, None),
            DropLeaveParam::PassConvex => (DropLeaveType::PassConvex, None),
            DropLeaveParam::PetSkill => (DropLeaveType::PetSkill, None),
            DropLeaveParam::ScreenScroll => (DropLeaveType::ScreenScroll, None),
            DropLeaveParam::TimeOut => (DropLeaveType::TimeOut, None),
            DropLeaveParam::UserPickup(id) => (DropLeaveType::UserPickup, Some(id)),
            DropLeaveParam::MobPickup(id) => (DropLeaveType::MobPickup, Some(id)),
            DropLeaveParam::PetPickup(id) => (DropLeaveType::PetPickup, Some(id)),
        };

        DropLeaveFieldResp {
            leave_type,
            id,
            pickup_id: pickup_id.into(),
        }
    }
}

impl SimplePool<Drop> {
    pub fn is_money(&self, item: DropId) -> Option<u32> {

        match self.items.get(&item) {
            Some(i) => match i.value {
                DropTypeValue::Item(_) => None,
                DropTypeValue::Mesos(m) => Some(m),
            },
            None => None,
        }
    }

    pub fn add_mob_drops(
        &mut self,
        killed_mob: MobId,
        pos: Vec2,
        fh: Option<&Foothold>,
        killer: CharacterID,
        sessions: &FieldRoomSet,
    ) -> anyhow::Result<()> {
        let Some(drops) = self.meta.get_drops_for_mob(killed_mob) else {
            return Ok(());
        };

        let money = drops.get_money_drop(&mut rand::thread_rng());
        let items = drops.get_item_drops(&mut rand::thread_rng());

        let n = items.len() + usize::from(money > 0);
        // Get spread for items + mesos, TODO mesos are optional, fix items being zero
        let mut spread = fh.map(|fh| fh.get_item_spread(pos.x as f32, n));

        fn map_coord(c: geo::Coord<f32>) -> Vec2 {
            Vec2::new(c.x as i16, c.y as i16)
        }

        if money > 0 {
            self.add(
                Drop {
                    owner: DropOwner::User(killer as u32),
                    pos: Vec2::from(
                        spread
                            .as_mut()
                            .and_then(|fh| fh.next().map(map_coord))
                            .unwrap_or(pos),
                    ),
                    start_pos: pos,
                    value: DropTypeValue::Mesos(money),
                    quantity: 1,
                },
                sessions,
            )?;
        }

        for (item, quantity) in items {
            self.add(
                Drop {
                    owner: DropOwner::User(killer as u32),
                    pos: spread
                        .as_mut()
                        .and_then(|fh| fh.next().map(map_coord))
                        .unwrap_or(pos),
                    start_pos: pos,
                    value: DropTypeValue::Item(item),
                    quantity,
                },
                sessions,
            )?;
        }

        Ok(())
    }
}
