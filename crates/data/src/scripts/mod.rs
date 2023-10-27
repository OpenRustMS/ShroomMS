use std::{pin::Pin, task::Poll};

use anyhow::Context;
use futures::Future;
use proto95::{
    game::script::{
        AskMsg, AskNumberMsg, AskTextMsg, MsgParamFlags, OptionAnswer, SayMsg, ScriptAnswerReq,
        ScriptMessage,
    },
    id::{ItemId, SkillId},
};
use tokio::sync::mpsc;

use crate::services::character::Character;

use proto95::fmt::{ShroomDisplay, ShroomMenuList};

#[derive(Debug, PartialEq, Eq)]
pub enum NpcAction {
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

pub async fn npc_script_1000(api: &mut ScriptCtx<Character>) -> anyhow::Result<()> {
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

impl ScriptCtx<Character> {
    pub async fn wait_for_next(&mut self) -> anyhow::Result<()> {
        let action = self.wait_for_action().await?;
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
            c.npc_msg.push_back(ScriptMessage::AskText(AskTextMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
                default_txt: default,
                min: min as u16,
                max: max as u16,
            }))
        });

        match self.wait_for_action().await? {
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
            c.npc_msg.push_back(ScriptMessage::AskNumber(AskNumberMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
                default_number: default,
                min,
                max,
            }))
        });

        match self.wait_for_action().await? {
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
            c.npc_msg.push_back(ScriptMessage::AskMenu(AskMsg {
                param: MsgParamFlags::empty(),
                msg,
            }))
        });

        Ok(match self.wait_for_action().await? {
            NpcAction::Selection(v) if v >= items.len() => {
                anyhow::bail!("Invalid selection: {v}, max {}", items.len())
            }
            NpcAction::Selection(v) => v,
            _ => anyhow::bail!("Expected YesNo action"),
        })
    }

    pub async fn ask_yes_no(&mut self, text: impl ShroomDisplay) -> anyhow::Result<bool> {
        self.with_mut(|c| {
            c.npc_msg.push_back(ScriptMessage::AskYesNo(AskMsg {
                param: MsgParamFlags::empty(),
                msg: text.to_shroom_string(),
            }))
        });

        match self.wait_for_action().await? {
            NpcAction::YesNo(b) => Ok(b),
            _ => anyhow::bail!("Expected YesNo action"),
        }
    }

    pub async fn say(&mut self, text: impl ShroomDisplay, last: bool) -> anyhow::Result<()> {
        self.with_mut(|c| {
            c.npc_msg.push_back(ScriptMessage::Say(SayMsg {
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
        self.with(|c| c.stats.level)
    }

    pub fn set_char_level(&mut self, lvl: u8) {
        self.with_mut(|c| c.stats.level_mut().set(lvl));
    }

    pub fn has_item(&self, id: ItemId) -> bool {
        self.with(|c| c.inventory.contains_id(&id).unwrap_or(false))
    }

    pub fn try_update_money(&mut self, money: i32) -> bool {
        self.with_mut(|c| c.update_mesos(money))
    }

    pub fn try_give_items(&mut self, ids: &[(ItemId, usize)]) -> bool {
        self.with_mut(|c| {
            for (id, quantity) in ids.iter().cloned() {
                c.add_stack_item(id.get_inv_type().unwrap(), id, quantity)
                    .unwrap();
            }

            true
        })
    }
}
pub struct ScriptCtx<T> {
    state: *mut T,
    waiting_for_action: bool,
    action_rx: mpsc::Receiver<NpcAction>,
}

impl<T> ScriptCtx<T> {
    pub fn with<F, U>(&self, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        // Safety: ScriptCtx can be only constructed by a `ScriptHandle`
        // which ensures the pointer is valid when It's used
        unsafe { f(self.state.as_ref().expect("state ref")) }
    }

    pub fn with_mut<F, U>(&mut self, f: F) -> U
    where
        F: FnOnce(&mut T) -> U,
    {
        // Safety: ScriptCtx can be only constructed by a `ScriptHandle`
        // which ensures the pointer is valid when It's used
        unsafe { f(self.state.as_mut().expect("state ref mut")) }
    }

    pub async fn wait_for_action(&mut self) -> anyhow::Result<NpcAction> {
        self.waiting_for_action = true;
        let msg = self.action_rx.recv().await.context("rx")?;
        self.waiting_for_action = false;

        if msg == NpcAction::End {
            anyhow::bail!("Script ended");
        }

        Ok(msg)
    }
}

pub struct ScriptHandle<State> {
    fut: Pin<Box<dyn Future<Output = anyhow::Result<()>>>>,
    tx: mpsc::Sender<NpcAction>,
    ctx: Box<ScriptCtx<State>>,
}

unsafe impl<State: Send> Send for ScriptHandle<State> {}
unsafe impl<State: Sync> Sync for ScriptHandle<State> {}

impl<State> ScriptHandle<State> {
    pub fn from_script_fn<'a, F, Fut>(f: F, data: &mut State) -> Self
    where
        F: FnOnce(&'a mut ScriptCtx<State>) -> Fut,
        Fut: Future<Output = anyhow::Result<()>> + 'static,
        State: 'static,
    {
        let ptr = data as *mut State;
        let (tx, rx) = mpsc::channel(16);
        let mut ctx = Box::new(ScriptCtx {
            state: ptr,
            action_rx: rx,
            waiting_for_action: false,
        });
        // We create a second reference to the context
        // however we ensure in poll_script that only one mut ref is used at a time
        let ctx_ref_2 = unsafe { (&mut *ctx as *mut ScriptCtx<State>).as_mut().expect("ctx") };
        Self {
            fut: Box::pin(f(ctx_ref_2)),
            tx,
            ctx,
        }
    }

    pub fn poll_script<'a>(&'a mut self, state: &'a mut State) -> ScriptPoll<'a, State> {
        // Assert that the state is the same and own It while polling
        assert_eq!(self.ctx.state, state as *mut _);
        ScriptPoll {
            script: self,
            _state: state,
        }
    }

    pub fn send_action(&self, action: NpcAction) {
        self.tx.try_send(action).unwrap();
    }
}

pub struct ScriptPoll<'a, State> {
    script: &'a mut ScriptHandle<State>,
    // Poll has to keep the reference to the state alive
    _state: &'a mut State,
}

pub enum ScriptPollResult {
    Done(anyhow::Result<()>),
    Pending,
}

impl<'a, State> Future for ScriptPoll<'a, State> {
    type Output = ScriptPollResult;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        match self.script.fut.as_mut().poll(cx) {
            Poll::Ready(v) => Poll::Ready(ScriptPollResult::Done(v)),
            Poll::Pending if self.script.ctx.waiting_for_action => {
                Poll::Ready(ScriptPollResult::Pending)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;

    use super::*;

    async fn script_t(session: &mut ScriptCtx<u32>) -> anyhow::Result<()> {
        session.with_mut(|s| *s += 1);
        sleep(Duration::from_millis(10)).await;
        let t = session.with(|s| s * 2);
        let _ = session.wait_for_action().await;
        session.with_mut(|s| *s = t);
        Ok(())
    }

    #[tokio::test]
    async fn script_poll() {
        let mut data = 0u32;
        let mut scrpt = ScriptHandle::from_script_fn(script_t, &mut data);
        let mut polls = 0;
        loop {
            polls += 1;
            match scrpt.poll_script(&mut data).await {
                ScriptPollResult::Done(_) => break,
                ScriptPollResult::Pending => sleep(Duration::from_millis(10)).await,
            }
            scrpt.tx.try_send(NpcAction::Next).unwrap();
        }

        // Polls twice, before/after the action
        assert_eq!(polls, 2);
        // (0 + 1) * 2
        assert_eq!(data, 2);
    }

    #[tokio::test]
    #[should_panic]
    async fn invalid_poll() {
        let mut data = 0u32;
        let mut scrpt = ScriptHandle::from_script_fn(script_t, &mut data);
        let mut other = 1u32;

        // This panic will fail because data != other
        scrpt.poll_script(&mut other).await;
    }
}
