// Derive may generate a: a for initialization
#![allow(clippy::redundant_field_names)]
#![allow(non_upper_case_globals)]

use shroom_pkt::{DecodePacket, EncodePacket};

pub mod fmt;
pub mod game;
pub mod id;
pub mod login;
pub mod recv_opcodes;
pub mod send_opcodes;
pub mod shared;

#[derive(Debug)]
pub struct DebugPkt<T>(pub T);

impl<T: EncodePacket + std::fmt::Debug> EncodePacket for DebugPkt<T> {
    const SIZE_HINT: shroom_pkt::SizeHint = T::SIZE_HINT;

    fn packet_len(&self) -> usize {
        self.0.packet_len()
    }

    fn encode_packet<B: bytes::BufMut>(
        &self,
        pw: &mut shroom_pkt::PacketWriter<B>,
    ) -> shroom_pkt::PacketResult<()> {
        dbg!(&self.0);
        self.0.encode_packet(pw)
    }
}

impl<'de, T: DecodePacket<'de> + std::fmt::Debug> DecodePacket<'de> for DebugPkt<T> {
    fn decode_packet(pr: &mut shroom_pkt::PacketReader<'de>) -> shroom_pkt::PacketResult<Self> {
        let pkt = T::decode_packet(pr)?;
        dbg!(&pkt);
        Ok(DebugPkt(pkt))
    }
}
