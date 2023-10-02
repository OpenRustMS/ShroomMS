pub mod inventory_set;
pub mod skill;
pub mod stats;
pub mod util;

use std::{
    collections::VecDeque,
    ops::{Add, Div},
    time::Duration,
};

use either::Either;
use itertools::Itertools;
use proto95::{
    game::{
        script::ScriptMessage,
        user::secondary_stats::{
            CharSecondaryStatFlags, CharSecondaryStatPartial, CharSecondaryTwoStatesPartial,
            LocalSecondaryStatSetResp, TempStatValue,
        },
    },
    id::{item_id::InventoryType, job_id::JobId, FaceId, HairId, ItemId, MapId, SkillId, Skin},
    shared::{
        char::{AvatarData, AvatarEquips, CharStat, CharStatPartial, InventorySize, PetIds},
        inventory::InventoryOperation,
        FootholdId, Gender, Vec2,
    },
};
use shroom_pkt::{time::DurationMs, ShroomIndexList8};

use crate::entities::character::Model;

use self::{
    inventory_set::{CharInventory, InventorySet},
    skill::{SkillData, SkillSet},
    stats::CharStats,
};

use super::{
    data::{character::CharacterID, ItemService},
    helper::delay_queue::DelayQueue,
    meta::skill::CharBuffStat,
    SharedGameServices,
};

#[derive(Debug)]
pub enum TimerEvents {
    BuffTimeout(CharSecondaryStatFlags),
}

#[derive(Debug)]
pub struct Character {
    pub game: SharedGameServices,
    pub id: CharacterID,
    pub name: String,
    pub gender: Gender,
    pub stats: CharStats,
    pub inventory: CharInventory,
    pub map_id: MapId,
    pub spawn_point: u8,
    pub skin: Skin,
    pub hair: HairId,
    pub face: FaceId,
    pub inv_size: InventorySize,
    pub skills: SkillSet,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub secondary_stats: CharSecondaryStatPartial,
    pub secondary_stats_flags: CharSecondaryStatFlags,
    pub timer_events: DelayQueue<TimerEvents>,
    pub npc_msg: VecDeque<ScriptMessage>,
}

impl Character {
    pub fn new(
        game: SharedGameServices,
        model: Model,
        inventory: InventorySet,
        skills: SkillSet,
    ) -> Self {
        Self {
            game,
            id: model.id,
            stats: (&model).into(),
            inventory: CharInventory::from_inv_set(inventory),
            gender: (&model.gender).into(),
            name: model.name.clone(),
            map_id: MapId(model.map_id as u32),
            skin: Skin::try_from(model.skin as u8).expect("skin"),
            hair: HairId(model.hair as u32),
            face: FaceId(model.face as u32),
            inv_size: model.get_inventory_size(),
            spawn_point: model.spawn_point as u8,
            skills,
            pos: Vec2::new(0, 0),
            fh: 0,
            secondary_stats: CharSecondaryStatPartial::default(),
            secondary_stats_flags: CharSecondaryStatFlags::empty(),
            timer_events: DelayQueue::new(),
            npc_msg: VecDeque::default(),
        }
    }

    pub fn add_stack_item(
        &mut self,
        inv_ty: InventoryType,
        id: ItemId,
        quantity: usize,
    ) -> anyhow::Result<()> {
        let item = self.game.data.item.get_stack_item_from_id(id, quantity)?;
        self.inventory.try_add_stack_item(item, inv_ty)?;
        Ok(())
    }

    pub fn add_equip_item(&mut self, id: ItemId) -> anyhow::Result<()> {
        let item = self.game.data.item.get_eq_item_from_id(id)?;
        self.inventory.try_add_equip(item)?;
        Ok(())
    }

    pub fn get_stats_update(&mut self) -> Option<CharStatPartial> {
        self.stats.get_stats_partial()
    }

    pub fn handle_timer_events(&mut self) -> Option<CharSecondaryStatFlags> {
        let mut reset_flags = CharSecondaryStatFlags::empty();
        for event in self.timer_events.drain_expired() {
            match event {
                TimerEvents::BuffTimeout(flag) => {
                    reset_flags.insert(flag);
                }
            }
        }

        (!reset_flags.is_empty()).then_some(reset_flags)
    }

    pub fn get_secondary_stats_update(&mut self) -> Option<LocalSecondaryStatSetResp> {
        if !self.secondary_stats_flags.is_empty() {
            let stats = self.secondary_stats.clone();

            self.secondary_stats_flags = CharSecondaryStatFlags::empty();
            Some(LocalSecondaryStatSetResp {
                stats: stats.into(),
                defense_atk: 0,
                defense_state: 0,
                swallow_buff_time: None.into(),
                dice_info: Default::default(),
                blessing_armor_inc_pad: None.into(),
                two_states: CharSecondaryTwoStatesPartial::default(),
                delay: DurationMs(0),
                movement_affecting: Some(true).into(),
            })
        } else {
            None
        }
    }

    pub fn transfer_map(&mut self, map: MapId, sp: u8) {
        self.map_id = map;
        self.spawn_point = sp;
        // Reset the updates, since we use set field anyway
        self.stats.reset();
    }

    pub fn unlock_char(&mut self) {
        *self.stats.action_locked_mut() = false;
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

        let delta = (self.stats.exp as f64 * reduction_rate) as u32;

        self.stats
            .exp_mut()
            .force_update(|exp| *exp = exp.saturating_sub(delta));
    }

    pub fn update_mesos(&mut self, delta: i32) -> bool {
        self.stats.money_mut().force_update(|money| {
            *money = (*money).saturating_add_signed(delta).min(i32::MAX as u32)
        });
        true
    }

    pub fn get_map_id(&self) -> MapId {
        self.map_id
    }

    pub fn money(&self) -> u32 {
        self.stats.money
    }

    pub fn get_inv_slots(&self) -> InventorySize {
        self.inv_size
    }

    pub fn is_dead(&self) -> bool {
        self.stats.hp.value == 0
    }

    pub fn add_sp(&mut self, add: u32) {
        self.stats.sp_mut().force_update(|sp| *sp += add as u16);
    }

    pub fn change_job(&mut self, job: JobId, prev_skills: bool) -> anyhow::Result<()> {
        *self.stats.job_mut() = job;

        // Give new skills
        self.skills.add_skills(
            self.game.meta.get_skills_for_job(job).map(SkillData::from),
            false,
        );

        if prev_skills {
            for prev_job in job.prev_jobs() {
                self.skills.add_skills(
                    self.game
                        .meta
                        .get_skills_for_job(prev_job)
                        .map(SkillData::from),
                    false,
                );
            }
        }
        Ok(())
    }

    pub fn give_test_set(&mut self, data: &ItemService) -> anyhow::Result<()> {
        for item in [1432040, 1432028] {
            self.inventory.add_equip_by_id(ItemId(item), data)?;
        }

        Ok(())
    }

    pub fn respawn(&mut self) {
        self.stats.mp_mut().set_stat(1);
        self.stats.hp_mut().set_stat(1);
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
            hp: self.stats.hp.value.into(),
            max_hp: self.stats.hp.max.into(),
            mp: self.stats.mp.value.into(),
            max_mp: self.stats.mp.max.into(),
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
                    .item_with_slots()
                    .map(|(slot, item)| (slot as u8, item.0.item_id))
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

    pub fn get_inv_op_updates(&mut self) -> Option<Vec<InventoryOperation>> {
        if self.inventory.pending_operations.ops.is_empty() {
            return None;
        }
        let mut ops = Vec::new();
        std::mem::swap(&mut ops, &mut self.inventory.pending_operations.ops);
        Some(ops)
    }

    pub fn use_skill(&mut self, skill_id: SkillId) -> anyhow::Result<()> {
        let skill = self.skills.get(skill_id)?;
        let mp_cost = skill.mp_cost();
        let cd = Duration::from_secs(15);
        if let Some(cost) = mp_cost {
            if !self.stats.try_update_mp(-(cost as i16)) {
                return Ok(());
            }
        }

        // Give buff
        self.apply_buff(skill_id);
        *self.stats.action_locked_mut() = false;
        self.skills.set_cooldown(skill_id, cd);

        Ok(())
    }

    pub fn skill_up(&mut self, skill_id: SkillId) -> anyhow::Result<()> {
        if self.stats.sp == 0 {
            anyhow::bail!("Insufficient SP");
        }

        self.skills.skill_up(skill_id, 1)?;
        *self.stats.sp_mut() -= 1;
        Ok(())
    }

    pub fn apply_buff(&mut self, id: SkillId) {
        let skill = self.skills.get(id).unwrap();
        if !skill.is_buff() {
            return;
        }

        let map_tmp = |b: &super::meta::skill::TempStatValue| TempStatValue {
            value: b.value.eval(skill.level as i64).unwrap() as u16,
            reason: id.0,
            duration: Duration::from_secs(b.duration.eval(skill.level as i64).unwrap() as u64)
                .into(),
        };

        for buff in skill.meta.buff_stat.iter() {
            match buff {
                CharBuffStat::Pad(b) => {
                    self.secondary_stats.pad = Some(map_tmp(b)).into();
                }
                CharBuffStat::Pdd(b) => {
                    self.secondary_stats.pdd = dbg!(Some(map_tmp(b)).into());
                    self.secondary_stats_flags
                        .insert(CharSecondaryStatFlags::Pdd);
                }
                CharBuffStat::Mad(b) => {
                    self.secondary_stats.mad = Some(map_tmp(b)).into();
                }
                CharBuffStat::Mdd(b) => {
                    self.secondary_stats.mdd = Some(map_tmp(b)).into();
                }
                CharBuffStat::Acc(b) => {
                    self.secondary_stats.acc = Some(map_tmp(b)).into();
                }
                CharBuffStat::Evasion(b) => {
                    self.secondary_stats.evasion = Some(map_tmp(b)).into();
                }
                CharBuffStat::Craft(b) => {
                    self.secondary_stats.craft = Some(map_tmp(b)).into();
                }
                CharBuffStat::Speed(b) => {
                    let b = map_tmp(b);
                    self.timer_events.push_after(
                        TimerEvents::BuffTimeout(CharSecondaryStatFlags::Speed),
                        Duration::from_secs(b.duration.0 as u64 / 1000),
                    );
                    self.secondary_stats.speed = Some(b).into();
                    self.secondary_stats_flags
                        .insert(CharSecondaryStatFlags::Speed);
                }
                CharBuffStat::Jump(b) => {
                    self.secondary_stats.jump = Some(map_tmp(b)).into();
                }
                CharBuffStat::ExtraMaxHp(b) => {
                    self.secondary_stats.extramaxhp = Some(map_tmp(b)).into();
                }
                CharBuffStat::ExtraMaxMp(b) => {
                    self.secondary_stats.extramaxmp = Some(map_tmp(b)).into();
                }
                CharBuffStat::ExtraPad(b) => {
                    self.secondary_stats.extrapad = Some(map_tmp(b)).into();
                }
                CharBuffStat::ExtraPdd(b) => {
                    self.secondary_stats.extrapdd = Some(map_tmp(b)).into();
                }
                CharBuffStat::ExtraMdd(b) => {
                    self.secondary_stats.extrapdd = Some(map_tmp(b)).into();
                }
                CharBuffStat::MagicGuard(b) => {
                    self.secondary_stats.magicguard = Some(map_tmp(b)).into();
                }
                CharBuffStat::DarkSight(b) => {
                    self.secondary_stats.darksight = Some(map_tmp(b)).into();
                }
                CharBuffStat::Booster(b) => {
                    self.secondary_stats.booster = Some(map_tmp(b)).into();
                }
                CharBuffStat::PowerGuard(b) => {
                    self.secondary_stats.powerguard = Some(map_tmp(b)).into();
                }
                CharBuffStat::Guard(b) => {
                    self.secondary_stats.guard = Some(map_tmp(b)).into();
                }
                CharBuffStat::SafetyDamage(b) => {
                    self.secondary_stats.safetydamage = Some(map_tmp(b)).into();
                }
                CharBuffStat::SafetyAbsorb(b) => {
                    self.secondary_stats.safetyabsorb = Some(map_tmp(b)).into();
                }
                CharBuffStat::MaxHp(b) => {
                    self.secondary_stats.maxhp = Some(map_tmp(b)).into();
                }
                CharBuffStat::MaxMp(b) => {
                    self.secondary_stats.maxmp = Some(map_tmp(b)).into();
                }
                CharBuffStat::Invincible(b) => {
                    self.secondary_stats.invincible = Some(map_tmp(b)).into();
                }
                CharBuffStat::SoulArrow(b) => {
                    self.secondary_stats.soularrow = Some(map_tmp(b)).into();
                }
                CharBuffStat::Stun(b) => {
                    self.secondary_stats.stun = Some(map_tmp(b)).into();
                }
                CharBuffStat::Poison(b) => {
                    self.secondary_stats.poison = Some(map_tmp(b)).into();
                }
                CharBuffStat::Seal(b) => {
                    self.secondary_stats.seal = Some(map_tmp(b)).into();
                }
                CharBuffStat::Darkness(b) => {
                    self.secondary_stats.darkness = Some(map_tmp(b)).into();
                }
                CharBuffStat::ComboCounter(b) => {
                    self.secondary_stats.combocounter = Some(map_tmp(b)).into();
                }
                CharBuffStat::WeaponCharge(b) => {
                    self.secondary_stats.weaponcharge = Some(map_tmp(b)).into();
                }
                CharBuffStat::DragonBlood(b) => {
                    self.secondary_stats.dragonblood = Some(map_tmp(b)).into();
                }
                CharBuffStat::HolySymbol(b) => {
                    self.secondary_stats.holysymbol = Some(map_tmp(b)).into();
                }
                CharBuffStat::MesoUp(b) => {
                    self.secondary_stats.mesoup = Some(map_tmp(b)).into();
                }
                CharBuffStat::ShadowPartner(b) => {
                    self.secondary_stats.shadowpartner = Some(map_tmp(b)).into();
                }
                CharBuffStat::PickPocket(b) => {
                    self.secondary_stats.pickpocket = Some(map_tmp(b)).into();
                }
                CharBuffStat::MesoGuard(b) => {
                    self.secondary_stats.mesoguard = Some(map_tmp(b)).into();
                }
                CharBuffStat::Thaw(b) => {
                    self.secondary_stats.thaw = Some(map_tmp(b)).into();
                }
                CharBuffStat::Weakness(b) => {
                    self.secondary_stats.weakness = Some(map_tmp(b)).into();
                }
                CharBuffStat::Curse(b) => {
                    self.secondary_stats.curse = Some(map_tmp(b)).into();
                }
                CharBuffStat::Slow(b) => {
                    self.secondary_stats.slow = Some(map_tmp(b)).into();
                }
                CharBuffStat::Morph(b) => {
                    self.secondary_stats.morph = Some(map_tmp(b)).into();
                }
                CharBuffStat::Ghost(b) => {
                    self.secondary_stats.ghost = Some(map_tmp(b)).into();
                }
                CharBuffStat::Regen(b) => {
                    let b = map_tmp(b);

                    self.timer_events.push_after(
                        TimerEvents::BuffTimeout(CharSecondaryStatFlags::Regen),
                        Duration::from_secs(b.duration.0 as u64 / 1000),
                    );
                    self.secondary_stats.regen = Some(b).into();
                    self.secondary_stats_flags
                        .insert(CharSecondaryStatFlags::Regen);
                }
                CharBuffStat::BasicStatUp(b) => {
                    self.secondary_stats.basicstatup = Some(map_tmp(b)).into();
                }
                CharBuffStat::Stance(b) => {
                    self.secondary_stats.stance = Some(map_tmp(b)).into();
                }
                CharBuffStat::SharpEyes(b) => {
                    self.secondary_stats.sharpeyes = Some(map_tmp(b)).into();
                }
                CharBuffStat::ManaReflection(b) => {
                    self.secondary_stats.manareflection = Some(map_tmp(b)).into();
                }
                CharBuffStat::Attract(b) => {
                    self.secondary_stats.attract = Some(map_tmp(b)).into();
                }
                CharBuffStat::SpiritJavelin(b) => {
                    self.secondary_stats.spiritjavelin = Some(map_tmp(b)).into();
                }
                CharBuffStat::Infinity(b) => {
                    self.secondary_stats.infinity = Some(map_tmp(b)).into();
                }
                CharBuffStat::Holyshield(b) => {
                    self.secondary_stats.holyshield = Some(map_tmp(b)).into();
                }
                CharBuffStat::HamString(b) => {
                    self.secondary_stats.hamstring = Some(map_tmp(b)).into();
                }
                CharBuffStat::Blind(b) => {
                    self.secondary_stats.blind = Some(map_tmp(b)).into();
                }
                CharBuffStat::Concentration(b) => {
                    self.secondary_stats.concentration = Some(map_tmp(b)).into();
                }
                CharBuffStat::BanMap(b) => {
                    self.secondary_stats.banmap = Some(map_tmp(b)).into();
                }
                CharBuffStat::MaxLevelBuff(b) => {
                    self.secondary_stats.maxlevelbuff = Some(map_tmp(b)).into();
                }
                CharBuffStat::Barrier(b) => {
                    self.secondary_stats.barrier = Some(map_tmp(b)).into();
                }
                CharBuffStat::DojangShield(b) => {
                    self.secondary_stats.dojangshield = Some(map_tmp(b)).into();
                }
                CharBuffStat::ReverseInput(b) => {
                    self.secondary_stats.reverseinput = Some(map_tmp(b)).into();
                }
                CharBuffStat::MesoUpByItem(b) => {
                    self.secondary_stats.mesoupbyitem = Some(map_tmp(b)).into();
                }
                CharBuffStat::ItemUpByItem(b) => {
                    self.secondary_stats.itemupbyitem = Some(map_tmp(b)).into();
                }
                CharBuffStat::RespectPImmune(b) => {
                    self.secondary_stats.respectpimmune = Some(map_tmp(b)).into();
                }
                CharBuffStat::RespectMImmune(b) => {
                    self.secondary_stats.respectmimmune = Some(map_tmp(b)).into();
                }
                CharBuffStat::DefenseAtt(b) => {
                    self.secondary_stats.defenseatt = Some(map_tmp(b)).into();
                }
                CharBuffStat::DefenseState(b) => {
                    self.secondary_stats.defensestate = Some(map_tmp(b)).into();
                }
                CharBuffStat::DojangBerserk(b) => {
                    self.secondary_stats.dojangberserk = Some(map_tmp(b)).into();
                }
                CharBuffStat::DojangInvincible(b) => {
                    self.secondary_stats.dojanginvincible = Some(map_tmp(b)).into();
                }
                CharBuffStat::Spark(b) => {
                    self.secondary_stats.spark = Some(map_tmp(b)).into();
                }
                CharBuffStat::SoulMasterFinal(b) => {
                    self.secondary_stats.soulmasterfinal = Some(map_tmp(b)).into();
                }
                CharBuffStat::WindBreakerFinal(b) => {
                    self.secondary_stats.windbreakerfinal = Some(map_tmp(b)).into();
                }
                CharBuffStat::ElementalReset(b) => {
                    self.secondary_stats.elementalreset = Some(map_tmp(b)).into();
                }
                CharBuffStat::WindWalk(b) => {
                    self.secondary_stats.windwalk = Some(map_tmp(b)).into();
                }
                CharBuffStat::EventRate(b) => {
                    self.secondary_stats.eventrate = Some(map_tmp(b)).into();
                }
                CharBuffStat::ComboAbilityBuff(b) => {
                    self.secondary_stats.comboabilitybuff = Some(map_tmp(b)).into();
                }
                CharBuffStat::ComboDrain(b) => {
                    self.secondary_stats.combodrain = Some(map_tmp(b)).into();
                }
                CharBuffStat::ComboBarrier(b) => {
                    self.secondary_stats.combobarrier = Some(map_tmp(b)).into();
                }
                CharBuffStat::BodyPressure(b) => {
                    self.secondary_stats.bodypressure = Some(map_tmp(b)).into();
                }
                CharBuffStat::SmartKnockback(b) => {
                    self.secondary_stats.smartknockback = Some(map_tmp(b)).into();
                }
                CharBuffStat::RepeatEffect(b) => {
                    self.secondary_stats.repeateffect = Some(map_tmp(b)).into();
                }
                CharBuffStat::ExpBuffRate(b) => {
                    self.secondary_stats.expbuffrate = Some(map_tmp(b)).into();
                }
                CharBuffStat::IncEffectHPPotion(b) => {
                    self.secondary_stats.inceffecthppotion = Some(map_tmp(b)).into();
                }
                CharBuffStat::IncEffectMPPotion(b) => {
                    self.secondary_stats.inceffectmppotion = Some(map_tmp(b)).into();
                }
                CharBuffStat::StopPortion(b) => {
                    self.secondary_stats.stopportion = Some(map_tmp(b)).into();
                }
                CharBuffStat::StopMotion(b) => {
                    self.secondary_stats.stopmotion = Some(map_tmp(b)).into();
                }
                CharBuffStat::Fear(b) => {
                    self.secondary_stats.fear = Some(map_tmp(b)).into();
                }
                CharBuffStat::EvanSlow(b) => {
                    self.secondary_stats.evanslow = Some(map_tmp(b)).into();
                }
                CharBuffStat::MagicShield(b) => {
                    self.secondary_stats.magicshield = Some(map_tmp(b)).into();
                }
                CharBuffStat::MagicResistance(b) => {
                    self.secondary_stats.magicresistance = Some(map_tmp(b)).into();
                }
                CharBuffStat::SoulStone(b) => {
                    self.secondary_stats.soulstone = Some(map_tmp(b)).into();
                }
                CharBuffStat::Flying(b) => {
                    self.secondary_stats.flying = Some(map_tmp(b)).into();
                }
                CharBuffStat::Frozen(b) => {
                    self.secondary_stats.frozen = Some(map_tmp(b)).into();
                }
                CharBuffStat::AssistCharge(b) => {
                    self.secondary_stats.assistcharge = Some(map_tmp(b)).into();
                }
                CharBuffStat::Enrage(b) => {
                    self.secondary_stats.enrage = Some(map_tmp(b)).into();
                }
                CharBuffStat::SuddenDeath(b) => {
                    self.secondary_stats.suddendeath = Some(map_tmp(b)).into();
                }
                CharBuffStat::NotDamaged(b) => {
                    self.secondary_stats.notdamaged = Some(map_tmp(b)).into();
                }
                CharBuffStat::FinalCut(b) => {
                    self.secondary_stats.finalcut = Some(map_tmp(b)).into();
                }
                CharBuffStat::ThornsEffect(b) => {
                    self.secondary_stats.thornseffect = Some(map_tmp(b)).into();
                }
                CharBuffStat::SwallowAttackDamage(b) => {
                    self.secondary_stats.swallowattackdamage = Some(map_tmp(b)).into();
                }
                CharBuffStat::MorewildDamageUp(b) => {
                    self.secondary_stats.morewilddamageup = Some(map_tmp(b)).into();
                }
                CharBuffStat::Mine(b) => {
                    self.secondary_stats.mine = Some(map_tmp(b)).into();
                }
                CharBuffStat::Cyclone(b) => {
                    self.secondary_stats.cyclone = Some(map_tmp(b)).into();
                }
                CharBuffStat::SwallowCritical(b) => {
                    self.secondary_stats.swallowcritical = Some(map_tmp(b)).into();
                }
                CharBuffStat::SwallowMaxMP(b) => {
                    self.secondary_stats.swallowmaxmp = Some(map_tmp(b)).into();
                }
                CharBuffStat::SwallowDefence(b) => {
                    self.secondary_stats.swallowdefence = Some(map_tmp(b)).into();
                }
                CharBuffStat::SwallowEvasion(b) => {
                    self.secondary_stats.swallowevasion = Some(map_tmp(b)).into();
                }
                CharBuffStat::Conversion(b) => {
                    self.secondary_stats.conversion = Some(map_tmp(b)).into();
                }
                CharBuffStat::Revive(b) => {
                    self.secondary_stats.revive = Some(map_tmp(b)).into();
                }
                CharBuffStat::Sneak(b) => {
                    self.secondary_stats.sneak = Some(map_tmp(b)).into();
                }
                CharBuffStat::Mechanic(b) => {
                    self.secondary_stats.mechanic = Some(map_tmp(b)).into();
                }
                CharBuffStat::Aura(b) => {
                    self.secondary_stats.aura = Some(map_tmp(b)).into();
                }
                CharBuffStat::DarkAura(b) => {
                    self.secondary_stats.darkaura = Some(map_tmp(b)).into();
                }
                CharBuffStat::BlueAura(b) => {
                    self.secondary_stats.blueaura = Some(map_tmp(b)).into();
                }
                CharBuffStat::YellowAura(b) => {
                    self.secondary_stats.yellowaura = Some(map_tmp(b)).into();
                }
                CharBuffStat::SuperBody(b) => {
                    self.secondary_stats.superbody = Some(map_tmp(b)).into();
                }
                CharBuffStat::MorewildMaxHP(b) => {
                    self.secondary_stats.morewildmaxhp = Some(map_tmp(b)).into();
                }
                CharBuffStat::Dice(b) => {
                    self.secondary_stats.dice = Some(map_tmp(b)).into();
                }
                CharBuffStat::BlessingArmor(b) => {
                    self.secondary_stats.blessingarmor = Some(map_tmp(b)).into();
                }
                CharBuffStat::DamR(b) => {
                    self.secondary_stats.damr = Some(map_tmp(b)).into();
                }
                CharBuffStat::TeleportMasteryOn(b) => {
                    self.secondary_stats.teleportmasteryon = Some(map_tmp(b)).into();
                }
                CharBuffStat::CombatOrders(b) => {
                    self.secondary_stats.combatorders = Some(map_tmp(b)).into();
                }
                CharBuffStat::Beholder(b) => {
                    self.secondary_stats.beholder = Some(map_tmp(b)).into();
                }
            }
        }
    }
}
