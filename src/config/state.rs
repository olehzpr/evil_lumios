use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::Dialogue,
    types::{MessageId, UserId},
};

#[derive(Clone, Default)]
pub enum StateMachine {
    #[default]
    Start,
    ReceiveEditTimetableEntry {
        id: i32,
    },
    ShowFullStats {
        message_id: MessageId,
        user_id: UserId,
    },
}

pub type BotDialogue = Dialogue<StateMachine, InMemStorage<StateMachine>>;
