use dialogue::{DialogueHandler, DialogueState};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    dptree::{case, deps},
    prelude::*,
    types::Update,
};

mod api;
mod command;
mod dialogue;
mod error;
mod functions;
mod http;

pub use error::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let bot = teloxide::Bot::from_env();
    let dialogue_handler = dialogue_handler();
    Dispatcher::builder(bot, dialogue_handler)
        .dependencies(deps![InMemStorage::<DialogueState>::new()])
        .build()
        .dispatch()
        .await;
}

fn command_handler() -> DialogueHandler {
    teloxide::filter_command::<command::Command, _>()
        .branch(case![command::Command::Start { character_id }].endpoint(dialogue::start_chat))
}

fn dialogue_handler() -> DialogueHandler {
    let message_handler = Update::filter_message()
        .branch(command_handler())
        .branch(case![DialogueState::Interaction { session_id }].endpoint(dialogue::interact));

    let callback_handler = Update::filter_callback_query()
    .branch(case![DialogueState::ChooseCharacter].endpoint(dialogue::choose_character));
    teloxide::dispatching::dialogue::enter::<Update, InMemStorage<DialogueState>, DialogueState, _>()
    .branch(message_handler)
    .branch(callback_handler)
}
