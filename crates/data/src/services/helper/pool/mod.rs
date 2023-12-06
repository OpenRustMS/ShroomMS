pub mod drop;
pub mod mob;
pub mod npc;
pub mod reactor;
pub mod summoned;
pub mod user;
pub use drop::Drop;

use meta::MetaService;
pub use mob::Mob;
pub use npc::Npc;
use shroom_pkt::{util::packet_buf::PacketBuf, ShroomPacket};
use shroom_srv::srv::{room_set::RoomSessionSet, server_room::RoomSessionHandler};

use std::{collections::BTreeMap, sync::atomic::AtomicU32};

use proto95::game::ObjectId;
use std::fmt::Debug;

pub fn next_id() -> ObjectId {
    static ID: AtomicU32 = AtomicU32::new(0);
    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub trait PoolId {}

pub trait PoolItem {
    type Id: Clone + Eq + std::fmt::Debug;
    type EnterPacket: ShroomPacket;
    type LeavePacket: ShroomPacket;
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

    fn add_filter<H: RoomSessionHandler>(
        &mut self,
        item: Self::Item,
        sessions: &mut RoomSessionSet<H>,
        src: H::SessionId,
    ) -> anyhow::Result<Self::Id> {
        let id = Self::Item::get_id(&item);
        let pkt = item.get_enter_pkt(id.clone());
        self.add_item(id.clone(), item)?;
        sessions.broadcast_encode_filter(pkt, src)?;
        Ok(id)
    }

    fn add<H: RoomSessionHandler>(
        &mut self,
        item: Self::Item,
        sessions: &mut RoomSessionSet<H>,
    ) -> anyhow::Result<Self::Id> {
        let id = Self::Item::get_id(&item);
        let pkt = item.get_enter_pkt(id.clone());
        self.add_item(id.clone(), item)?;
        sessions.broadcast_encode(pkt)?;
        Ok(id)
    }

    fn remove<H: RoomSessionHandler>(
        &mut self,
        id: Self::Id,
        param: <Self::Item as PoolItem>::LeaveParam,
        sessions: &mut RoomSessionSet<H>,
    ) -> anyhow::Result<Self::Item> {
        let Some(item) = self.remove_item(&id)? else {
            anyhow::bail!("Item does not exist");
        };

        let pkt = item.get_leave_pkt(id, param);
        sessions.broadcast_encode(pkt)?;
        Ok(item)
    }

    fn on_enter(&self, packet_buf: &mut PacketBuf) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct SimplePool<T: PoolItem> {
    items: BTreeMap<T::Id, T>,
    pub meta: &'static MetaService,
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
            packet_buf.encode(pkt.get_enter_pkt(id.clone()))?;
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

    pub fn must_get_mut(&mut self, id: &ObjectId) -> anyhow::Result<&mut T> {
        self.items
            .get_mut(id)
            .ok_or_else(|| anyhow::anyhow!("Item does not exist"))
    }

    pub fn must_get(&self, id: &ObjectId) -> anyhow::Result<&T> {
        self.items
            .get(id)
            .ok_or_else(|| anyhow::anyhow!("Item does not exist"))
    }

    pub fn from_elems(meta: &'static MetaService, elems: impl Iterator<Item = T>) -> Self {
        let mut pool = SimplePool::new(meta);
        pool.items
            .extend(elems.map(|item| (T::get_id(&item), item)));
        pool
    }
}
