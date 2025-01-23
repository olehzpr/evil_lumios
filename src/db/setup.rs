use std::env;

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use crate::state::DbPool;

pub fn establish_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
