use crate::db::{models::UserStats, stats::FullStats};

use super::utils::adapt_for_markdown;

pub fn short_stats(stats: UserStats) -> String {
    format!(
        "*Власна статистика*\n\
        ```\n\
        Коротка статистика\n\
        Баланс:               {:>7}\n\
        Щоденний ліміт:       {:>7}\n\
        Доступно на сьогодні: {:>7}\n\
        ```",
        stats.balance,
        stats.daily_limit,
        stats.daily_limit - stats.daily_used
    )
}

pub fn full_stats(stats: FullStats) -> String {
    let current_streak = if stats.current_streak > 0 {
        format!("{} перемог", stats.current_streak.abs())
    } else {
        format!("{} поразок", stats.current_streak.abs())
    };
    format!(
        "*Власна статистика*\n\
        ```\n\
        Повна статистика\n\
        Баланс:               {:>7}\n\
        Щоденний ліміт:       {:>7}\n\
        Доступно на сьогодні: {:>7}\n\
        Середня ставка:       {:>7}\n\
        Кількість ставок:     {:>7}\n\
        -    Перемог:         {:>7}\n\
        -    Поразок:         {:>7}\n\
        Найдовша серія:\n\
        -    Перемог:         {:>7}\n\
        -    Поразок:         {:>7}\n\
        Поточна серія:    {:>11}\n\
        ```",
        stats.balance,
        stats.daily_limit,
        stats.daily_limit - stats.daily_used,
        stats.average_bet,
        stats.total_gambles,
        stats.total_won,
        stats.total_lost,
        stats.longest_winning_streak,
        stats.longest_losing_streak,
        current_streak
    )
}

pub fn casino_welcome() -> (String, String) {
    let text = adapt_for_markdown(
    "Я бачу, ти вирішив випробувати свою удачу в казино. Що ж, прямуй за мною... наш шлях починається."
    .to_string());
    let image_url =
        "https://vrwf71w421.ufs.sh/f/Mb9Zy1a6B3fQTVV8sDLHXoq2JsGady4L3cntPhFrezAMZ1kl".to_string();
    (text, image_url)
}

pub fn casino_arrival() -> (String, String) {
    let text = adapt_for_markdown(
    "Ось ми й дісталися до казино. Тепер залишилося лише відчинити двері й зробити перший крок у світ ризику та азарту... але пам’ятай, ти можеш і не повернутися."
    .to_string());
    let image_url =
        "https://vrwf71w421.ufs.sh/f/Mb9Zy1a6B3fQeXVcpUy4tiJaTF63qupnIx8gHBh0WbLws1DY".to_string();
    (text, image_url)
}
