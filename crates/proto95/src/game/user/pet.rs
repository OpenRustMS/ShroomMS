use shroom_pkt::{packet_opcode, shroom_enum_code, ShroomList8, ShroomPacket};

use crate::{
    id::ItemId,
    send_opcodes::SendOpcodes,
    shared::{char::CharacterId, movement::MovePath, Vec2, FootholdId},
};

#[derive(ShroomPacket, Debug)]
pub struct PetMoveResp {
    pub user: CharacterId,
    pub pet_id: u8,
    pub move_path: MovePath,
}
packet_opcode!(PetMoveResp, SendOpcodes::PetMove);

#[derive(ShroomPacket, Debug)]
pub struct PetNameChangedResp {
    pub user: CharacterId,
    pub pet_id: u8,
    pub name: String,
    pub name_tag: bool,
}
packet_opcode!(PetNameChangedResp, SendOpcodes::PetNameChanged);

#[derive(ShroomPacket, Debug)]
pub struct PetExceptionListResp {
    pub user: CharacterId,
    pub pet_id: u8,
    pub pet_sn: u64,
    pub exception_list: ShroomList8<ItemId>,
}
packet_opcode!(PetExceptionListResp, SendOpcodes::PetLoadExceptionList);

#[derive(ShroomPacket, Debug)]
pub struct PetActionResp {
    pub user: CharacterId,
    pub pet_id: u8,
    pub ty: u8,
    pub action: u8,
    pub chat: String,
    pub chat_balloon: bool,
}
packet_opcode!(PetActionResp, SendOpcodes::PetAction);

shroom_enum_code!(
    PetActivateError,
    u8,
    None = 0,
    PetWentHome = 1,
    PetMagicalTimeExpired = 2,
    UnableToUsePet = 3,
    CannotSummon = 4
);

#[derive(ShroomPacket, Debug)]
pub struct PetActivateResp {
    pub user: CharacterId,
    pub pet_id: u8,
    pub succesful: bool,
    pub error: PetActivateError,
    pub pet_tmpl_id: u32,
    pub peta_name: String,
    pub pet_locker_sn: u64,
    pub pos: Vec2,
    pub move_action: u8,
    pub fh: FootholdId,
    pub name_tag: bool,
    pub chat_balloon: bool,
}
packet_opcode!(PetActivateResp, SendOpcodes::PetActivated);


