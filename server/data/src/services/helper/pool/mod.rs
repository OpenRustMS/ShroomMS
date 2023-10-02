pub mod drop;
pub mod mob;
pub mod npc;
pub mod reactor;
pub mod user;

pub use drop::Drop;

pub use mob::Mob;
pub use npc::Npc;
use shroom_pkt::{util::packet_buf::PacketBuf, EncodePacket, HasOpcode};

use std::{collections::BTreeMap, sync::atomic::AtomicU32};

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
    type Id: Clone + Eq + std::fmt::Debug;
    type EnterPacket: EncodePacket + HasOpcode;
    type LeavePacket: EncodePacket + HasOpcode;
    type LeaveParam;

    fn get_id(&self) -> Self::Id;

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket;
    fn get_leave_pkt(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeavePacket;
}

pub trait Pool {
    type Id: Clone + Eq + std::fmt::Debug;
    type Item: PoolItem<Id = Self::Id>;

    fn add_item(&mut self, id: Self::Id, item: Self::Item) -> anyhow::Result<()>;
    fn remove_item(&mut self, id: &Self::Id) -> anyhow::Result<Option<Self::Item>>;

    fn add_filter(
        &mut self,
        item: Self::Item,
        sessions: &FieldRoomSet,
        src: CharacterID,
    ) -> anyhow::Result<Self::Id> {
        let id = Self::Item::get_id(&item);
        let pkt = item.get_enter_pkt(id.clone());
        self.add_item(id.clone(), item)?;

        sessions.broadcast_filter(SessionMsg::from_packet(pkt), &src)?;
        Ok(id)
    }

    fn add(&mut self, item: Self::Item, sessions: &FieldRoomSet) -> anyhow::Result<Self::Id> {
        let id = Self::Item::get_id(&item);
        let pkt = item.get_enter_pkt(id.clone());
        self.add_item(id.clone(), item)?;

        sessions.broadcast(SessionMsg::from_packet(pkt))?;
        Ok(id)
    }

    fn remove(
        &mut self,
        id: Self::Id,
        param: <Self::Item as PoolItem>::LeaveParam,
        sessions: &FieldRoomSet,
    ) -> anyhow::Result<Self::Item> {
        let Some(item) = self.remove_item(&id)? else {
            anyhow::bail!("Item does not exist");
        };

        let pkt = item.get_leave_pkt(id, param);
        sessions.broadcast(SessionMsg::from_packet(pkt))?;
        Ok(item)
    }

    fn on_enter(&self, packet_buf: &mut PacketBuf) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct SimplePool<T: PoolItem> {
    items: BTreeMap<T::Id, T>,
    meta: &'static MetaService,
}

impl<T: PoolItem> Pool for SimplePool<T>
where
    T::Id: Ord,
{
    type Item = T;
    type Id = T::Id;

    fn add_item(&mut self, id: T::Id, item: Self::Item) -> anyhow::Result<()> {
        self.items.insert(id, item);
        Ok(())
    }

    fn remove_item(&mut self, id: &T::Id) -> anyhow::Result<Option<Self::Item>> {
        Ok(self.items.remove(id))
    }

    fn on_enter(&self, packet_buf: &mut PacketBuf) -> anyhow::Result<()> {
        for (id, pkt) in self.items.iter() {
            packet_buf.encode_packet(pkt.get_enter_pkt(id.clone()))?;
        }

        Ok(())
    }
}

impl<T> SimplePool<T>
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
        let mut pool = SimplePool::new(meta);
        pool.items
            .extend(elems.map(|item| (T::get_id(&item), item)));
        pool
    }
}
