use std::sync::Arc;

use diesel::{r2d2::ConnectionManager, PgConnection};
use teloxide::{
    dispatching::ShutdownToken,
    types::{ChatId, MessageId},
};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub pool: DbPool,
    pub http_client: reqwest::Client,
    pub sender: tokio::sync::broadcast::Sender<Event>,
    pub receiver: tokio::sync::broadcast::Receiver<Event>,
}

#[derive(Clone, Debug)]
pub enum InteractionState {
    WaitingForInput { prompt: String },
}

pub type State = Arc<AppState>;

const MAX_CHANNEL_CAPACITY: usize = 100;

impl AppState {
    pub fn new(pool: DbPool) -> Arc<Self> {
        let (event_tx, event_rx) = tokio::sync::broadcast::channel::<Event>(MAX_CHANNEL_CAPACITY);

        Arc::new(Self {
            pool,
            http_client: reqwest::Client::new(),
            sender: event_tx,
            receiver: event_rx,
        })
    }
}

pub enum InputCommand {
    Exit,
    Help,
    Restart,
    Unknown,
}

impl From<&str> for InputCommand {
    fn from(input: &str) -> Self {
        match input {
            "exit" => Self::Exit,
            "help" => Self::Help,
            "restart" => Self::Restart,
            _ => Self::Unknown,
        }
    }
}

pub struct ShutdownTokens {
    pub dispatcher_token: ShutdownToken,
    pub shutdown_tx: tokio::sync::mpsc::Sender<()>,
}

#[derive(Clone, Debug)]
pub enum Event {
    DeleteMessage {
        chat_id: ChatId,
        message_id: MessageId,
    },
    Exit,
}
