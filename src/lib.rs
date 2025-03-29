use dialogue::{DialogueHandler, DialogueState};
use entity::{CharacterContext, ConversationContext, MessageContext, UserContext};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    dptree::{case, deps},
    prelude::*,
    types::Update,
};

mod api;
mod command;
mod config;
mod db;
mod dialogue;
mod entity;
mod error;
mod functions;
mod http;

pub use config::{ApiKeyConfig, Config};
pub use error::*;

pub async fn run(config: config::Config) {
    let db_pool = db::connect(&config.database_path)
        .await
        .expect("Failed to connect to database");
    db::try_create_tables(&db_pool)
        .await
        .expect("Failed to create tables");
    let bot = teloxide::Bot::new(config.telegram_token);
    let dialogue_handler = dialogue_handler();
    Dispatcher::builder(bot, dialogue_handler)
        .dependencies(deps![InMemStorage::<DialogueState>::new()])
        .dependencies(deps![
            CharacterContext::new(db_pool.clone()),
            ConversationContext::new(db_pool.clone()),
            MessageContext::new(db_pool.clone()),
            UserContext::new(db_pool.clone())
        ])
        .build()
        .dispatch()
        .await;
    db::close(db_pool).await;
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
    teloxide::dispatching::dialogue::enter::<Update, InMemStorage<DialogueState>, DialogueState, _>(
    )
    .branch(message_handler)
    .branch(callback_handler)
}
