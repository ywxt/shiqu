use teloxide::{
    dispatching::{
        DpHandlerDescription,
        dialogue::{self, InMemStorage},
    },
    dptree::{Handler, case},
    prelude::*,
    types::Me,
};

use crate::{command::Command, functions::send_characters_chooser};

#[derive(Debug, Clone, Default)]
pub enum DialogueState {
    #[default]
    Start,
    Interaction {
        session_id: String,
    },
}

pub type Dialogue = teloxide::prelude::Dialogue<DialogueState, InMemStorage<DialogueState>>;
pub type DialogueHandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn dialogue_handler()
-> Handler<'static, DependencyMap, DialogueHandlerResult, DpHandlerDescription> {
    dialogue::enter::<Update, InMemStorage<DialogueState>, DialogueState, _>()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .branch(case![Command::Start { character_id }].endpoint(start)),
        )
        .branch(
            Update::filter_message()
                .branch(case![DialogueState::Interaction { session_id }].endpoint(interact)),
        )
}

async fn start(
    bot: Bot,
    dialogue: Dialogue,
    msg: Message,
    character_id: String, // Available from `case![StartCommand::Start(start)]`
    me: Me,
) -> DialogueHandlerResult {
    if character_id.is_empty() {
        send_characters_chooser(bot, msg, me).await?;
        dialogue.exit().await?;
    } else {
        dialogue
            .update(DialogueState::Interaction {
                session_id: "test".to_string(),
            })
            .await?;
    }
    Ok(())
}

async fn interact(
    _bot: Bot,
    _dialogue: Dialogue,
    _msg: Message,
    _session_id: String, // Available from `case![DialogueState::Interaction{session_id}]`
    _me: Me,
) -> DialogueHandlerResult {
    unimplemented!("interact")
}
