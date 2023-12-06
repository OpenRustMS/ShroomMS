use std::{
    net::IpAddr,
    ops::{Deref, DerefMut, Neg},
};

use either::Either;
use meta::FieldMeta;
use proto95::{
    game::{
        chat::{ChatMsgReq, UserChatMsgResp},
        field::{
            CrcSeed, FieldCharData, FieldTransferData, LogoutGiftConfig, NotificationList,
            SetFieldResp,
        },
        life::{
            mob::{MobMoveCtrlAckResp, MobMoveReq},
            reactor::ReactorHitReq,
        },
        user::{
            char::{CharDataAll, CharDataFlags},
            secondary_stats::LocalSecondaryStatResetResp,
            ChangeSkillRecordResp, UserDropMoneyReq, UserDropPickUpReq, UserMeleeAttackReq,
            UserMoveReq, UserSkillUpReq, UserSkillUseReq, UserStatChangeReq, UserTransferFieldReq,
        },
        BroadcastMessageResp, UserPortalScriptReq, friend::{FriendResultResp, FriendList}, keymaps::FuncKeyMapInitResp, ClaimSvrStatusChangedResp, CtxSetGenderResp,
    },
    id::FieldId,
    login::{
        world::{ChannelId, WorldId},
        ClientKey,
    },
    recv_opcodes::RecvOpcodes,
    shared::{
        char::{
            CharDataEquipped, CharDataHeader, CharDataStat, CharStatChangedResp, SkillInfo,
            SocialRecords, TeleportRockInfo,
        },
        inventory::{InvChangeSlotPosReq, InventoryOperationsResp},
        item::Item,
    },
};
use shroom_pkt::{
    partial::PartialFlag, PacketWriter, ShroomExpirationTime, ShroomIndexListZ16,
    ShroomIndexListZ8, ShroomList16, ShroomPacket, ShroomTime,
};
use shroom_srv::srv::{
    server_room::{RoomCtx, RoomSessionHandler},
    server_socket::ServerSocketHandle, server_system::SystemMsg,
};

use crate::scripts::NpcScriptHandle;

use super::{
    data::character::CharacterID,
    field::{FieldHandler, FieldRoomCtxExt},
    helper::pool::{
        drop::{DropLeaveParam, DropTypeValue},
        Drop,
    },
    session::shroom_session_manager::OwnedShroomGameSession,
    SharedServices, repl::GameRepl,
};

pub struct GameSession {
    pub services: SharedServices,
    pub session: OwnedShroomGameSession,
    pub addr: IpAddr,
    pub channel_id: ChannelId,
    pub world_id: WorldId,
    pub client_key: ClientKey,
    //pub repl: GameRepl,
    pub script_handle: NpcScriptHandle,
    pub field_id: FieldId,
    pub field_meta: FieldMeta,
    pub repl: GameRepl
}

pub struct Ctx<'ctx, 'ctxx, 'sck> {
    pub sck: &'sck mut ServerSocketHandle,
    pub ctx: &'ctx mut RoomCtx<'ctxx, GameSession>,
}

impl<'ctx, 'ctxx, 'sck> Deref for Ctx<'ctx, 'ctxx, 'sck> {
    type Target = RoomCtx<'ctxx, GameSession>;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl<'ctx, 'ctxx, 'sck> DerefMut for Ctx<'ctx, 'ctxx, 'sck> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ctx
    }
}

impl<'ctx, 'ctxx, 'sck> Ctx<'ctx, 'ctxx, 'sck> {
    pub fn reply(&mut self, pkt: impl ShroomPacket) -> anyhow::Result<()> {
        let mut pw = PacketWriter::default();
        pkt.encode_packet(&mut pw)?;
        self.sck.send(pw.into_packet())?;
        Ok(())
    }
}

macro_rules! op_handler {
    ($op:ident, $this:ident, $ctx:ident, $pr:ident, $($ty:ty => $handler:ident),*) => {
        match $op {
            $(
                <$ty>::OPCODE => $this.$handler($ctx, <$ty>::decode_packet(&mut $pr)?),
            )*
            _ => {
                log::warn!("Unhandled packet: {:?}", $op);
                Ok(())
            }
        }
    };
}

impl RoomSessionHandler for GameSession {
    type RoomHandler = FieldHandler;
    type Msg = ();


    type SessionId = CharacterID;

    type RoomId = FieldId;

    fn session_id(&self) -> Self::SessionId {
        self.session.char.id
    }

    fn room_id(&self) -> Self::RoomId {
        self.field_id
    }


    fn on_enter_room(
        &mut self,
        sck: &mut shroom_srv::srv::server_socket::ServerSocketHandle,
        ctx: &mut RoomCtx<'_, Self>,
    ) -> anyhow::Result<()> {
        log::info!("Entering room...");
        self.init_char(Ctx { sck, ctx })?;
        Ok(())
    }

    fn on_update(
        &mut self,
        sck: &mut shroom_srv::srv::server_socket::ServerSocketHandle,
        ctx: &mut RoomCtx<'_, Self>,
    ) -> anyhow::Result<()> {
        self.update_char_stats(Ctx { sck, ctx })?;
        Ok(())
    }

    fn on_msg(
        &mut self,
        _sck: &mut shroom_srv::srv::server_socket::ServerSocketHandle,
        _ctx: &mut RoomCtx<'_, Self>,
        _msg: Self::Msg,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_packet(
        &mut self,
        sck: &mut shroom_srv::srv::server_socket::ServerSocketHandle,
        ctx: &mut RoomCtx<Self>,
        packet: shroom_srv::srv::server_socket::Packet,
    ) -> anyhow::Result<()> {
        let op: RecvOpcodes = packet.into_reader().read_opcode()?;
        let mut pr = packet.into_reader();

        let ctx_ = Ctx { sck, ctx };
        let res = op_handler!(
            op,
            self,
            ctx_,
            pr,
            UserSkillUseReq =>  handle_use_skill,
            ReactorHitReq => handle_reactor_hit,
            MobMoveReq => handle_mob_move,
            UserPortalScriptReq => handle_portal_script,
            UserMoveReq => handle_user_move,
            ChatMsgReq => handle_chat_msg,
            UserDropMoneyReq => handle_drop_money,
            UserDropPickUpReq => handle_drop_pick_up,
            UserMeleeAttackReq => handle_melee_attack,
            UserSkillUpReq => handle_skill_up,
            InvChangeSlotPosReq => handle_inv_change_slot,
            UserStatChangeReq => handle_stat_change,
            UserTransferFieldReq => handle_field_transfer
        );
        res?;

        self.update_char_stats(Ctx { sck, ctx })?;
        Ok(())
    }

    fn on_switch_room(
        ctx: &mut RoomCtx<'_, Self>,
        session: shroom_srv::srv::room_set::ServerSessionData<Self>,
        new_room: Self::RoomId
    ) -> anyhow::Result<()> {
        ctx.room_ctx.tx.send(SystemMsg::ChangeRoom (
            session,
            new_room,
        ))?;

        Ok(())
    }

}

impl GameSession {
    fn char_id(&self) -> CharacterID {
        self.session.char.id
    }

    pub fn enable_char(&mut self) {
        self.session.char.unlock_char()
    }

    fn update_char_stats(&mut self, mut ctx: Ctx) -> anyhow::Result<()> {
        if let Some(partial) = self.session.char.get_stats_update() {
            ctx.reply(CharStatChangedResp {
                excl: true, //TODO handle this
                stats: PartialFlag {
                    hdr: (),
                    data: partial,
                },
                secondary_stat: false,
                battle_recovery: false,
            })?;
        }

        if let Some(ops) = self.session.char.get_inv_op_updates() {
            ctx.reply(InventoryOperationsResp {
                reset_excl: true,
                operations: ops.into(),
                secondary_stat_changed: false,
            })?;
        }

        if let Some(skills) = self.session.char.skills.get_updates() {
            ctx.reply(ChangeSkillRecordResp {
                reset_excl: true,
                skill_records: skills.into(),
                updated_secondary_stat: false,
            })?;
        }

        if let Some(skill_cd) = self.session.char.skills.get_cooldown_updates() {
            for cd in skill_cd {
                ctx.reply(cd)?;
            }
        }

        if let Some(secondary_stat) = self.session.char.get_secondary_stats_update() {
            ctx.reply(secondary_stat)?;
        }

        // Handle timer events

        if let Some(flags) = self.session.char.handle_timer_events() {
            ctx.reply(LocalSecondaryStatResetResp {
                flags,
                movement_affecting: true,
            })?;
        }

        Ok(())
    }

    fn handle_use_skill(&mut self, mut ctx: Ctx, req: UserSkillUseReq) -> anyhow::Result<()> {
        self.session.char.use_skill(req.skill_id)?;

        if let Some(summon) = self.session.char.do_summon.take() {
            ctx.summon_spawn(
                summon.char_id as i32,
                summon.char_level,
                summon.skill_id,
                summon.skill_level,
                summon.pos,
                summon.fh,
            )?;
        }

        Ok(())
    }

    fn handle_reactor_hit(&mut self, mut ctx: Ctx, req: ReactorHitReq) -> anyhow::Result<()> {
        ctx.attack_reactor(req.id, self.char_id())?;
        Ok(())
    }

    fn handle_mob_move(&mut self, mut ctx: Ctx, req: MobMoveReq) -> anyhow::Result<()> {
        let ctrl_sn = req.ctrl_sn;
        let id = req.id;
        ctx.update_mob_pos(req, self.char_id())?;
        ctx.reply(MobMoveCtrlAckResp {
            id,
            ctrl_sn,
            next_atk_possible: false,
            mp: 0,
            skill_id: 0,
            slv: 0,
        })?;
        Ok(())
    }

    fn handle_portal_script(&mut self, _ctx: Ctx, _req: UserPortalScriptReq) -> anyhow::Result<()> {
        self.enable_char();
        Ok(())
    }

    fn handle_user_move(&mut self, mut ctx: Ctx, req: UserMoveReq) -> anyhow::Result<()> {
        let chr = &mut self.session.char;
        chr.pos = req.move_path.pos;
        let last = req.move_path.get_last_pos_fh();

        if let Some((pos, fh)) = last {
            chr.pos = pos;
            chr.fh = fh.unwrap_or(chr.fh);
        }

        ctx.update_user_pos(req.move_path, self.char_id())?;
        Ok(())
    }

    fn handle_drop_money(&mut self, mut ctx: Ctx, req: UserDropMoneyReq) -> anyhow::Result<()> {
        let can_drop = self.session.char.update_mesos((req.money as i32).neg());
        if can_drop {
            let char = &self.session.char;
            ctx.add_drop(Drop {
                owner: proto95::game::drop::DropOwner::User(char.id as u32),
                pos: char.pos,
                start_pos: char.pos,
                value: DropTypeValue::Mesos(req.money),
                quantity: 1,
            })?;
        }

        self.enable_char();

        Ok(())
    }

    fn handle_drop_pick_up(&mut self, mut ctx: Ctx, req: UserDropPickUpReq) -> anyhow::Result<()> {
        // Try to claim the drop
        let Ok(drop) = ctx.remove_drop(
            req.drop_id,
            DropLeaveParam::UserPickup(self.session.char.id as u32),
        ) else {
            return Ok(());
        };

        let char = &mut self.session.char;
        match drop.value {
            DropTypeValue::Mesos(money) => {
                char.update_mesos(money as i32);
            }
            DropTypeValue::Item(item_id) => {
                let inv_ty = item_id.get_inv_type()?;
                if !inv_ty.is_stack() {
                    char.add_equip_item(item_id)?;
                } else {
                    char.add_stack_item(inv_ty, item_id, drop.quantity)?;
                };
            }
        }
        Ok(())
    }

    fn handle_melee_attack(&mut self, mut ctx: Ctx, req: UserMeleeAttackReq) -> anyhow::Result<()> {
        let char = &mut self.session.char;
        for target in req.targets {
            let dmg = target.hits.iter().sum::<u32>();
            ctx.attack_mob(target.mob_id, dmg, char.id)?;
        }
        char.handle_attack()?;
        Ok(())
    }

    fn handle_skill_up(&mut self, _ctx: Ctx, req: UserSkillUpReq) -> anyhow::Result<()> {
        self.session.char.skill_up(req.skill_id)?;
        Ok(())
    }

    fn handle_inv_change_slot(
        &mut self,
        mut ctx: Ctx,
        req: InvChangeSlotPosReq,
    ) -> anyhow::Result<()> {
        let count = (req.count != u16::MAX).then_some(req.count as usize);
        let drop = req.to == 0;
        let from = (req.inv_type, req.from).try_into()?;
        // Check for drop
        if drop {
            let item = self.session.char.inventory.drop_item(from, count)?;
            // TODO handle persistent equip items
            let drop = match item {
                Either::Left(eq) => Drop {
                    owner: proto95::game::drop::DropOwner::User(self.session.char.id as u32),
                    pos: self.session.char.pos,
                    start_pos: self.session.char.pos,
                    value: DropTypeValue::Item(eq.item_id),
                    quantity: 1,
                },
                Either::Right(stack) => Drop {
                    owner: proto95::game::drop::DropOwner::User(self.session.char.id as u32),
                    pos: self.session.char.pos,
                    start_pos: self.session.char.pos,
                    value: DropTypeValue::Item(stack.item_id),
                    quantity: stack.quantity as usize,
                },
            };

            ctx.add_drop(drop)?;
        } else {
            let to = (req.inv_type, req.to).try_into()?;
            self.session.char.inventory.move_item(from, to, count)?;
        }

        self.enable_char();
        Ok(())
    }

    pub fn do_field_transfer(&mut self, mut ctx: Ctx, field: FieldId, spawn: u8) -> anyhow::Result<()> {
        let meta = &self.services.game.meta;
        ctx.room_sessions.register_transition(self.char_id(), field);
        self.session.char.transfer_map(field, spawn);
        self.field_id = field;
        self.field_meta = meta.get_field_data(field).unwrap();
        log::info!("Transfering map");
        Ok(())
    }

    fn handle_field_transfer(
        &mut self,
        ctx: Ctx,
        req: UserTransferFieldReq,
    ) -> anyhow::Result<()> {
        let meta = &self.services.game.meta;
        let field_meta = &self.field_meta;
        let (field, spawn) = if self.session.char.is_dead() {
            self.session.char.respawn();
            // TODO the portal is not correct
            meta.get_return_field_spawn(field_meta).unwrap_or_else(|| {
                (
                    self.field_id,
                    field_meta.get_first_portal_id().unwrap(),
                )
            })
        } else {
            meta.get_portal_map_spawn(self.field_id, self.field_meta, &req.portal)
                .ok_or_else(|| anyhow::format_err!("Invalid portal"))?
        };

        self.do_field_transfer(ctx, field, spawn)?;
        Ok(())
    }

    fn handle_stat_change(&mut self, _ctx: Ctx, req: UserStatChangeReq) -> anyhow::Result<()> {
        let char = &mut self.session.char;
        char.stats.update_hp(req.hp as i16);
        char.stats.update_mp(req.mp as i16);
        Ok(())
    }

    fn handle_chat_msg(&mut self, mut ctx: Ctx, req: ChatMsgReq) -> anyhow::Result<()> {
        let admin = false;
        if let Some(s) = req.msg.strip_prefix('@') {
            log::info!("repl: {}", s);
            let _repl_resp = self.handle_repl(ctx, s)?;
            //let Some(msg) = repl_resp else { return Ok(()) };
            /*let resp = UserChatMsgResp {
                char: self.session.char.id as u32,
                is_admin: admin,
                msg,
                only_balloon: false,
            };

            ctx.reply(resp)?;*/
        } else {
            ctx.chat(UserChatMsgResp {
                char: self.session.char.id as u32,
                is_admin: admin,
                msg: req.msg,
                only_balloon: req.only_balloon,
            })?;
        };
        Ok(())
    }

    fn set_field(&self, char_data: bool, sp: u8) -> SetFieldResp {
        let field_data = if char_data {
            let char = &self.session.char;

            let equipped: ShroomIndexListZ16<Item> = self
                .session
                .char
                .inventory
                .invs
                .equipped
                .item_with_slots()
                .map(|(slot, item)| (slot as u16, Item::Equip(item.0.item.as_ref().into())))
                .collect();

            let equip: ShroomIndexListZ16<Item> = self
                .session
                .char
                .inventory
                .invs
                .equip
                .item_with_slots()
                .map(|(slot, item)| (slot as u16 + 1, Item::Equip(item.item.as_ref().into())))
                .collect();

            let etc: ShroomIndexListZ8<Item> = self
                .session
                .char
                .inventory
                .invs
                .etc
                .item_with_slots()
                .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.into())))
                .collect();

            let setup: ShroomIndexListZ8<Item> = self
                .session
                .char
                .inventory
                .invs
                .misc
                .item_with_slots()
                .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.into())))
                .collect();

            let cash: ShroomIndexListZ8<Item> = self
                .session
                .char
                .inventory
                .invs
                .cash
                .item_with_slots()
                .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.into())))
                .collect();

            let use_: ShroomIndexListZ8<Item> = self
                .session
                .char
                .inventory
                .invs
                .use_
                .item_with_slots()
                .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.into())))
                .collect();

            let char_equipped = CharDataEquipped {
                equipped,
                equip,
                ..Default::default()
            };

            let skillrecords: ShroomList16<SkillInfo> =
                self.session.char.skills.get_skill_info().into();

            let char_data = CharDataAll {
                stat: CharDataStat {
                    stat: char.get_all_stats(),
                    friend_max: 30,
                    linked_character: None.into(),
                },
                money: char.money(),
                invsize: char.get_inv_slots(),
                equipextslotexpiration: ShroomExpirationTime::never(),
                equipped: char_equipped,
                useinv: use_,
                setupinv: setup,
                etcinv: etc,
                cashinv: cash,
                skillrecords,
                skllcooltime: ShroomList16::default(),
                quests: ShroomList16::default(),
                questscompleted: ShroomList16::default(),
                minigamerecords: ShroomList16::default(),
                socialrecords: SocialRecords::default(),
                teleportrockinfo: TeleportRockInfo::default(),
                newyearcards: ShroomList16::default(),
                questrecordsexpired: ShroomList16::default(),
                questcompleteold: ShroomList16::default(),
                visitorquestloginfo: ShroomList16::default(),
            };

            Either::Left(FieldCharData {
                seed: CrcSeed {
                    s1: 1,
                    s2: 2,
                    s3: 3,
                },
                logout_gift_config: LogoutGiftConfig {
                    predict_quit: 0,
                    gift_commodity_id: [0; 3],
                },
                char_data_hdr: CharDataHeader {
                    combat_orders: 0,
                    extra_data: None.into(),
                },
                char_data,
                char_data_flags: CharDataFlags::all(),
            })
        } else {
            Either::Right(FieldTransferData {
                revive: false,
                map: self.session.char.map_id,
                portal: sp,
                hp: self.session.char.stats.hp.value as u32,
                chase_target_pos: None.into(),
            })
        };

        SetFieldResp {
            client_option: ShroomList16::default(),
            channel_id: self.channel_id as u32,
            has_char_data: field_data.is_left(),
            char_data: field_data.into(),
            notifications: NotificationList::default(),
            old_driver_id: 0,
            unknown_flag_1: 0,
            server_time: ShroomTime::now(),
        }
    }

    pub fn init_char(&mut self, mut ctx: Ctx) -> anyhow::Result<()> {
        ctx.reply(self.set_field(true, 0))?;

        ctx.reply(FriendResultResp::Reset3(FriendList::empty()))?;
        ctx.reply(FuncKeyMapInitResp::default_map())?;
        ctx.reply(ClaimSvrStatusChangedResp { connected: true })?;
        ctx.reply(CtxSetGenderResp {
            gender: self.session.char.gender,
        })?;

        ctx.reply(BroadcastMessageResp::PinkMessage("Hello".to_string()))?;
        ctx.ctx.enter_field(self.char_id(), self.session.char.get_avatar_data(), &mut ctx.sck)?;

        self.session.char.unlock_char();

        Ok(())
    }
}
