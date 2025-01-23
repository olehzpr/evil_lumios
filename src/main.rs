pub mod bot;
pub mod config;
pub mod db;
pub mod schema;
pub mod state;

use std::env;

use bot::handler::handler;
use config::{commands::Command, state::StateMachine};
use dotenvy::dotenv;
use state::{AppState, Event, State};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::setup::establish_connection_pool();

    let state = AppState::new(pool);

    let bot_token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN must be set in .env");

    let bot = Bot::new(bot_token);

    Command::set_bot_commands(&bot)
        .await
        .expect("Failed to set bot commands");

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
        dispatcher.dispatch().await;
    });

    let mut recv = state.sender.subscribe();

    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        state.sender.send(Event::Exit).unwrap();
    });

    loop {
        tokio::select! {
            Ok(event) = recv.recv() => {
                if let Event::Exit = event {
                    eprintln!("Received shutdown signal");
                    _ = shutdown_token.shutdown();
                    std::process::exit(0);
                }
            }
        }
    }
}
