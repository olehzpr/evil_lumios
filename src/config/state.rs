use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::Dialogue,
    types::{ChatId, MessageId, UserId},
};

#[derive(Clone, Default)]
pub enum StateMachine {
    #[default]
    Start,
    ReceiveEditTimetableEntry {
        id: i32,
    },
    ReceiveEditTimetableEntryFromMessage {
        id: i32,
        chat_id: ChatId,
        message_id: MessageId,
    },
    ShowFullStats {
        message_id: MessageId,
        user_id: UserId,
    },
}

pub type BotDialogue = Dialogue<StateMachine, InMemStorage<StateMachine>>;
