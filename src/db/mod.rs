use async_trait::async_trait;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

use crate::State;

pub mod chat;
pub mod gamble;
pub mod models;
pub mod queue;
pub mod setup;
pub mod stats;
pub mod timetable;
pub mod user;

#[async_trait]
pub trait StateWithConnection {
    async fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>>;
}

#[async_trait]
impl StateWithConnection for State {
    async fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to connect to database")
    }
}
