#[macro_export]
macro_rules! delete_message {
    ($state:ident, $msg:ident) => {
        $state.sender.send(crate::Event::DeleteMessage {
            chat_id: $msg.chat.id,
            message_id: $msg.id,
        })?;
    };
}

#[macro_export]
macro_rules! param {
    ($bot:ident, $msg:ident, $state:ident, $type:ty, $response:literal) => {
        match crate::bot::utils::params::get_param::<$type>(&$msg) {
            Ok(name) => name,
            Err(_) => {
                let new_msg = $bot.send_message($msg.chat.id, $response).await?;
                delete_message!($state, $msg);
                delete_message!($state, new_msg);
                return Ok(());
            }
        }
    };
}
