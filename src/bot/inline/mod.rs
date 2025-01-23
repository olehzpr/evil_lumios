use dck_random::get_dck_random;
use pair_random::get_pair_random;
use teloxide::{
    payloads::AnswerInlineQuerySetters,
    prelude::{Request, Requester},
    types::{InlineQuery, InlineQueryResult},
    Bot,
};

use super::timetable::HandlerResult;

pub mod dck_random;
pub mod pair_random;
pub mod utils;

pub async fn answer_inline_query(bot: Bot, q: InlineQuery) -> HandlerResult {
    let queries = vec![
        InlineQueryResult::Article(get_pair_random(&q)),
        InlineQueryResult::Article(get_dck_random(&q)),
    ];
    let response = bot
        .answer_inline_query(&q.id, queries)
        .cache_time(0)
        .send()
        .await;
    if let Err(err) = response {
        log::error!("Error in handler: {:?}", err);
    }

    Ok(())
}
