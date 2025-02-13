use std::env;

use rand::seq::SliceRandom;

use crate::state::State;

pub async fn get_random_gif(state: &State, is_win: bool) -> anyhow::Result<String> {
    let keyword = if is_win {
        get_random_win_keyword()
    } else {
        get_random_lose_keyword()
    };
    let tenor_api_key = env::var("TENOR_API_KEY").expect("TENOR_API_KEY must be set");
    tracing::info!(
        "Fetching gif for keyword: {}, with key {}",
        keyword,
        tenor_api_key
    );
    let gif_response = state
        .http_client
        .get("https://tenor.googleapis.com/v2/search")
        .query(&[("q", keyword)])
        .query(&[("key", tenor_api_key)])
        .query(&[("client_key", "evil-lumios")])
        .query(&[("limit", "10")])
        .query(&[("random", "true")])
        .query(&[("media_filter", "gif")])
        .send()
        .await?;

    let content = gif_response.json::<serde_json::Value>().await?;
    let urls = content["results"]
        .as_array()
        .and_then(|array| {
            Some(
                array
                    .iter()
                    .map(|x| x["media_formats"]["gif"]["url"].to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap_or_default();

    let gif_url = match urls.choose(&mut rand::thread_rng()) {
        Some(url) => url,
        None => {
            tracing::warn!("No gif urls found");
            return Ok("".to_string());
        }
    };

    Ok(gif_url.trim_matches('"').to_string())
}

fn get_random_win_keyword() -> String {
    let keywords = vec![
        "Jackpot",
        "Casino",
        "Luck",
        "Lucky",
        "Gamble",
        "Rich",
        "Royal Flush",
        "JJK",
        "Gojo Satoru",
        "Anime",
        "Mahoraga",
        "Gojo",
    ];
    return keywords[rand::random::<usize>() % keywords.len()].to_string();
}

fn get_random_lose_keyword() -> String {
    let keywords = vec![
        "Loser",
        "Casino",
        "Bankrupt",
        "Dark Souls",
        "Wasted",
        "Died",
        "Death",
        "Elden Ring",
        "JJK",
        "Anime",
    ];
    return keywords[rand::random::<usize>() % keywords.len()].to_string();
}
