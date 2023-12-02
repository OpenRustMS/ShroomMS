use shroom_pkt::{
    mark_shroom_bitflags, packet_with_opcode, CondOption, PacketWrapped, ShroomList32, ShroomList8,
    ShroomOption8, ShroomPacket, ShroomPacketEnum,
};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes, shared::char::CashID};
#[derive(ShroomPacket, Debug)]
pub struct MsgParam(u8);

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct MsgParamFlags: u8 {
        const SPEAKER_TEMPLATE = 4;
    }
}
mark_shroom_bitflags!(MsgParamFlags);

impl MsgParamFlags {
    pub fn has_override_speaker(&self) -> bool {
        self.contains(MsgParamFlags::SPEAKER_TEMPLATE)
    }
}

#[derive(ShroomPacket, Debug)]
pub struct MsgHeader {
    param: MsgParamFlags,
    color: u8,
    // TODO: this is not used for every msg
    #[pkt(check(cond = "MsgParamFlags::has_override_speaker", field = "param"))]
    override_speaker: CondOption<u32>,
}

#[derive(ShroomPacket, Debug)]
pub struct SayMsg {
    pub param: MsgParamFlags,
    #[pkt(check(cond = "MsgParamFlags::has_override_speaker", field = "param"))]
    pub speaker_tmpl_id: CondOption<u32>,
    pub txt: String,
    pub has_prev: bool,
    pub has_next: bool,
}

#[derive(ShroomPacket, Debug)]
pub struct SayImageMsg {
    pub param: MsgParamFlags,
    pub images: ShroomList8<String>,
}

#[derive(ShroomPacket, Debug)]
pub struct AskTextMsg {
    // TODO: Override speaker option?
    pub param: MsgParamFlags,
    pub msg: String,
    pub default_txt: String,
    pub min: u16,
    pub max: u16,
}

#[derive(ShroomPacket, Debug)]
pub struct AskNumberMsg {
    pub param: MsgParamFlags,
    pub msg: String,
    pub default_number: u32,
    pub min: u32,
    pub max: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct AskSelectMsg {
    pub param: MsgParamFlags,
    pub dlg_ty: u32, // TODO Should be <= 1 ???
    pub default_select: u32,
    pub options: ShroomList32<String>,
}

#[derive(ShroomPacket, Debug)]
pub struct AskMsg {
    pub param: MsgParamFlags,
    pub msg: String,
}

#[derive(ShroomPacket, Debug)]
pub struct AskAvatarMsg {
    pub param: MsgParamFlags,
    pub msg: String,
    pub avatars: ShroomList8<u32>,
}

#[derive(ShroomPacket, Debug)]
pub struct AskPetMsg {
    pub param: MsgParamFlags,
    pub msg: String,
    /// The latter u8 is unused
    pub pets: ShroomList8<(CashID, u8)>,
}

#[derive(ShroomPacket, Debug)]
pub struct AskSliderMsg {
    pub param: MsgParamFlags,
    pub mode: u32, // TODO: 0 seems slide menu ex, 1 slide menu normal
    pub value: u32,
    pub msg: String,
}

// TODO each one has to decode the header
#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum ScriptMessage {
    Say(SayMsg) = 0,
    SayImage(ShroomList8<String>) = 1,
    AskYesNo(AskMsg) = 2,
    AskText(AskTextMsg) = 3,
    AskNumber(AskNumberMsg) = 4,
    AskMenu(AskMsg) = 5,
    // TODO: Quiz
    AskAvatar(AskAvatarMsg) = 8,
    AskMembershopAvatar(AskAvatarMsg) = 9,
    AskPet(AskPetMsg) = 10,
    // TODO: Special case with a byte after size of list, AskPet(AskPetMsg) = 11,
    AskYesNoQuest(AskMsg) = 13,
    AskBoxText(AskTextMsg) = 14,
    AskSlider(AskSliderMsg) = 15,
}

#[derive(ShroomPacket, Debug)]
pub struct ScriptMessageResp {
    pub script_flag: u8,
    pub speaker_id: u32,
    pub msg: ScriptMessage,
}
packet_with_opcode!(ScriptMessageResp, SendOpcodes::ScriptMessage);

#[derive(Debug)]
pub struct OptionAnswer(pub Option<bool>);

impl OptionAnswer {
    pub fn is_quit(&self) -> bool {
        self.0.is_none()
    }
}

impl PacketWrapped for OptionAnswer {
    type Inner = u8;
    type IntoValue<'a> = Self::Inner;  
    
    fn packet_from(inner: Self::Inner) -> Self {
        Self(match inner {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        })
    }
    fn packet_into_inner(&self) -> Self::Inner {
        self.0.map(|b| b as u8).unwrap_or(0xff)
    }
}

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum ScriptAnswerReq {
    /// True if next
    PrevNext(OptionAnswer) = 0,
    ImgNext(OptionAnswer) = 1,
    YesNo(OptionAnswer) = 2,
    InputText(ShroomOption8<String>) = 3,
    InputNumber(u32) = 4,
    InputSelection(ShroomOption8<u32>) = 5,
    /// Avatar Index
    AvatarSelection(ShroomOption8<u8>) = 8,
    /// Avatar Index
    AvatarMembershipSelection(ShroomOption8<u8>) = 9,
    PetSelection(ShroomOption8<CashID>) = 10,
    //TODO: This one is special it has an extra byte after size PetSelectionAll(ShroomOption8<CashID>) = 11,
    InputBoxText(ShroomOption8<String>) = 14,
    InputSliderValue(ShroomOption8<u32>) = 15,
}
packet_with_opcode!(ScriptAnswerReq, RecvOpcodes::UserScriptMessageAnswer);
