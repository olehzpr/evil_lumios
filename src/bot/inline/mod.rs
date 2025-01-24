use std::fs;

use dck_random::{get_dck_random, DckConfig};
use gender::{gender_random, GenderConfig};
use pair_random::{get_pair_random, PairConfig};
use polyana::{polyana_random, PolyanaConfig};
use serde::Deserialize;
use teloxide::{
    payloads::AnswerInlineQuerySetters,
    prelude::{Request, Requester},
    types::{InlineQuery, InlineQueryResult},
    Bot,
};
use zrada::{zrada_random, ZradaConfig};

use super::timetable::HandlerResult;

pub mod dck_random;
pub mod gender;
pub mod pair_random;
pub mod polyana;
pub mod utils;
pub mod zrada;

#[derive(Deserialize)]
pub struct Config {
    pair: PairConfig,
    dck: DckConfig,
    polyana: PolyanaConfig,
    gender: GenderConfig,
    zrada: ZradaConfig,
}

pub async fn answer_inline_query(bot: Bot, q: InlineQuery) -> HandlerResult {
    let config = load_config("src/bot/inline/config.toml");
    if let Err(err) = config {
        tracing::error!("Invalid configuration. Skipping inline query: {:?}", err);
        return Ok(());
    }
    let config = config.unwrap();
    let queries = vec![
        InlineQueryResult::Article(get_pair_random(&q, &config.pair)),
        InlineQueryResult::Article(get_dck_random(&q, &config.dck)),
        InlineQueryResult::Article(polyana_random(&q, &config.polyana)),
        InlineQueryResult::Article(gender_random(&q, &config.gender)),
        InlineQueryResult::Article(zrada_random(&q, &config.zrada)),
    ];
    let response = bot
        .answer_inline_query(&q.id, queries)
        .cache_time(0)
        .send()
        .await;
    if let Err(err) = response {
        tracing::error!("Inline query answer error: {:?}", err);
    }

    Ok(())
}

pub fn load_config(file_path: &str) -> anyhow::Result<Config> {
    let config_data = fs::read_to_string(file_path).map_err(|err| {
        tracing::error!("Failed to read the TOML configuration file: {:?}", err);
        anyhow::anyhow!(err)
    })?;
    let config: Config = toml::from_str(&config_data).map_err(|err| {
        tracing::error!("Failed to parse TOML configuration: {:?}", err);
        anyhow::anyhow!(err)
    })?;
    Ok(config)
}
