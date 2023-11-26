use async_trait::async_trait;
use proto95::{id::ItemId, shared::char::Money};

pub mod npc;
pub mod poll_state;

pub enum NpcInput {
    Next,
    Prev,
    Selection(usize),
    InputTxt(String),
    InputNum(u32),
    YesNo(bool),
    AvatarSelection(usize),
    PetSelection(usize),
    SliderValue(u32),
    End,
}

#[async_trait]
pub trait SessionCtx {
    fn level(&self) -> u8;
    fn set_level(&mut self, level: u8) -> anyhow::Result<()>;

    fn give_item(&self, item: ItemId, count: usize) -> anyhow::Result<bool>;
    fn give_multiple_items(&mut self, items: &[(ItemId, usize)]) -> anyhow::Result<bool>;

    fn money(&self) -> Money;
    fn set_money(&mut self, money: Money) -> anyhow::Result<()>;
    fn update_money(&mut self, delta: i32) -> anyhow::Result<()>;

    async fn say(&self, msg: &str) -> anyhow::Result<()>;

    async fn say_next(&self, msg: &str) -> anyhow::Result<()>;
    async fn ask_yes_no(&self, msg: &str) -> anyhow::Result<bool>;
}

#[async_trait]
pub trait NpcScript {
    type Ctx: SessionCtx;

    fn is_finished(&self) -> bool;
    async fn run(&mut self, ctx: &mut Self::Ctx, input: NpcInput) -> anyhow::Result<()>;
}

pub struct NpcHandle<S, Ctx> {
    script: S,
    ctx: Ctx,
    finished: bool,
}

impl<S, Ctx> NpcHandle<S, Ctx>
where
    S: NpcScript<Ctx = Ctx>,
{
    pub fn new(script: S, ctx: Ctx) -> Self {
        Self {
            script,
            ctx,
            finished: false,
        }
    }

    pub async fn run(&mut self, input: NpcInput) -> anyhow::Result<()> {
        if self.finished {
            return Ok(());
        }
        self.script.run(&mut self.ctx, input).await?;
        self.finished = self.script.is_finished();
        Ok(())
    }
}
