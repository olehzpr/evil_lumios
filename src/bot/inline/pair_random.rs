use serde::Deserialize;
use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText};

use super::utils::hashed_rand;

#[derive(Deserialize)]
pub struct PairConfig {
    id: String,
    emoji: Vec<String>,
    title: String,
    title_with_name: String,
    response: String,
    username_response: String,
    description: String,
    img_url: String,
}

pub fn get_pair_random(
    q: &teloxide::types::InlineQuery,
    config: &PairConfig,
) -> teloxide::types::InlineQueryResultArticle {
    let username = q.from.username.as_ref().unwrap().as_str();

    let message = match q.query.trim() {
        "" => config.title.clone(),
        query if query.starts_with("@") => config.title_with_name.replace("{name}", &q.query[1..]),
        _ => config.title_with_name.replace("{name}", &q.query),
    };

    let answer = match q.query.trim() {
        "" => InputMessageContent::Text(InputMessageContentText::new("* звуки мовчання *")),
        query if query.starts_with("@") => {
            let emoji =
                &config.emoji[hashed_rand(&[username, &query[1..]]) as usize % config.emoji.len()];
            let percent = hashed_rand(&[username, &query[1..]]) % 101;

            InputMessageContent::Text(InputMessageContentText::new(
                config
                    .username_response
                    .replace("{emoji}", emoji)
                    .replace("{name}", &q.query[1..])
                    .replace("{percent}", &percent.to_string()),
            ))
        }
        _ => {
            let emoji =
                &config.emoji[hashed_rand(&[username, &q.query]) as usize % config.emoji.len()];
            let percent = hashed_rand(&[username, &q.query]) % 101;

            InputMessageContent::Text(InputMessageContentText::new(
                config
                    .response
                    .replace("{emoji}", emoji)
                    .replace("{name}", &q.query)
                    .replace("{percent}", &percent.to_string()),
            ))
        }
    };

    InlineQueryResultArticle::new(config.id.clone(), message, answer)
        .description(config.description.clone())
        .thumbnail_url(config.img_url.parse().unwrap())
}
