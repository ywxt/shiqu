use dialogue::DialogueState;
use teloxide::{
    dispatching::{Dispatcher, dialogue::InMemStorage},
    dptree::deps,
};

mod command;
mod dialogue;
mod functions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let bot = teloxide::Bot::from_env();
    let dialogue_handler = dialogue::dialogue_handler();
    Dispatcher::builder(bot, dialogue_handler)
        .dependencies(deps![InMemStorage::<DialogueState>::new()])
        .build()
        .dispatch()
        .await;
}
