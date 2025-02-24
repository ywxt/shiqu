use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
#[non_exhaustive]
pub enum Command {
    Start{
        character_id: String,
    },
    Help,
}