use crate::bot::handler::HandlerResult;
use crate::bot::utils::random::get_random_bool;
use crate::db::gamble::{insert_gamble, GambleDto, GambleType};
use crate::db::stats::{get_group_stats, update_balance};
use crate::db::user::get_user_by_account_id;
use crate::state::Event;
use crate::{bot::ui, db::stats::get_user_stats, State};
use crate::{delete_message, param};
use reqwest::Url;
use teloxide::payloads::{EditMessageReplyMarkupSetters, SendMessageSetters, SendPhotoSetters};
use teloxide::prelude::Request;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use teloxide::{prelude::Requester, types::Message, Bot};

use super::gifs::get_random_gif;

pub async fn stats(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let users_stats = get_group_stats(&state.db, msg.chat.id).await?;
    let res = ui::stats::group_stats(users_stats);
    let new_msg = bot
        .send_message(msg.chat.id, &res)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .send()
        .await?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);
    Ok(())
}

pub async fn casino(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let (res, url) = ui::stats::casino_welcome();
    let bot_name = bot.get_me().await?.user.username.unwrap();
    let new_msg = bot
        .send_photo(msg.chat.id, InputFile::url(Url::parse(&url).unwrap()))
        .caption(res)
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::url(
                "Пройти до казино",
                Url::parse(&format!("https://t.me/{}?start=casino", bot_name)).unwrap(),
            ),
        ]]))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .send()
        .await?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);

    Ok(())
}

pub async fn me(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let user_id = msg.from.as_ref().unwrap().id;
    let stats = get_user_stats(&state.db, msg.from.unwrap().id).await?;
    let res = ui::stats::short_stats(stats);
    if let Err(e) = state.sender.send(Event::DeleteMessage {
        chat_id: msg.chat.id,
        message_id: msg.id,
    }) {
        eprintln!("Failed to send delete message event: {:?}", e);
    }

    let sent_msg = bot
        .send_message(msg.chat.id, &res)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::callback("Показати всю статистику", "loading"),
        ]]))
        .await?;

    bot.edit_message_reply_markup(sent_msg.chat.id, sent_msg.id)
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::callback(
                "Показати всю статистику",
                format!("show-full-stats_{}_{}", user_id, sent_msg.id),
            ),
        ]]))
        .send()
        .await?;

    delete_message!(state, msg);
    delete_message!(state, sent_msg);

    Ok(())
}

pub async fn wheel(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "Wheel command").await?;
    Ok(())
}

pub async fn gamble(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let amount = param!(bot, msg, state, u32, "Вкажіть ціле невідʼємне число");

    let result = make_bet(&state, &msg, Amount::Value(amount)).await;
    if let Err(error) = result {
        bot.send_message(msg.chat.id, error.to_string()).await?;
        return Ok(());
    }
    let mut result = result.unwrap();

    let gif = get_random_gif(&state, result.is_win).await?;

    let new_msg = bot
        .send_animation(msg.chat.id, InputFile::url(Url::parse(&gif).unwrap()))
        .send()
        .await?;

    result.message_id = new_msg.id;

    let gamble = insert_gamble(&state.db, result).await?;

    state.sender.send(Event::GambleResult {
        chat_id: msg.chat.id,
        gamble_id: gamble.id,
    })?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);

    Ok(())
}

pub async fn gamble_all(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let result = make_bet(&state, &msg, Amount::All).await;
    if let Err(error) = result {
        bot.send_message(msg.chat.id, error.to_string()).await?;
        return Ok(());
    }
    let mut result = result.unwrap();

    let gif = get_random_gif(&state, result.is_win).await?;

    let new_msg = bot
        .send_animation(msg.chat.id, InputFile::url(Url::parse(&gif).unwrap()))
        .send()
        .await?;

    result.message_id = new_msg.id;

    let gamble = insert_gamble(&state.db, result).await?;

    state.sender.send(Event::GambleResult {
        chat_id: msg.chat.id,
        gamble_id: gamble.id,
    })?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);

    Ok(())
}

enum Amount {
    All,
    Value(u32),
}

async fn make_bet(state: &State, msg: &Message, amount: Amount) -> anyhow::Result<GambleDto> {
    let user = msg.from.as_ref().unwrap();
    let stored_user = get_user_by_account_id(&state, user.id).await?;
    let user_stats = get_user_stats(&state.db, user.id).await?; //fix error handling;

    let amount = match amount {
        Amount::All => user_stats.balance as u32,
        Amount::Value(value) => value,
    };

    if user_stats.balance < amount as i32 {
        return Err(anyhow::anyhow!("Недостатньо коштів"));
    }

    let username = user.username.as_ref().unwrap_or(&String::new()).clone();

    let result = get_random_bool(username);

    const WIN_COEFFICIENT: f32 = 0.4;
    const LOSE_COEFFICIENT: f32 = 0.5;
    let new_balance = if result {
        user_stats.balance + (amount as f32 * WIN_COEFFICIENT) as i32
    } else {
        user_stats.balance - (amount as f32 * LOSE_COEFFICIENT) as i32
    };

    let change = new_balance - user_stats.balance;

    update_balance(&state.db, stored_user.id, change).await?;

    Ok(GambleDto {
        user_id: stored_user.id,
        message_id: msg.id,
        is_win: result,
        change,
        bet: amount as i32,
        gamble_type: GambleType::Bet,
    })
}
