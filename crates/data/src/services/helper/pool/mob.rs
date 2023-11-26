use std::collections::VecDeque;

use crate::services::{
    data::character::CharacterID,
    field::{FieldRoomSet, SessionMsg},
};

use meta::{FieldMob, MetaService, MobMeta};
use proto95::{
    game::{
        life::mob::{
            CarnivalTeam, LocalMobData, MobChangeControllerResp, MobControlLevel, MobDamagedResp,
            MobEnterFieldResp, MobHPIndicatorResp, MobId, MobInitData, MobLeaveFieldResp,
            MobLeaveType, MobMoveReq, MobMoveResp, MobSummonType, MobTemporaryStatPartial,
            PartialMobTemporaryStat,
        },
        ObjectId,
    },
    shared::{FootholdId, Vec2},
};
use shroom_pkt::util::packet_buf::PacketBuf;

use super::{next_id, Pool, PoolItem, SimplePool};

#[derive(Debug)]
pub struct Mob {
    pub meta: MobMeta,
    pub tmpl_id: MobId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub origin_fh: Option<FootholdId>,
    pub hp: u32,
    pub perc: u8,
    pub spawn_ix: Option<usize>,
}

impl Mob {
    pub fn damage(&mut self, dmg: u32) {
        self.hp = self.hp.saturating_sub(dmg);
        self.perc = ((self.hp * 100) / self.meta.max_hp) as u8;
    }

    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

#[derive(Debug)]
pub struct SpawnPoint {
    pub meta: MobMeta,
    pub tmpl_id: MobId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub origin_fh: Option<FootholdId>,
}

impl PoolItem for Mob {
    type Id = u32;

    type EnterPacket = MobEnterFieldResp;

    type LeavePacket = MobLeaveFieldResp;

    type LeaveParam = MobLeaveType;

    fn get_id(&self) -> Self::Id {
        next_id()
    }

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket {
        let empty_stats = PartialMobTemporaryStat {
            hdr: (),
            data: MobTemporaryStatPartial {
                ..Default::default()
            },
        };

        MobEnterFieldResp {
            id,
            calc_dmg_index: 5,
            tmpl_id: self.tmpl_id,
            stats: empty_stats,
            init_data: MobInitData {
                pos: self.pos,
                move_action: 3,
                fh: self.fh,
                origin_fh: self.origin_fh.unwrap_or(0), //char_fh
                summon_type: MobSummonType::Normal(()),
                carnival_team: CarnivalTeam::None,
                effect_id: 0,
                phase: 0,
            },
        }
    }

    fn get_leave_pkt(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeavePacket {
        MobLeaveFieldResp {
            id,
            leave_type: param,
        }
    }
}

#[derive(Debug)]
pub struct MobPool {
    pub mobs: SimplePool<Mob>,
    pub spawn_points: Vec<SpawnPoint>,
    pub respawn_queue: VecDeque<usize>,
}

impl Pool for MobPool {
    type Id = ObjectId;

    type Item = Mob;

    fn add_item(&mut self, id: Self::Id, item: Self::Item) -> anyhow::Result<()> {
        self.mobs.add_item(id, item)
    }

    fn remove_item(&mut self, id: &Self::Id) -> anyhow::Result<Option<Self::Item>> {
        self.mobs.remove_item(id)
    }

    fn on_enter(&self, packet_buf: &mut PacketBuf) -> anyhow::Result<()> {
        self.mobs.on_enter(packet_buf)
    }
}

impl MobPool {
    pub fn from_spawns(
        meta: &'static MetaService,
        spawns: impl Iterator<Item = (MobId, MobMeta, &'static FieldMob)>,
    ) -> Self {
        let mobs = SimplePool::new(meta);
        let mut spawn_points = Vec::new();
        for (id, meta, mob) in spawns {
            spawn_points.push(SpawnPoint {
                meta,
                tmpl_id: id,
                pos: mob.pos,
                fh: mob.fh as FootholdId,
                origin_fh: Some(mob.fh as FootholdId),
            });
        }
        let n = spawn_points.len();

        Self {
            mobs,
            spawn_points,
            respawn_queue: VecDeque::from_iter(0..n),
        }
    }

    pub fn respawn(&mut self, sessions: &FieldRoomSet) -> anyhow::Result<()> {
        if self.respawn_queue.is_empty() {
            return Ok(());
        }
        // TODO use a buffer
        while let Some(ix) = self.respawn_queue.pop_front() {
            let spawn = &self.spawn_points[ix];
            let mob = Mob {
                meta: spawn.meta,
                tmpl_id: spawn.tmpl_id,
                pos: spawn.pos,
                fh: spawn.fh,
                origin_fh: spawn.origin_fh,
                hp: spawn.meta.max_hp,
                perc: 100,
                spawn_ix: Some(ix),
            };

            self.mobs.add(mob, sessions)?;
            log::info!("Respawned mob {}", spawn.tmpl_id);
        }

        Ok(())
    }

    pub fn assign_controller(
        &self,
        session_id: CharacterID,
        sessions: &FieldRoomSet,
    ) -> anyhow::Result<()> {
        //TODO move out loop
        for (id, mob) in self.mobs.items.iter() {
            let empty_stats = PartialMobTemporaryStat {
                hdr: (),
                data: MobTemporaryStatPartial {
                    ..Default::default()
                },
            };

            let pkt = MobChangeControllerResp {
                level: MobControlLevel::Control,
                crc_seed: None.into(),
                id: *id,
                local_mob_data: Some(LocalMobData {
                    calc_damage_index: 5,
                    tmpl_id: mob.tmpl_id,
                    stats: empty_stats,
                })
                .into(),
            };

            sessions.send_to(&session_id, SessionMsg::from_packet(pkt))?;
        }
        Ok(())
    }

    pub fn attack_mob(
        &mut self,
        attacker: CharacterID,
        id: ObjectId,
        dmg: u32,
        buf: &mut PacketBuf,
        sessions: &FieldRoomSet,
    ) -> anyhow::Result<bool> {
        let mob = self
            .mobs
            .items
            .get_mut(&id)
            .ok_or(anyhow::format_err!("Invalid mob"))?;
        mob.damage(dmg);

        let pkt = MobDamagedResp {
            id,
            ty: 0,
            dec_hp: dmg,
            hp: mob.hp,
            max_hp: mob.meta.max_hp,
        };

        sessions.broadcast_filter(SessionMsg::from_packet(pkt), &attacker)?;

        buf.encode_packet(MobHPIndicatorResp {
            id,
            hp_perc: mob.perc,
        })?;

        Ok(mob.is_dead())
    }

    pub fn mob_move(
        &mut self,
        id: ObjectId,
        req: MobMoveReq,
        controller: CharacterID,
        sessions: &FieldRoomSet,
    ) -> anyhow::Result<()> {
        let Some(mob) = self.mobs.items.get_mut(&id) else {
            return Ok(());
        };

        let last_pos_fh = req.move_path.path.get_last_pos_fh();

        if let Some((pos, fh)) = last_pos_fh {
            //TODO post mob state to msg state here
            mob.pos = pos;
            mob.fh = fh.unwrap_or(mob.fh);
        }

        let pkt = MobMoveResp {
            id,
            not_force_landing: false,
            not_change_action: false,
            next_attack_possible: false,
            action_dir: req.action_dir,
            data: req.data,
            multi_target: req.multi_target,
            rand_time: req.rand_time,
            move_path: req.move_path.path,
        };

        sessions.broadcast_filter(SessionMsg::from_packet(pkt), &controller)?;
        Ok(())
    }

    pub fn kill_mob(&mut self, id: ObjectId, sessions: &FieldRoomSet) -> anyhow::Result<Mob> {
        let mob = self.remove(id, MobLeaveType::Etc(()), sessions)?;
        if let Some(ix) = mob.spawn_ix {
            self.respawn_queue.push_back(ix);
        }

        Ok(mob)
    }
}
