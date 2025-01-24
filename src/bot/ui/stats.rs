use crate::db::{models::UserStats, stats::FullStats};

pub fn short_stats(stats: UserStats) -> String {
    format!(
        "*Власна статистика*\n\
        ```\n\
        Коротка статистика\n\
        Баланс:               {:>11}\n\
        Щоденний ліміт:       {:>11}\n\
        Доступно на сьогодні: {:>11}\n\
        ```",
        stats.balance,
        stats.daily_limit,
        stats.daily_limit - stats.daily_used
    )
}

pub fn full_stats(stats: FullStats) -> String {
    let mut current_streak = if stats.current_streak > 0 {
        format!("{} перемог", stats.current_streak.abs())
    } else {
        format!("{} поразок", stats.current_streak.abs())
    };
    if stats.current_streak > 5 {
        current_streak += " 🔥";
    }
    if stats.current_streak < -5 {
        current_streak += " 😭";
    }
    format!(
        "*Власна статистика*\n\
        ```\n\
        Повна статистика\n\
        Баланс:               {:>11}\n\
        Щоденний ліміт:       {:>11}\n\
        Доступно на сьогодні: {:>11}\n\
        Середня ставка:       {:>11}\n\
        Кількість ставок:     {:>11}\n\
        -    Перемог:         {:>11}\n\
        -    Поразок:         {:>11}\n\
        Найдовша серія:\n\
        -    Перемог:         {:>11}\n\
        -    Поразок:         {:>11}\n\
        Поточна серія:        {:>11}\n\
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
