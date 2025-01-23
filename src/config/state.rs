use teloxide::{dispatching::dialogue::InMemStorage, prelude::Dialogue};

#[derive(Clone, Default)]
pub enum StateMachine {
    #[default]
    Start,
    ReceiveEditTimetableEntry {
        id: i32,
    },
}

pub type BotDialogue = Dialogue<StateMachine, InMemStorage<StateMachine>>;
