use std::sync::Arc;

use diesel::{r2d2::ConnectionManager, PgConnection};
use teloxide::types::{ChatId, MessageId};

use crate::redis::setup::RedisStore;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub pool: DbPool,
    pub redis: RedisStore,
    pub http_client: reqwest::Client,
    pub sender: tokio::sync::broadcast::Sender<Event>,
    pub receiver: tokio::sync::broadcast::Receiver<Event>,
}

pub type State = Arc<AppState>;

const MAX_CHANNEL_CAPACITY: usize = 100;

impl AppState {
    pub fn new(pool: DbPool, redis: RedisStore) -> Arc<Self> {
        let (event_tx, event_rx) = tokio::sync::broadcast::channel::<Event>(MAX_CHANNEL_CAPACITY);

        Arc::new(Self {
            pool,
            redis,
            http_client: reqwest::Client::new(),
            sender: event_tx,
            receiver: event_rx,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    DeleteMessage {
        chat_id: ChatId,
        message_id: MessageId,
    },
    Notify {
        chat_id: ChatId,
        entry_id: i32,
    },
    Exit,
}
