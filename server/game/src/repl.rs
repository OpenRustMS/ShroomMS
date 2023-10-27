use clap::{Command, FromArgMatches, Parser, Subcommand};
use data::{
    scripts::{npc_script_1000, ScriptHandle},
    services::{
        field::FieldMsg,
        helper::pool::{
            drop::{Drop, DropTypeValue},
            Mob,
        },
    },
};
use proto95::id::{job_id::JobId, ItemId, MapId};

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
    Teleport { id: Option<u32> },
    Sp { add: u32 },
    Job { id: u32 },
    TestSet,
    Level { level: u8 },
    MaxSkills,
    SpamDrop,
    StopSpamDrop,
    Dialog,
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
    pub async fn handle_repl_cmd(
        &mut self,
        ctx: &mut Ctx,
        cmd: ReplCmd,
    ) -> anyhow::Result<Option<String>> {
        Ok(match cmd {
            ReplCmd::Mob { id } => {
                let mob = id.unwrap_or(1110100);
                let meta = self.services.game.meta.get_mob_data(mob).unwrap();
                self.send_field_msg(FieldMsg::MobAdd(Mob {
                    meta,
                    tmpl_id: mob,
                    pos: self.char().pos,
                    fh: self.char().fh,
                    origin_fh: None,
                    hp: meta.max_hp,
                    perc: 100,
                    spawn_ix: None,
                }))
                .await?;
                None
            }
            ReplCmd::Mesos { amount } => {
                self.send_field_msg(FieldMsg::DropAdd(Drop {
                    owner: proto95::game::drop::DropOwner::None,
                    pos: self.char().pos,
                    start_pos: self.char().pos,
                    value: DropTypeValue::Mesos(amount),
                    quantity: 1,
                }))
                .await?;
                None
            }
            ReplCmd::Item { id } => {
                let item = id.map_or(ItemId::ADVANCED_MONSTER_CRYSTAL_1, ItemId);

                self.send_field_msg(FieldMsg::DropAdd(Drop {
                    owner: proto95::game::drop::DropOwner::None,
                    pos: self.char().pos,
                    start_pos: self.char().pos,
                    value: DropTypeValue::Item(item),
                    quantity: 1,
                }))
                .await?;
                None
            }
            ReplCmd::FakeUser { id } => {
                log::info!("Adding fake user {id} not implemented yet");
                /*self.field.add_user(User {
                    avatar_data: self.session.char.get_avatar_data(),
                    char_id: id,
                    pos: self.char().pos,
                    fh: self.char().fh,
                })?;*/
                None
            }
            ReplCmd::Aggro => {
                self.send_field_msg(FieldMsg::MobAssignController(self.char().id))
                    .await?;
                None
            }
            ReplCmd::Dispose => {
                self.enable_char();
                None
            }
            ReplCmd::Chat { msg } => Some(msg),
            ReplCmd::Teleport { id } => {
                let map = id.unwrap_or(1010000);
                self.join_field(ctx, MapId(map), None).await?;
                None
            }
            ReplCmd::Sp { add } => {
                self.char_mut().add_sp(add);
                None
            }
            ReplCmd::Job { id } => {
                let job = JobId::try_from(id as u16)?;
                self.char_mut().change_job(job, true)?;
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
                self.char_mut().skills.max_skills();
                None
            }
            ReplCmd::SpamDrop => {
                self.send_field_msg(FieldMsg::StartSpamDrop(self.char().pos))
                    .await?;
                None
            }
            ReplCmd::StopSpamDrop => {
                self.send_field_msg(FieldMsg::StopSpamDrop).await?;
                None
            }
            ReplCmd::Dialog => {
                // Get npc
                self.npc_script_handle = Some(ScriptHandle::from_script_fn(
                    npc_script_1000,
                    self.char_mut(),
                ));

                self.poll_npc(ctx).await?;
                None
            }
        })
    }

    pub async fn handle_repl(&mut self, ctx: &mut Ctx, s: &str) -> anyhow::Result<Option<String>> {
        Ok(match self.repl.match_cmd(s) {
            Err(_) => Some(self.repl.help()),
            Ok(cmd) => self.handle_repl_cmd(ctx, cmd).await?,
        })
    }
}
