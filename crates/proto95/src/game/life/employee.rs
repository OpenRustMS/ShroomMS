use shroom_pkt::{packet_opcode, ShroomPacket, ShroomOption8};

use crate::{
    game::ObjectId,
    send_opcodes::SendOpcodes,
    shared::{FootholdId, Vec2},
};

#[derive(ShroomPacket, Debug)]
pub struct EmployeeMiniRoomBalloon {
    pub sn: u32,
    pub spec: u8,
    pub cur_users: u8,
    pub max_users: u8

}

#[derive(ShroomPacket, Debug)]
pub struct EmployeeCreateResp {
    pub id: ObjectId,
    pub employee_tmpl_id: u32,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub char_name: String,
    // TODO: the u8 is mini room ty
    pub balloon: ShroomOption8<EmployeeMiniRoomBalloon>
}
packet_opcode!(EmployeeCreateResp, SendOpcodes::EmployeeEnterField);


#[derive(ShroomPacket, Debug)]
pub struct EmployeeMiniRoomBalloonResp {
    pub employee_id: ObjectId,
    // TODO: the u8 is mini room ty
    pub balloon: ShroomOption8<EmployeeMiniRoomBalloon>
}
packet_opcode!(EmployeeMiniRoomBalloonResp, SendOpcodes::EmployeeMiniRoomBalloon);

#[derive(ShroomPacket, Debug)]
pub struct EmployeeRemoveResp {
    pub id: ObjectId,
}
packet_opcode!(EmployeeRemoveResp, SendOpcodes::EmployeeLeaveField);
