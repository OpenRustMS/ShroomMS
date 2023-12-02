use std::num::Saturating;

use proto95::{
    game::{
        life::reactor::{ReactorEnterFieldResp, ReactorId, ReactorLeaveFieldResp},
        ObjectId,
    },
    shared::Vec2,
};

use super::{next_id, PoolItem};

#[derive(Debug)]
pub struct Reactor {
    pub pos: Vec2,
    pub tmpl_id: ReactorId,
    pub state: Saturating<u8>,
    pub name: Option<String>,
}

impl PoolItem for Reactor {
    type Id = ObjectId;

    type EnterPacket = ReactorEnterFieldResp;

    type LeavePacket = ReactorLeaveFieldResp;

    type LeaveParam = ();

    fn get_id(&self) -> Self::Id {
        next_id()
    }

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket {
        ReactorEnterFieldResp {
            id,
            tmpl_id: self.tmpl_id,
            state: self.state.0,
            pos: self.pos,
            flipped: false,
            name: String::new(),
        }
    }

    fn get_leave_pkt(&self, id: Self::Id, _param: Self::LeaveParam) -> Self::LeavePacket {
        ReactorLeaveFieldResp {
            id,
            state: self.state.0,
            pos: self.pos,
        }
    }
}
