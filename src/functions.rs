use teloxide::{
    Bot,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
};

use crate::dialogue::DialogueHandlerResult;

pub async fn send_characters_chooser(bot: Bot, msg: Message) -> DialogueHandlerResult {
    let characters =
        ["Alice", "Bob", "Charlie"].map(|chr| InlineKeyboardButton::callback(chr, chr));
    bot.send_message(msg.chat.id, "Choose a character:")
        .reply_markup(InlineKeyboardMarkup::new([characters]))
        .await?;
    Ok(())
}
