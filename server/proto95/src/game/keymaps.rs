use shroom_net_derive::ShroomPacket;
use shroom_net::{packet::{proto::option::ShroomOptionR8}, packet_opcode};

use crate::send_opcodes::SendOpcodes;

#[derive(Debug, ShroomPacket, Default, Clone, Copy)]
pub struct KeyBinding {
    pub ty: u8,
    pub action_id: u32,
}

#[derive(Debug, ShroomPacket)]
pub struct FuncKeyMapInitResp {
    // Reversed option, if set to none the default key map is used
    pub key_bindings: ShroomOptionR8<[KeyBinding; 90]>,
}

impl FuncKeyMapInitResp {
    pub fn default_map() -> Self {
        Self { key_bindings: None.into() }
    }
}

packet_opcode!(FuncKeyMapInitResp, SendOpcodes::FuncKeyMappedInit);
