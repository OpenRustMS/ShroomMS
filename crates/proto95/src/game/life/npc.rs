use shroom_pkt::{packet_with_opcode, ShroomList8, ShroomPacket};

use crate::{
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{char::AvatarData, movement::MovePath, FootholdId, Range2, Vec2}, game::ObjectId,
};


pub type NpcId = u32;

#[derive(ShroomPacket, Debug)]
pub struct NpcPoolPacket<T> {
    pub id: ObjectId,
    pub data: T,
}

#[derive(ShroomPacket, Debug)]
pub struct NpcInitData {
    pub pos: Vec2,
    pub move_action: u8,
    pub fh: FootholdId,
    pub range_horz: Range2,
    pub enabled: bool,
}

#[derive(ShroomPacket, Debug)]
pub struct NpcEnterFieldResp {
    pub id: ObjectId,
    pub template_id: NpcId,
    pub init: NpcInitData,
}
packet_with_opcode!(NpcEnterFieldResp, SendOpcodes::NpcEnterField);

#[derive(ShroomPacket, Debug)]
pub struct NpcLeaveFieldResp {
    pub id: ObjectId,
}
packet_with_opcode!(NpcLeaveFieldResp, SendOpcodes::NpcLeaveField);

#[derive(ShroomPacket, Debug)]
pub struct NpcImitateData {
    pub tmpl_id: NpcId,
    pub name: String,
    pub avatar_look: AvatarData,
}

#[derive(ShroomPacket, Debug)]
pub struct NpcImitateDataResp {
    pub data: ShroomList8<NpcImitateData>,
}
packet_with_opcode!(NpcImitateDataResp, SendOpcodes::ImitatedNPCData);

#[derive(ShroomPacket, Debug)]
pub struct NpcUpdateLimitedDisableInfoResp {
    pub data: ShroomList8<ObjectId>,
}
packet_with_opcode!(
    NpcUpdateLimitedDisableInfoResp,
    SendOpcodes::LimitedNPCDisableInfo
);

#[derive(ShroomPacket, Debug)]
pub struct NpcChangeControllerResp {
    pub local: bool,
    pub id: ObjectId,
    //TODO only decoded if local == true
    pub tmpl_id: NpcId,
    pub init_data: NpcInitData,
}
packet_with_opcode!(NpcChangeControllerResp, SendOpcodes::NpcChangeController);

#[derive(ShroomPacket, Debug)]
pub struct ScriptInfo {
    pub script: String,
    pub start_date: u32,
    pub end_date: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct ModScript {
    pub template_id: u32,
    pub script: ScriptInfo,
}

#[derive(ShroomPacket, Debug)]
pub struct NpcSetScriptResp {
    pub scripts: ShroomList8<ModScript>,
}
packet_with_opcode!(NpcSetScriptResp, SendOpcodes::NpcSetScript);

#[derive(ShroomPacket, Debug)]
pub struct NpcMove {
    pub action: u8,
    pub chat: u8, //TODO correct?
    pub move_path: MovePath,
}
pub type NpcMoveResp = NpcPoolPacket<NpcMove>;
packet_with_opcode!(NpcMoveResp, SendOpcodes::NpcMove);

#[derive(ShroomPacket, Debug)]
pub struct NpcUpdateLimitedInfo {
    pub enabled: bool,
}
pub type NpcUpdateLimitedInfoResp = NpcPoolPacket<NpcUpdateLimitedInfo>;
packet_with_opcode!(NpcUpdateLimitedInfoResp, SendOpcodes::NpcUpdateLimitedInfo);

#[derive(ShroomPacket, Debug)]
pub struct NpcSetSpecialAction {
    pub action: String,
}
pub type NpcSetSpecialActionResp = NpcPoolPacket<NpcSetSpecialAction>;
packet_with_opcode!(NpcSetSpecialActionResp, SendOpcodes::NpcSpecialAction);

#[derive(ShroomPacket, Debug)]
pub struct UserSelectNpcReq {
    pub id: NpcId,
    pub pos: Vec2,
}
packet_with_opcode!(UserSelectNpcReq, RecvOpcodes::UserSelectNpc);