use shroom_pkt::{PacketTryWrapped, ShroomOption8, ShroomPacket, ShroomPacketEnum};

use crate::{
    id::{FieldId, SkillId},
    login::world::ChannelId,
    shared::{char::CharacterId, NameStr},
};

use super::life::mob::MobId;

pub type PartyID = u32;
const MAX_PARTY_MEMBERS: usize = 6;

#[derive(Debug, Default, Copy, Clone)]
pub enum PartyMemberStatus {
    #[default]
    Offline,
    CashShop,
    Online(ChannelId),
}

impl PacketTryWrapped for PartyMemberStatus {
    type Inner = i32;

    fn packet_into_inner(&self) -> Self::Inner {
        match self {
            Self::Offline => -2,
            Self::CashShop => -1,
            Self::Online(channel_id) => *channel_id as i32,
        }
    }

    fn packet_try_from(v: Self::Inner) -> shroom_pkt::PacketResult<Self> {
        match v {
            -2 => Ok(Self::Offline),
            -1 => Ok(Self::CashShop),
            // TODO channel max
            0..=20 => Ok(Self::Online(v as u16)),
            _ => Err(shroom_pkt::Error::InvalidEnumDiscriminant(v as usize)),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub enum PartyMemberFieldId {
    #[default]
    Offline,
    CashShop,
    Online(FieldId),
}

impl PacketTryWrapped for PartyMemberFieldId {
    type Inner = i32;

    fn packet_into_inner(&self) -> Self::Inner {
        match self {
            Self::Offline => -2,
            Self::CashShop => FieldId::NONE.0 as i32,
            Self::Online(field_id) => field_id.0 as i32,
        }
    }

    fn packet_try_from(v: Self::Inner) -> shroom_pkt::PacketResult<Self> {
        Ok(match v {
            -1 => Self::CashShop,
            // TODO use Mapid None
            999999999 => Self::Offline,
            0.. => {
                let map = FieldId(v as u32);
                if map == FieldId::NONE {
                    Self::Offline
                } else {
                    Self::Online(map)
                }
            }
            _ => return Err(shroom_pkt::Error::InvalidEnumDiscriminant(v as usize)),
        })
    }
}

#[derive(ShroomPacket, Debug, Default, Copy, Clone)]
pub struct PartyMemberTownPortal {
    pub town_id: u32,
    pub field_id: FieldId,
    pub skill_id: SkillId,
    pub pos: euclid::default::Vector2D<i32>,
}

pub struct PartyMember {
    pub id: CharacterId,
    pub name: NameStr,
    pub job: u32,
    pub status: PartyMemberStatus,
    pub field_id: PartyMemberFieldId,
    pub town_portal: PartyMemberTownPortal,
    pub pq_reward: u32,
    pub pq_reward_type: u32,
}

#[derive(ShroomPacket, Debug, Default)]
pub struct PartyData {
    pub members: [CharacterId; MAX_PARTY_MEMBERS],
    pub names: [NameStr; MAX_PARTY_MEMBERS],
    pub jobs: [u32; MAX_PARTY_MEMBERS],
    pub statuses: [PartyMemberStatus; MAX_PARTY_MEMBERS],
    pub leader: CharacterId,
    pub field_ids: [PartyMemberFieldId; MAX_PARTY_MEMBERS],
    pub town_portals: [PartyMemberTownPortal; MAX_PARTY_MEMBERS],
    pub pq_rewards: [u32; MAX_PARTY_MEMBERS],
    pub pq_reward_types: [u32; MAX_PARTY_MEMBERS],
    pub pq_reward_mob: MobId,
    pub pq_reward: bool,
}

impl PartyData {
    pub fn set_members(&mut self, member: &[PartyMember]) {
        for (i, m) in member.iter().enumerate() {
            self.members[i] = m.id;
            self.names[i] = m.name.clone();
            self.jobs[i] = m.job;
            self.statuses[i] = m.status;
            self.field_ids[i] = m.field_id;
            self.town_portals[i] = m.town_portal;
            self.pq_rewards[i] = m.pq_reward;
            self.pq_reward_types[i] = m.pq_reward_type;
        }
    }

    pub fn members(&self) -> impl Iterator<Item = PartyMember> + '_ {
        self.members
            .iter()
            .enumerate()
            .filter(|v| *v.1 != 0)
            .map(|(ix, id)| PartyMember {
                id: *id,
                name: self.names[ix].clone(),
                job: self.jobs[ix],
                status: self.statuses[ix],
                field_id: self.field_ids[ix],
                town_portal: self.town_portals[ix],
                pq_reward: self.pq_rewards[ix],
                pq_reward_type: self.pq_reward_types[ix],
            })
    }
}

#[derive(ShroomPacket, Debug)]
pub struct NewParty {
    pub party_id: PartyID,
    pub town_id: i32,
    pub field_id: FieldId,
    pub skill_id: SkillId,
    pub u1: u16,
    pub u2: u16,
}

#[derive(ShroomPacket, Debug)]
pub struct InviteParty {
    pub party_id: PartyID,
    pub inviter: String,
    pub level: u32,
    pub job: u32,
    pub u1: u8,
}

#[derive(ShroomPacket, Debug)]
pub struct ChangePartyLeader {
    pub party_id: PartyID,
    pub new_leader: CharacterId,
    pub disconnect: bool,
}

#[derive(ShroomPacket, Debug)]
pub struct MemberWithdraw {
    pub kicked: bool,
    pub name: String,
    pub party_data: PartyData,
}

#[derive(ShroomPacket, Debug)]
pub struct WithdrawParty {
    pub party_id: PartyID,
    pub char_id: CharacterId,
    /// If the member is not the leader this is set
    pub member_withdraw: ShroomOption8<MemberWithdraw>,
}

#[derive(ShroomPacket, Debug)]
pub struct UserMigrationParty {
    pub party_id: PartyID,
    pub party_data: PartyData,
}

#[derive(ShroomPacket, Debug)]
pub struct MemberChangeLevelJob {
    pub char_id: CharacterId,
    pub level: u32,
    pub job: u32,
}

#[derive(ShroomPacket, Debug)]
pub struct JoinParty {
    pub party_id: PartyID,
    pub name: NameStr,
    pub member_withdraw: ShroomOption8<MemberWithdraw>,
}

#[derive(ShroomPacketEnum, Debug)]
#[repr(u8)]
pub enum PartyResultResp {
    Invite(InviteParty) = 4,
    CreateNewParty(NewParty) = 8,

    Withdraw(WithdrawParty) = 0xC,
    WithdrawNotJoined(()) = 0xD,
    WithdrawUnknown(()) = 0xE,

    Join(JoinParty) = 0xF,
    Join1(JoinParty) = 0x10, //TODO correct?
    JoinAlreadyJoined(()) = 0x11,
    JoinAlreadyFull(()) = 0x12,
    JoinOverDesiredSize(()) = 0x13,
    JoinUnknownUser(()) = 0x14,
    JoinUnknown(()) = 0x15,

    InviteSent(String) = 0x16,
    InviteBlockedUser(()) = 0x17,
    InviteAlreadyInvited(()) = 0x18,
    InviteAlreadyInvitedByUser(()) = 0x19,

    InviteRejected(String) = 0x1A,

    ChangeLeader(ChangePartyLeader) = 0x1F,
    ChangeLeaderNotSameField(()) = 0x20,
    ChangeLeaderNoMemberyInSameField(()) = 0x21,
    ChangeLeaderNotSameChannel(()) = 0x22,
    ChangeLeaderUnknown(()) = 0x23,

    UserMigration(()) = 0x26,
    ChangeLevelJob(MemberChangeLevelJob) = 0x27,
}
