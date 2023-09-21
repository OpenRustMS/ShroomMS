pub mod drop;
pub mod mob;
pub mod npc;
pub mod reactor;
pub mod user;

pub use drop::Drop;

pub use mob::Mob;
pub use npc::Npc;
use shroom_pkt::{util::packet_buf::PacketBuf, EncodePacket, HasOpcode};

use std::{
    collections::BTreeMap,
    sync::atomic::AtomicU32,
};

use proto95::game::ObjectId;
use std::fmt::Debug;

use crate::services::{
    data::character::CharacterID,
    field::{FieldRoomSet, SessionMsg},
    meta::meta_service::MetaService,
};

pub fn next_id() -> ObjectId {
    static ID: AtomicU32 = AtomicU32::new(0);
    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub trait PoolId {}

pub trait PoolItem {
    type Id: Clone + Eq;
    type EnterPacket: EncodePacket + HasOpcode;
    type LeavePacket: EncodePacket + HasOpcode;
    type LeaveParam;

    fn get_id(&self) -> Self::Id;

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket;
    fn get_leave_pkt(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeavePacket;
}

#[derive(Debug)]
pub struct Pool<T>
where
    T: PoolItem<Id = ObjectId>,
{
    items: BTreeMap<T::Id, T>,
    meta: &'static MetaService,
}

impl<T> Pool<T>
where
    T: PoolItem<Id = ObjectId>,
{
    pub fn new(meta: &'static MetaService) -> Self {
        Self {
            items: BTreeMap::new(),
            meta,
        }
    }
    pub fn from_elems(meta: &'static MetaService, elems: impl Iterator<Item = T>) -> Self {
        let mut pool = Pool::new(meta);
        pool.items
            .extend(elems.map(|item| (T::get_id(&item), item)));
        pool
    }

    pub fn update(&mut self, id: ObjectId, update: impl Fn(&mut T)) {
        if let Some(item) = self.items.get_mut(&id) {
            update(item);
        }
    }

    pub fn add(&mut self, item: T, sessions: &FieldRoomSet) -> anyhow::Result<u32> {
        let id = T::get_id(&item);
        let pkt = item.get_enter_pkt(id);
        self.items.insert(id, item);

        sessions.broadcast(SessionMsg::from_packet(pkt))?;
        Ok(id)
    }

    pub fn add_filter(
        &mut self,
        item: T,
        sessions: &FieldRoomSet,
        src: CharacterID,
    ) -> anyhow::Result<u32> {
        let id = T::get_id(&item);
        let pkt = item.get_enter_pkt(id);
        self.items.insert(id, item);

        sessions.broadcast_filter(SessionMsg::from_packet(pkt), &src)?;
        Ok(id)
    }

    pub fn remove(
        &mut self,
        id: T::Id,
        param: T::LeaveParam,
        sessions: &FieldRoomSet,
    ) -> anyhow::Result<T> {
        //TODO migrate to actors
        let Some(item) = self.items.remove(&id) else {
            anyhow::bail!("Item does not exist");
        };

        let pkt = item.get_leave_pkt(id, param);
        sessions.broadcast(SessionMsg::from_packet(pkt))?;
        Ok(item)
    }

    pub fn on_enter(&self, packet_buf: &mut PacketBuf) -> anyhow::Result<()> {
        for (id, pkt) in self.items.iter() {
            packet_buf.encode_packet(pkt.get_enter_pkt(*id))?;
        }

        Ok(())
    }
}
