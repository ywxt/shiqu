use teloxide::{
    dispatching::{DpHandlerDescription, dialogue::InMemStorage},
    prelude::*,
    types::Me,
};

use crate::functions::send_characters_chooser;

#[derive(Debug, Clone, Default)]
pub enum DialogueState {
    #[default]
    Start,
    ChooseCharacter,
    Interaction {
        session_id: String,
    },
}

pub type Dialogue = teloxide::prelude::Dialogue<DialogueState, InMemStorage<DialogueState>>;
pub type DialogueHandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub type DialogueHandler =
    Handler<'static, DependencyMap, DialogueHandlerResult, DpHandlerDescription>;

pub async fn start_chat(
    bot: Bot,
    dialogue: Dialogue,
    msg: Message,
    character_id: String,
) -> DialogueHandlerResult {
    if character_id.is_empty() {
        send_characters_chooser(bot, msg).await?;
        dialogue.update(DialogueState::ChooseCharacter).await?;
    } else {
        dialogue
            .update(DialogueState::Interaction {
                session_id: "test".to_string(),
            })
            .await?;
    }
    Ok(())
}

pub async fn interact(
    bot: Bot,
    dialogue: Dialogue,
    msg: Message,
    session_id: String, // Available from `case![DialogueState::Interaction{session_id}]`
    me: Me,
) -> DialogueHandlerResult {
    bot.send_message(msg.chat.id, msg.text().unwrap_or_default())
        .await?;
    Ok(())
}

pub async fn choose_character(
    _bot: Bot,
    _dialogue: Dialogue,
    _query: CallbackQuery,
    _me: Me,
) -> DialogueHandlerResult {
    unimplemented!("choose_character")
}
