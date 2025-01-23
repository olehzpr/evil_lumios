use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

use crate::State;

pub mod models;
pub mod setup;

pub async fn connection(state: &State) -> PooledConnection<ConnectionManager<PgConnection>> {
    state.pool.get().expect("Failed to connect to database")
}
