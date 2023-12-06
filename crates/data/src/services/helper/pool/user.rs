use proto95::{
    game::user::{
        remote::{
            GuildMarkData, TamingMobData, UserEnterFieldResp, UserLeaveFieldResp, UserMoveResp,
            UserRemoteInitData,
        },
        secondary_stats::RemoteCharSecondaryStatPartial,
    },
    id::{job_id::JobId, ItemId},
    shared::{
        char::{AvatarData, CharacterId},
        movement::MovePath,
        Vec2,
    },
};
use shroom_pkt::ShroomIndexListZ;
use shroom_srv::srv::{room_set::RoomSessionSet, server_room::RoomSessionHandler};

use crate::services::data::character::CharacterID;

use super::{PoolItem, SimplePool};

#[derive(Debug)]
pub struct User {
    pub char_id: CharacterId,
    pub pos: Vec2,
    pub fh: u16,
    pub avatar_data: AvatarData,
}

impl PoolItem for User {
    type Id = CharacterId;

    type EnterPacket = UserEnterFieldResp;

    type LeavePacket = UserLeaveFieldResp;

    type LeaveParam = ();

    fn get_id(&self) -> Self::Id {
        self.char_id
    }

    fn get_enter_pkt(&self, _id: Self::Id) -> Self::EnterPacket {
        let secondary_stat = RemoteCharSecondaryStatPartial {
            //shadowpartner: Some(4111002).into(),
            darksight: Some(()).into(),
            curse: Some(1000).into(),
            ..Default::default()
        };

        let avatar = self.avatar_data.clone();

        UserEnterFieldResp {
            char_id: self.char_id,
            user_init_data: UserRemoteInitData {
                level: 30,
                name: self.char_id.to_string(),
                guild_name: "Eden".to_string(),
                guild_mark: GuildMarkData::default(),
                secondary_stat: secondary_stat.into(),
                avatar,
                driver_id: 0,
                passenger_id: 0,
                choco_count: 0,
                active_effect_item: ItemId(0),
                completed_set_item_id: ItemId(0),
                portable_chair: ItemId(0),
                pos: self.pos,
                fh: self.fh,
                show_admin_effects: false,
                pet_infos: ShroomIndexListZ::default(),
                taming_mob: TamingMobData::default(),
                mini_room: None.into(),
                ad_board: None.into(),
                couple: None.into(),
                friendship: None.into(),
                marriage: None.into(),
                load_flags: 0,
                new_year_cards: None.into(),
                phase: 0,
                defense_att: 0,
                defense_state: 0,
                job: JobId::Bandit,
                move_action: 0,
            },
        }
    }

    fn get_leave_pkt(&self, _id: Self::Id, _param: Self::LeaveParam) -> Self::LeavePacket {
        UserLeaveFieldResp {
            char_id: self.char_id,
        }
    }
}

impl SimplePool<User> {
    pub fn user_move<H: RoomSessionHandler<SessionId = CharacterID>>(
        &mut self,
        id: H::SessionId,
        move_path: MovePath,
        sessions: &mut RoomSessionSet<H>,
    ) -> anyhow::Result<()> {
        // Non-existing users are silent errors
        let Some(usr) = self.items.get_mut(&(id as u32)) else {
            return Ok(());
        };

        let last_pos_fh = move_path.get_last_pos_fh();
        if let Some((pos, fh)) = last_pos_fh {
            usr.fh = fh.unwrap_or(usr.fh);
            usr.pos = pos;
        }

        let pkt = UserMoveResp {
            char_id: id as u32,
            move_path,
        };

        sessions.broadcast_encode_filter(pkt, id)?;
        Ok(())
    }
}
