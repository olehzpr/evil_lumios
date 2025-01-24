use serde::Deserialize;
use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText};

use super::utils::hashed_rand;

#[derive(Deserialize)]
pub struct GenderConfig {
    id: String,
    title: String,
    responses: Vec<String>,
    lychko_response: String,
    description: String,
    img_url: String,
}

pub fn gender_random(
    q: &teloxide::types::InlineQuery,
    config: &GenderConfig,
) -> teloxide::types::InlineQueryResultArticle {
    let username = q.from.username.as_ref().unwrap();
    let random_value = hashed_rand(&[username]);
    let mut answer = &config.responses[random_value as usize % config.responses.len()];
    if username == "@lychkoalexander" {
        answer = &config.lychko_response;
    }
    let gender = InlineQueryResultArticle::new(
        config.id.clone(),
        config.title.clone(),
        InputMessageContent::Text(InputMessageContentText::new(answer)),
    )
    .description(config.description.clone())
    .thumbnail_url(config.img_url.parse().unwrap());

    return gender;
}
