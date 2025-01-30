use std::{collections::VecDeque, sync::Arc};

use dashmap::DashMap;
use diesel::{r2d2::ConnectionManager, PgConnection};
use teloxide::types::{ChatId, Message, MessageId, UserId};

use crate::db::models::{Timetable, TimetableEntry};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub pool: DbPool,
    pub http_client: reqwest::Client,
    pub sender: tokio::sync::broadcast::Sender<Event>,
    pub receiver: tokio::sync::broadcast::Receiver<Event>,
    pub cache: DashMap<String, CacheValue>,
    pub fifo_cache: std::sync::Mutex<FifoCache>,
}

pub struct FifoCache {
    pub messages: VecDeque<MessageId>,
}

pub enum CacheValue {
    Integer(i32),
    Text(String),
    List(Vec<String>),
    Timetable(Timetable),
    TimetableEntry(TimetableEntry),
    TimetableEntries(Vec<TimetableEntry>),
    Chat(ChatId),
    User(UserId),
    Message(Message),
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
            cache: DashMap::new(),
            fifo_cache: std::sync::Mutex::new(FifoCache {
                messages: VecDeque::new(),
            }),
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
