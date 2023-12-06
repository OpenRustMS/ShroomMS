use crate::services::helper::pool::{
    drop::{Drop, DropTypeValue},
    Mob,
};
use clap::{Command, FromArgMatches, Parser, Subcommand};
use proto95::{
    game::{
        shop::{OpenShopResp, ShopItem},
        user::pet::{PetActionResp, PetActivateError, PetActivateResp},
    },
    id::{job_id::JobId, FieldId, ItemId},
    shared::char::CharacterId,
};

use super::{
    field::FieldRoomCtxExt,
    game::{Ctx, GameSession},
};

#[derive(Parser, Debug)]
pub enum ReplCmd {
    Mob { id: Option<u32> },
    Pet { id: Option<u32> },
    Mesos { amount: u32 },
    Item { id: Option<u32> },
    Chat { msg: String },
    FakeUser { id: u32 },
    Aggro,
    Dispose,
    Teleport { id: Option<u32> },
    Sp { add: u32 },
    Job { id: u32 },
    TestSet,
    Level { level: u8 },
    MaxSkills,
    SpamDrop,
    StopSpamDrop,
    Dialog,
    Shop,
    Img,
}

pub struct GameRepl {
    cli: Command,
}

impl Default for GameRepl {
    fn default() -> Self {
        Self::new()
    }
}

impl GameRepl {
    pub fn new() -> Self {
        const PARSER_TEMPLATE: &str = "\
    {all-args}
";
        let cmd = Command::new("repl")
            .multicall(true)
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand_value_name("APPLET")
            .subcommand_help_heading("APPLETS")
            .help_template(PARSER_TEMPLATE);

        let cmd = ReplCmd::augment_subcommands(cmd);

        Self { cli: cmd }
    }
    pub fn match_cmd(&mut self, s: &str) -> anyhow::Result<ReplCmd> {
        let args = s.split_whitespace();
        let matches = self.cli.try_get_matches_from_mut(args)?;
        Ok(ReplCmd::from_arg_matches(&matches)?)
    }

    pub fn help(&mut self) -> String {
        self.cli.render_help().to_string()
    }
}

impl GameSession {
    pub fn handle_repl_cmd(
        &mut self,
        mut ctx: Ctx,
        cmd: ReplCmd,
    ) -> anyhow::Result<Option<String>> {
        let pos = self.session.char.pos;
        let fh = self.session.char.fh;
        let char_id = self.session.char.id as CharacterId;
        Ok(match cmd {
            ReplCmd::Mob { id } => {
                let mob = id.unwrap_or(1110100);
                let meta = self.services.game.meta.get_mob_data(mob).unwrap();
                ctx.ctx.add_mob(Mob {
                    meta,
                    tmpl_id: mob,
                    pos,
                    fh,
                    origin_fh: None,
                    hp: meta.max_hp,
                    perc: 100,
                    spawn_ix: None,
                })?;
                None
            }
            ReplCmd::Pet { id } => {
                let id = id.unwrap_or(5000008);
                ctx.reply(PetActivateResp {
                    user: char_id,
                    pet_id: 0,
                    succesful: true,
                    error: PetActivateError::None,
                    pet_tmpl_id: id,
                    peta_name: "Monkey".to_string(),
                    pet_locker_sn: 0,
                    pos,
                    move_action: 0,
                    fh,
                    name_tag: true,
                    chat_balloon: true,
                })?;
                ctx.reply(PetActionResp {
                    user: char_id,
                    pet_id: 0,
                    ty: 0,
                    action: 0,
                    chat: "Hello, I'm a monkey!".to_string(),
                    chat_balloon: true,
                })?;

                None
            }
            ReplCmd::Img => None,
            ReplCmd::Mesos { amount } => {
                ctx.ctx.add_drop(Drop {
                    owner: proto95::game::drop::DropOwner::None,
                    pos,
                    start_pos: pos,
                    value: DropTypeValue::Mesos(amount),
                    quantity: 1,
                })?;
                None
            }
            ReplCmd::Item { id } => {
                let item = id.map_or(ItemId::ADVANCED_MONSTER_CRYSTAL_1, ItemId);

                ctx.ctx.add_drop(Drop {
                    owner: proto95::game::drop::DropOwner::None,
                    pos,
                    start_pos: pos,
                    value: DropTypeValue::Item(item),
                    quantity: 1,
                })?;
                None
            }
            ReplCmd::FakeUser { id } => {
                log::info!("Adding fake user {id} not implemented yet");
                /*self.field.add_user(User {
                    avatar_data: self.session.char.get_avatar_data(),
                    char_id: id,
                    pos,
                    fh,
                })?;*/
                None
            }
            ReplCmd::Aggro => {
                ctx.ctx.assign_mob_controller(char_id as i32)?;
                None
            }
            ReplCmd::Dispose => {
                self.enable_char();
                None
            }
            ReplCmd::Chat { msg } => Some(msg),
            ReplCmd::Teleport { id } => {
                let field = id.map(FieldId).unwrap_or(FieldId::AMHERST);
                self.do_field_transfer(ctx, field, 0)?;
                None
            }
            ReplCmd::Sp { add } => {
                self.session.char.add_sp(add);
                None
            }
            ReplCmd::Job { id } => {
                let job = JobId::try_from(id as u16)?;
                self.session.char.change_job(job, true)?;
                None
            }
            ReplCmd::Level { level } => {
                *self.session.char.stats.level_mut() = level;
                None
            }
            ReplCmd::TestSet => {
                let item = &self.services.game.data.item;
                self.session.char.give_test_set(item)?;
                None
            }
            ReplCmd::MaxSkills => {
                self.session.char.skills.max_skills();
                None
            }
            ReplCmd::SpamDrop => {
                ctx.ctx.start_spam_drop(pos)?;
                None
            }
            ReplCmd::StopSpamDrop => {
                ctx.ctx.stop_spam_drop()?;
                None
            }
            ReplCmd::Dialog => {
                //TODO: self.start_script(ctx, npc_script_1000).await?;
                None
            }
            ReplCmd::Shop => {
                let npc_tmpl_id = 11000;
                let shop = self.services.game.meta.get_npc_shop(npc_tmpl_id).unwrap();
                ctx.reply(OpenShopResp {
                    npc_tmpl_id,
                    items: shop
                        .items
                        .iter()
                        .map(|item| ShopItem {
                            item_id: ItemId(item.item_id),
                            price: item.price,
                            quantity: 0,
                            discount_rate: 0,
                            token_item_id: ItemId(0),
                            token_price: 0,
                            item_period: 0,
                            level_limited: 0,
                            max_per_slot: u8::MAX as u16,
                        })
                        .collect(),
                })
                .unwrap();
                None
            }
        })
    }

    pub fn handle_repl(&mut self, ctx: Ctx, s: &str) -> anyhow::Result<Option<String>> {
        Ok(match self.repl.match_cmd(s) {
            Err(_) => Some(self.repl.help()),
            Ok(cmd) => self.handle_repl_cmd(ctx, cmd)?,
        })
    }
}
