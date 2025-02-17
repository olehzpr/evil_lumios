use std::env;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::prelude::*;

use super::migration::Migrator;

pub async fn connect_db() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(20)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(20))
        .sqlx_logging(true);

    match Database::connect(options).await {
        Ok(db) => {
            let _ = Migrator::up(&db, None).await.map_err(|e| {
                tracing::error!("Error running migrations: {:?}", e);
                std::process::exit(1);
            });
            tracing::info!("Connected to database");
            db
        }
        Err(e) => {
            tracing::error!("Error connecting to database: {:?}", e);
            std::process::exit(1);
        }
    }
}
