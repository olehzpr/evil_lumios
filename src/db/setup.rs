use std::{env, time::Duration};

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect_db() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(10)
        .max_lifetime(Duration::from_secs(30 * 60))
        .idle_timeout(Duration::from_secs(5 * 60))
        .test_before_acquire(true)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
