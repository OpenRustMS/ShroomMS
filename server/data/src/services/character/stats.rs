use num::Saturating;
use proto95::{id::job_id::JobId, shared::char::CharStatPartial};

use crate::entities::character;

pub trait ClampedStatNum: num::Unsigned + Saturating + Ord + Clone + Copy {
    type Signed: num::Signed + Ord;

    fn try_clamp_add(&self, max: Self, delta: Self::Signed) -> Option<Self>;
    fn clamp_add(&mut self, max: Self, delta: Self::Signed);
}

impl ClampedStatNum for u16 {
    type Signed = i16;

    fn try_clamp_add(&self, max: Self, delta: Self::Signed) -> Option<Self> {
        if let Some(v) = self.checked_add_signed(delta) {
            if v <= max {
                return Some(v);
            }
        }

        None
    }

    fn clamp_add(&mut self, max: Self, delta: Self::Signed) {
        let v = self.saturating_add_signed(delta);
        *self = v.min(max);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClampedStat<T> {
    pub value: T,
    pub max: T,
}

impl<T: ClampedStatNum> ClampedStat<T> {
    pub fn new(value: T, max: T) -> Self {
        Self { value, max }
    }

    pub fn add(&mut self, delta: T::Signed) {
        self.value.clamp_add(self.max, delta);
    }

    pub fn try_add(&self, delta: T::Signed) -> Option<Self> {
        let max = self.max;
        self.value
            .try_clamp_add(max, delta)
            .map(|value| Self { value, max })
    }

    pub fn set_stat(&mut self, val: T) {
        self.value = self.max.min(val);
    }

    pub fn update_max(&mut self, max: T) {
        self.max = max;
        self.value = self.max.min(self.value);
    }
}

macro_rules! map_partial_stats {
    ($stats:expr, $update_stats:ident, $($stat:ident,)*) => {
        $(if $stats.flags.contains(CharStatsFlags::$stat) {
            $update_stats.$stat = Some($stats.$stat.into()).into();
        })*
    };
}

#[derive(Debug, Clone, trackr::Tracked)]
pub struct CharStats {
    #[track(flag)]
    flags: CharStatsFlags,
    pub hp: ClampedStat<u16>,
    pub mp: ClampedStat<u16>,
    pub str: u16,
    pub dex: u16,
    pub int: u16,
    pub luk: u16,
    pub money: u32,
    pub exp: u32,
    pub job: JobId,
    pub ap: u16,
    pub sp: u16,
    pub fame: u16,
    pub level: u8,
    pub action_locked: bool,
}

impl From<&character::Model> for CharStats {
    fn from(value: &character::Model) -> Self {
        CharStats {
            hp: ClampedStat::new(value.hp as u16, value.max_hp as u16),
            mp: ClampedStat::new(value.mp as u16, value.max_mp as u16),
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
            flags: Default::default(),
            action_locked: true,
        }
    }
}

impl CharStats {
    pub fn reset(&mut self) {
        self.flags = CharStatsFlags::empty();
        self.action_locked = true;
    }

    pub fn set_hp(&mut self, hp: u16) {
        self.hp_mut().value = hp;
    }

    pub fn update_hp(&mut self, d: i16) {
        self.hp_mut().add(d);
    }

    pub fn update_mp(&mut self, d: i16) {
        self.mp_mut().add(d);
    }

    pub fn try_update_hp(&mut self, d: i16) -> bool {
        if let Some(hp) = self.hp.try_add(d) {
            self.hp_mut().force_set(hp);
            true
        } else {
            false
        }
    }

    pub fn try_update_mp(&mut self, d: i16) -> bool {
        if let Some(mp) = self.mp.try_add(d) {
            self.mp_mut().force_set(mp);
            true
        } else {
            false
        }
    }

    pub fn get_stats_partial(&mut self) -> Option<CharStatPartial> {
        if self.flags.is_empty() {
            return None;
        }

        let mut update_stats = CharStatPartial::default();

        if self.flags.contains(CharStatsFlags::hp) {
            update_stats.hp = Some(self.hp.value as u32).into();
            update_stats.maxhp = Some(self.hp.max as u32).into();
        }

        if self.flags.contains(CharStatsFlags::hp) {
            update_stats.hp = Some(self.hp.value as u32).into();
            update_stats.maxhp = Some(self.hp.max as u32).into();
        }

        map_partial_stats!(
            self,
            update_stats,
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

        self.reset();

        Some(update_stats)
    }
}
