#[macro_export]
macro_rules! delete_message {
    ($state:ident, $msg:ident) => {
        $state.sender.send(Event::DeleteMessage {
            chat_id: $msg.chat.id,
            message_id: $msg.id,
        })?;
    };
}
