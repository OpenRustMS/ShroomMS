use std::time::{Duration, Instant};

use crate::services::helper::delay_queue::DelayQueue;
use bitflags::Flags;
use proto95::game::user::secondary_stats::{CharSecondaryStatFlags, TempStatValue};

pub type BuffEpoch = usize;

pub trait Buff {
    fn get_expiration(&self) -> Option<Instant>;
    fn get_epoch(&self) -> BuffEpoch;
    fn set_epoch(&mut self, epoch: BuffEpoch);

    fn is_expired(&self, now: Instant) -> bool {
        self.get_expiration().map(|t| t <= now).unwrap_or(false)
    }

    fn get_duration(&self) -> Option<Duration> {
        self.get_expiration().map(|t| {
            t.checked_duration_since(Instant::now())
                .unwrap_or_else(|| Duration::from_secs(0))
        })
    }
}

pub trait BuffStorage {
    type Flag: bitflags::Flags + Clone;

    type BasicBuff: Buff;
    type ExtBuff: Buff;

    fn insert_buff(&mut self, flag: Self::Flag, buff: Self::BasicBuff);
    fn insert_ext_buff(&mut self, flag: Self::Flag, buff: Self::ExtBuff);

    fn remove_buff(&mut self, flag: Self::Flag, epoch: BuffEpoch) -> bool;

    fn get_buff(&self, flag: Self::Flag) -> Option<&Self::BasicBuff>;
    fn get_buff_mut(&mut self, flag: Self::Flag) -> Option<&mut Self::BasicBuff>;
}

#[derive(Debug)]
pub struct BuffManager<T: BuffStorage> {
    pub update_mask: T::Flag,
    pub buff_mask: T::Flag,
    pub storage: T,
    pub expiring_buffs: DelayQueue<(T::Flag, BuffEpoch)>,
    epoch: BuffEpoch,
}

impl<T: BuffStorage> BuffManager<T> {
    pub fn new(storage: T) -> Self {
        Self {
            update_mask: T::Flag::empty(),
            buff_mask: T::Flag::empty(),
            storage,
            expiring_buffs: DelayQueue::new(),
            epoch: 0,
        }
    }

    fn set_buff_flag(&mut self, flag: T::Flag, dur: Option<Duration>) {
        if let Some(dur) = dur {
            self.expiring_buffs
                .push((flag.clone(), self.epoch), Instant::now() + dur);
        }
        self.update_mask.insert(flag.clone());
        self.buff_mask.insert(flag);
        self.epoch += 1;
    }

    pub fn set_buff(&mut self, flag: T::Flag, mut buff: T::BasicBuff) {
        buff.set_epoch(self.epoch);
        let dur = buff.get_duration();
        self.storage.insert_buff(flag.clone(), buff);
        self.set_buff_flag(flag, dur);
    }

    pub fn set_ext_buff(&mut self, flag: T::Flag, mut buff: T::ExtBuff) {
        buff.set_epoch(self.epoch);
        let dur = buff.get_duration();
        self.storage.insert_ext_buff(flag.clone(), buff);
        self.set_buff_flag(flag, dur);
    }

    pub fn update_buff(&mut self, flag: T::Flag, f: impl FnOnce(&mut T::BasicBuff) -> bool) {
        if let Some(buff) = self.storage.get_buff_mut(flag.clone()) {
            if f(buff) {
                self.update_mask.insert(flag);
            }
        }
    }

    pub fn set_update(&mut self, flag: T::Flag) {
        self.update_mask.insert(flag);
    }

    pub fn get_updates(&mut self) -> Option<T::Flag> {
        if self.update_mask.is_empty() {
            return None;
        }

        let mask = self.update_mask.clone();
        self.update_mask = T::Flag::empty();
        Some(mask)
    }

    pub fn get_buff(&self, flag: T::Flag) -> Option<&T::BasicBuff> {
        self.storage.get_buff(flag)
    }

    pub fn get_buff_mut(&mut self, flag: T::Flag) -> Option<&mut T::BasicBuff> {
        self.storage.get_buff_mut(flag)
    }

    pub fn tick(&mut self) -> Option<T::Flag> {
        let mut remove_mask = T::Flag::empty();
        while let Some((buff_flag, epoch)) = self.expiring_buffs.pop() {
            if self.storage.remove_buff(buff_flag.clone(), epoch) {
                self.buff_mask.remove(buff_flag.clone());
                remove_mask.insert(buff_flag);
            }
        }

        if remove_mask.is_empty() {
            return None;
        }
        Some(remove_mask)
    }
}

#[derive(Debug)]
pub struct CharBuff {
    pub buff_id: u32,
    pub value: i16,
    pub max_value: Option<i16>,
    pub expiration: Instant,
    pub epoch: BuffEpoch,
}

impl Buff for CharBuff {
    fn get_expiration(&self) -> Option<Instant> {
        Some(self.expiration)
    }

    fn get_epoch(&self) -> BuffEpoch {
        self.epoch
    }

    fn set_epoch(&mut self, epoch: BuffEpoch) {
        self.epoch = epoch;
    }
}

impl CharBuff {
    pub fn new(buff_id: u32, value: i16, dur: Duration) -> Self {
        Self {
            buff_id,
            value,
            expiration: Instant::now() + dur,
            epoch: 0,
            max_value: None
        }
    }
}

impl From<&CharBuff> for TempStatValue {
    fn from(val: &CharBuff) -> Self {
        TempStatValue {
            value: val.value,
            reason: val.buff_id,
            duration: val.get_duration().unwrap().into(),
        }
    }
}

#[derive(Debug)]
pub struct CharBuffStorage {
    buffs: [Option<CharBuff>; 128],
}

impl Default for CharBuffStorage {
    fn default() -> Self {
        const INIT: Option<CharBuff> = None;
        Self { buffs: [INIT; 128] }
    }
}

impl CharBuffStorage {
    fn flag_to_ix(flag: CharSecondaryStatFlags) -> usize {
        flag.bits().trailing_zeros() as usize
    }
}

impl BuffStorage for CharBuffStorage {
    type Flag = CharSecondaryStatFlags;

    type BasicBuff = CharBuff;

    type ExtBuff = CharBuff;

    fn insert_buff(&mut self, flag: Self::Flag, buff: Self::BasicBuff) {
        self.buffs[Self::flag_to_ix(flag)] = Some(buff);
    }

    fn insert_ext_buff(&mut self, flag: Self::Flag, buff: Self::ExtBuff) {
        self.buffs[Self::flag_to_ix(flag)] = Some(buff);
    }

    fn remove_buff(&mut self, flag: Self::Flag, epoch: usize) -> bool {
        let ix = Self::flag_to_ix(flag);
        if let Some(ref buff) = self.buffs[ix] {
            if buff.get_epoch() == epoch {
                self.buffs[ix] = None;
                return true;
            }
        }

        false
    }
    
    fn get_buff(&self, flag: CharSecondaryStatFlags) -> Option<&CharBuff> {
        let ix = CharBuffStorage::flag_to_ix(flag);
        self.buffs[ix].as_ref()
    }

    fn get_buff_mut(&mut self, flag: CharSecondaryStatFlags) -> Option<&mut CharBuff> {
        let ix = CharBuffStorage::flag_to_ix(flag);
        self.buffs[ix].as_mut()
    }
}
