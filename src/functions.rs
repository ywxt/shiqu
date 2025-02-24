use teloxide::{
    Bot,
    types::{Me, Message},
};

use crate::dialogue::DialogueHandlerResult;

pub async fn send_characters_chooser(_bot: Bot, _msg: Message, _me: Me) -> DialogueHandlerResult {
    unimplemented!("send_characters_chooser")
}
