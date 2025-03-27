use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
#[non_exhaustive]
pub enum Command {
    #[command(description = "Start a dialogue with a character.")]
    Start { character_id: String },
    #[command(description = "display this text.")]
    Help,
}
