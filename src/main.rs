pub mod api;
pub mod bot;
pub mod config;
pub mod cron;
pub mod db;
pub mod redis;
pub mod schema;
pub mod state;

use std::env;

use bot::handler::handler;
use config::{commands::Command, state::StateMachine};
use dotenvy::dotenv;
use state::{AppState, Event, State};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tokio::signal;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .init();
    tracing::info!("Starting app");

    let pool = db::setup::establish_connection_pool();
    let redis = redis::setup::RedisStore::from_env();

    let state = AppState::new(pool, redis);

    let bot_token = match env::var("TELOXIDE_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("TELOXIDE_TOKEN must be set in .env: {:?}", e);
            std::process::exit(1);
        }
    };

    let bot = Bot::new(bot_token);

    if let Err(e) = Command::set_bot_commands(&bot).await {
        tracing::error!("Failed to set bot commands: {:?}", e);
    }

    tokio::spawn(bot::event_handler::event_loop(bot.clone(), state.clone()));

    let mut dispatcher = Dispatcher::builder(bot, handler())
        .dependencies(dptree::deps![
            InMemStorage::<StateMachine>::new(),
            state.clone()
        ])
        .error_handler(LoggingErrorHandler::with_custom_text("An error occurred"))
        .build();
    let shutdown_token = dispatcher.shutdown_token();

    tokio::spawn(async move {
        tracing::info!("Starting telegram dispatcher");
        dispatcher.dispatch().await;
    });

    tokio::spawn(api::start(state.clone()));

    if let Err(e) = cron::cron_loop(state.clone()).await {
        tracing::error!("Failed to start cron loop: {:?}", e);
    }

    let mut recv = state.sender.subscribe();

    tokio::spawn(async move {
        tracing::info!("Enabled graceful shutdown. Press Ctrl+C to exit");
        signal::ctrl_c().await.unwrap();
        state.sender.send(Event::Exit).unwrap();
    });

    loop {
        tokio::select! {
            Ok(event) = recv.recv() => {
                if let Event::Exit = event {
                    tracing::info!("Received shutdown signal");
                    _ = shutdown_token.shutdown();
                    std::process::exit(0);
                }
            }
        }
    }
}
