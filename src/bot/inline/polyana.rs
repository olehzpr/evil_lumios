use serde::Deserialize;
use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText};

use super::utils::rand;

#[derive(Deserialize)]
pub struct PolyanaConfig {
    id: String,
    title: String,
    responses: Vec<String>,
    description: String,
    img_url: String,
}

pub fn polyana_random(
    _q: &teloxide::types::InlineQuery,
    config: &PolyanaConfig,
) -> teloxide::types::InlineQueryResultArticle {
    let random_value = rand();
    let answer = &config.responses[random_value as usize % config.responses.len()];
    let polyana_random = InlineQueryResultArticle::new(
        config.id.clone(),
        config.title.clone(),
        InputMessageContent::Text(InputMessageContentText::new(answer.clone())),
    )
    .description(config.description.clone())
    .thumbnail_url(config.img_url.parse().unwrap());

    polyana_random
}
