use shroom_pkt::{packet_with_opcode, ShroomOptionR8, ShroomPacket};

use crate::send_opcodes::SendOpcodes;

#[derive(Debug, ShroomPacket, Default, Clone, Copy)]
pub struct KeyBinding {
    pub ty: u8,
    pub action_id: u32,
}

#[derive(Debug, ShroomPacket)]
pub struct FuncKeyMapInitResp {
    /// None means the default key map is used
    pub key_bindings: ShroomOptionR8<[KeyBinding; 90]>,
}

impl FuncKeyMapInitResp {
    pub fn default_map() -> Self {
        Self {
            key_bindings: None.into(),
        }
    }
}

packet_with_opcode!(FuncKeyMapInitResp, SendOpcodes::FuncKeyMappedInit);
