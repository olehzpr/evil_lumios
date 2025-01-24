use serde::Deserialize;
use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText};

use super::utils::hashed_rand;

#[derive(Deserialize)]
pub struct DckConfig {
    id: String,
    emoji: Vec<String>,
    title: String,
    missing_response: String,
    smallest_response: String,
    normal_response: String,
    largest_response: String,
    description: String,
    img_url: String,
}

pub fn get_dck_random(
    q: &teloxide::types::InlineQuery,
    config: &DckConfig,
) -> teloxide::types::InlineQueryResultArticle {
    let username = q.from.username.as_ref().unwrap();
    let random_value = hashed_rand(&[username]);
    let emoji = &config.emoji[random_value as usize % config.emoji.len()];
    let len = ((random_value % 3000) as f64) / 100.0 + 1.0;
    let answer = match random_value % 100 {
        0..=10 => config.missing_response.clone(),
        11..=20 => config.smallest_response.clone(),
        21..=90 => config
            .normal_response
            .replace("{emoji}", emoji)
            .replace("{len}", &len.to_string()),
        _ => config.largest_response.clone(),
    };
    let dck_random = InlineQueryResultArticle::new(
        config.id.clone(),
        config.title.clone(),
        InputMessageContent::Text(InputMessageContentText::new(answer)),
    )
    .description(config.description.clone())
    .thumbnail_url(config.img_url.parse().unwrap());

    return dck_random;
}
