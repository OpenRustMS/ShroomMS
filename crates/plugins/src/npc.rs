use std::{future::Future, pin::Pin};

use proto95::{
    fmt::{ShroomDisplay, ShroomMenuList},
    game::script::{
        AskMsg, AskNumberMsg, AskTextMsg, MsgParamFlags, OptionAnswer, SayMsg, ScriptAnswerReq,
        ScriptMessage,
    },
    id::{ItemId, SkillId},
    shared::char::Money,
};

use crate::poll_state::{StateHandle, StateRef};

#[derive(Debug, PartialEq, Eq)]
pub enum NpcAction {
    Start,
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

impl From<ScriptAnswerReq> for NpcAction {
    fn from(req: ScriptAnswerReq) -> Self {
        match req {
            ScriptAnswerReq::PrevNext(OptionAnswer(Some(true))) => Self::Next,
            ScriptAnswerReq::PrevNext(OptionAnswer(Some(false))) => Self::Prev,
            ScriptAnswerReq::ImgNext(OptionAnswer(Some(true))) => Self::Next,
            ScriptAnswerReq::YesNo(OptionAnswer(Some(v))) => Self::YesNo(v),
            ScriptAnswerReq::InputText(v) if v.is_some() => {
                Self::InputTxt(v.as_ref().unwrap().clone())
            }
            ScriptAnswerReq::InputNumber(v) => Self::InputNum(v),
            ScriptAnswerReq::InputSelection(v) if v.is_some() => {
                NpcAction::Selection(v.unwrap() as usize)
            }
            ScriptAnswerReq::AvatarSelection(v) if v.is_some() => {
                NpcAction::AvatarSelection(v.unwrap() as usize)
            }
            ScriptAnswerReq::AvatarMembershipSelection(v) if v.is_some() => {
                NpcAction::AvatarSelection(v.unwrap() as usize)
            }
            ScriptAnswerReq::PetSelection(v) if v.is_some() => {
                NpcAction::PetSelection(v.unwrap() as usize)
            }
            ScriptAnswerReq::InputBoxText(v) if v.is_some() => {
                Self::InputTxt(v.as_ref().unwrap().clone())
            }
            ScriptAnswerReq::InputSliderValue(v) if v.is_some() => Self::SliderValue(v.unwrap()),
            _ => Self::End,
        }
    }
}

pub trait CharCtx {
    fn send_msg(&mut self, msg: ScriptMessage);

    fn set_money(&mut self, money: Money);
    fn update_money(&mut self, delta: i32) -> bool;
    fn get_money(&self) -> Money;

    fn get_level(&self) -> u8;
    fn set_level(&mut self, level: u8);

    fn has_item(&self, id: ItemId) -> bool;
    fn try_add_item(&mut self, id: ItemId, quantity: usize) -> bool;
    fn try_add_items(&mut self, items: &[(ItemId, usize)]) -> bool;
}

impl<Ctx: CharCtx> StateRef<Ctx, NpcAction> {
    pub async fn wait_for_next(&mut self) -> anyhow::Result<()> {
        let action = self.next_input().await?;
        if action != NpcAction::Next {
            anyhow::bail!("Expected next action");
        }

        Ok(())
    }

    pub async fn ask_text(
        &mut self,
        text: impl ShroomDisplay,
        min: u32,
        max: u32,
        default: String,
    ) -> anyhow::Result<String> {
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskText(AskTextMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
                default_txt: default,
                min: min as u16,
                max: max as u16,
            }))
        });

        match self.next_input().await? {
            NpcAction::InputTxt(v) if v.len() < min as usize || v.len() > max as usize => {
                anyhow::bail!("Invalid text: {v}, min {min}, max {max}")
            }
            NpcAction::InputTxt(v) => Ok(v),
            _ => anyhow::bail!("Expected InputTxt action"),
        }
    }

    pub async fn ask_number(
        &mut self,
        text: impl ShroomDisplay,
        min: u32,
        max: u32,
        default: u32,
    ) -> anyhow::Result<u32> {
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskNumber(AskNumberMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
                default_number: default,
                min,
                max,
            }))
        });

        match self.next_input().await? {
            NpcAction::InputNum(v) if v < min || v > max => {
                anyhow::bail!("Invalid number: {v}, min {min}, max {max}")
            }
            NpcAction::InputNum(v) => Ok(v),
            _ => anyhow::bail!("Expected InputNum action"),
        }
    }

    pub async fn ask_selection<T: ShroomDisplay>(
        &mut self,
        text: &str,
        items: ShroomMenuList<T>,
    ) -> anyhow::Result<usize> {
        let msg = format!("{}\n{}", text, items).to_shroom_string();
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskMenu(AskMsg {
                param: MsgParamFlags::empty(),
                msg,
            }))
        });

        Ok(match self.next_input().await? {
            NpcAction::Selection(v) if v >= items.len() => {
                anyhow::bail!("Invalid selection: {v}, max {}", items.len())
            }
            NpcAction::Selection(v) => v,
            _ => anyhow::bail!("Expected YesNo action"),
        })
    }

    pub async fn ask_yes_no(&mut self, text: impl ShroomDisplay) -> anyhow::Result<bool> {
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::AskYesNo(AskMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
            }))
        });

        match self.next_input().await? {
            NpcAction::YesNo(b) => Ok(b),
            _ => anyhow::bail!("Expected YesNo action"),
        }
    }

    pub async fn say(&mut self, text: impl ShroomDisplay, last: bool) -> anyhow::Result<()> {
        self.with_mut(|c| {
            c.send_msg(ScriptMessage::Say(SayMsg {
                param: MsgParamFlags::empty(),
                txt: text.to_shroom_string(),
                has_prev: true,
                has_next: !last,
                speaker_tmpl_id: None.into(),
            }))
        });
        Ok(())
    }

    pub async fn say_next(&mut self, text: impl ShroomDisplay) -> anyhow::Result<()> {
        self.say(text, false).await?;
        self.wait_for_next().await?;
        Ok(())
    }

    pub async fn say_end(&mut self, text: impl ShroomDisplay) -> anyhow::Result<()> {
        self.say(text, true).await?;
        self.wait_for_next().await?;
        Ok(())
    }

    pub fn char_level(&self) -> u8 {
        self.with(|c| c.get_level())
    }

    pub fn set_char_level(&mut self, lvl: u8) {
        self.with_mut(|c| c.set_level(lvl));
    }

    pub fn has_item(&self, id: ItemId) -> bool {
        self.with(|c| c.has_item(id))
    }

    pub fn try_update_money(&mut self, money: i32) -> bool {
        self.with_mut(|c| c.update_money(money))
    }

    pub fn try_give_items(&mut self, ids: &[(ItemId, usize)]) -> bool {
        self.with_mut(|c| c.try_add_items(ids))
    }
}

pub async fn npc_script_1000<Ctx: CharCtx>(
    mut api: StateRef<Ctx, NpcAction>,
) -> anyhow::Result<()> {
    api.say_next("Hello I'm a NPC").await?;

    let item = ItemId::ALL_CURE_POTION;
    if api.ask_yes_no("Do you want starter items").await? {
        api.say_next(format!(
            "I'll give you 3 {item:+} and {}",
            "500 mesos".blue()
        ))
        .await?;
        api.try_give_items(&[(item, 3)]);
        api.try_update_money(500);
    }

    let dispel = SkillId(2311001);

    api.say_next(format!(
        "Don't forget to use your spells like: {0} ## {0:+}",
        dispel,
    ))
    .await?;

    let sel = api
        .ask_selection("Language?", vec!["English", "Spanish", "German"].into())
        .await?;
    let bye = match sel {
        0 => "Goodbye!",
        1 => "Adios!",
        2 => "Auf Wiedersehen!",
        _ => unreachable!(),
    };
    api.say_end(bye).await?;
    Ok(())
}

pub type BoxedFuture = Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + Sync>>;

pub struct NpcScriptHandle<Ctx> {
    script: Option<StateHandle<BoxedFuture, Ctx, NpcAction>>,
}

impl<Ctx> Default for NpcScriptHandle<Ctx> {
    fn default() -> Self {
        Self { script: None }
    }
}

impl<Ctx: CharCtx> NpcScriptHandle<Ctx> {
    pub async fn start_fn<F>(&mut self, f: F, ctx: &mut Ctx) -> anyhow::Result<()>
    where
        F: FnOnce(StateRef<Ctx, NpcAction>) -> BoxedFuture,
    {
        self.script = Some(StateHandle::from_fn(f));
        self.script.as_mut().unwrap().run_once(ctx).await;
        Ok(())
    }

    pub async fn run(&mut self, input: NpcAction, ctx: &mut Ctx) -> anyhow::Result<()> {
        if let Some(script) = self.script.as_mut() {
            if let Some(res) = script.transition(input, ctx).await {
                self.script = None;
                return res;
            }
        }

        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.script.as_ref().map_or(true, |s| s.is_finished())
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[derive(Debug, Default)]
    pub struct FakeChar {
        pub level: u8,
        pub money: Money,
        pub items: Vec<ItemId>,
    }

    impl CharCtx for FakeChar {
        fn send_msg(&mut self, msg: ScriptMessage) {
            println!("send_msg: {:?}", msg);
        }

        fn set_money(&mut self, money: Money) {
            self.money = money;
        }

        fn update_money(&mut self, delta: i32) -> bool {
            match self.money.checked_add_signed(delta) {
                Some(money) => {
                    self.money = money;
                    true
                }
                None => false,
            }
        }

        fn get_money(&self) -> Money {
            self.money
        }

        fn get_level(&self) -> u8 {
            self.level
        }

        fn set_level(&mut self, level: u8) {
            self.level = level;
        }

        fn has_item(&self, id: ItemId) -> bool {
            self.items.contains(&id)
        }

        fn try_add_item(&mut self, id: ItemId, quantity: usize) -> bool {
            if self.items.len() + quantity > 100 {
                return false;
            }
            self.items.extend(iter::repeat(id).take(quantity));
            true
        }

        fn try_add_items(&mut self, items: &[(ItemId, usize)]) -> bool {
            for (id, quantity) in items {
                if !self.try_add_item(*id, *quantity) {
                    return false;
                }
            }
            true
        }
    }

    #[tokio::test]
    async fn simple_script() {
        let mut fake_char = FakeChar::default();
        let mut npc = NpcScriptHandle::default();
        npc.start_fn(|api| Box::pin(async move { npc_script_1000(api).await }), &mut fake_char).await.unwrap();

        npc.run(NpcAction::Next, &mut fake_char).await.unwrap();
        npc.run(NpcAction::YesNo(false), &mut fake_char)
            .await
            .unwrap();
        npc.run(NpcAction::Next, &mut fake_char).await.unwrap();
        npc.run(NpcAction::Selection(0), &mut fake_char)
            .await
            .unwrap();
        npc.run(NpcAction::Next, &mut fake_char).await.unwrap();

        assert!(npc.script.is_none());
    }
}
