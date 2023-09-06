use clap::{Command, FromArgMatches, Parser, Subcommand};
use data::services::helper::pool::{
    drop::{Drop, DropTypeValue},
    user::User,
    Mob,
};
use proto95::id::{ItemId, MapId};

use crate::{Ctx, GameHandler};

#[derive(Parser, Debug)]
pub enum ReplCmd {
    Mob { id: Option<u32> },
    Mesos { amount: u32 },
    Item { id: Option<u32> },
    Chat { msg: String },
    FakeUser { id: u32 },
    Aggro,
    Dispose,
    Teleport,
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

impl GameHandler {
    pub async fn handle_repl_cmd(ctx: &mut Ctx, cmd: ReplCmd) -> anyhow::Result<Option<String>> {
        Ok(match cmd {
            ReplCmd::Mob { id } => {
                let mob = id.unwrap_or(1110100);
                let meta = ctx.services.meta.get_mob_data(mob).unwrap();
                ctx.field
                    .add_mob(Mob {
                        meta,
                        tmpl_id: mob,
                        pos: ctx.char().pos,
                        fh: ctx.char().fh,
                        origin_fh: None,
                        hp: meta.max_hp,
                        perc: 100,
                    })
                    .await?;
                None
            }
            ReplCmd::Mesos { amount } => {
                ctx.field.add_drop(Drop {
                    owner: proto95::game::drop::DropOwner::None,
                    pos: ctx.char().pos,
                    start_pos: ctx.char().pos,
                    value: DropTypeValue::Mesos(amount),
                    quantity: 1,
                })?;
                None
            }
            ReplCmd::Item { id } => {
                let item = id.map_or(ItemId::ADVANCED_MONSTER_CRYSTAL_1, ItemId);
                ctx.field.add_drop(Drop {
                    owner: proto95::game::drop::DropOwner::None,
                    pos: ctx.char().pos,
                    start_pos: ctx.char().pos,
                    value: DropTypeValue::Item(item),
                    quantity: 1,
                })?;
                None
            }
            ReplCmd::FakeUser { id } => {
                ctx.field.add_user(User {
                    avatar_data: ctx.session.char.get_avatar_data(),
                    char_id: id,
                    pos: ctx.char().pos,
                    fh: ctx.char().fh,
                })?;
                None
            }
            ReplCmd::Aggro => {
                ctx.field
                    .assign_mob_controller(ctx.session_handle.clone())?;
                None
            }
            ReplCmd::Dispose => {
                ctx.enable_char();
                None
            }
            ReplCmd::Chat { msg } => Some(msg),
            ReplCmd::Teleport => {
                GameHandler::join_field(ctx, MapId(1010000), None).await?;
                None
            }
        })
    }

    pub async fn handle_repl(ctx: &mut Ctx, s: &str) -> anyhow::Result<Option<String>> {
        Ok(match ctx.repl.match_cmd(s) {
            Err(_) => Some(ctx.repl.help()),
            Ok(cmd) => Self::handle_repl_cmd(ctx, cmd).await?,
        })
    }
}
