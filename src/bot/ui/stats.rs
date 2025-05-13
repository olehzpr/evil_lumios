use crate::models::stats::{FullStats, GroupStats};
use crate::models::user::UserStatsModel;

use super::utils::adapt_for_markdown;
use rand::seq::SliceRandom;

pub fn short_stats(stats: UserStatsModel) -> String {
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
        format!(
            "{} {}",
            stats.current_streak.abs(),
            win_with_case(stats.current_streak.abs())
        )
    } else {
        format!(
            "{} {}",
            stats.current_streak.abs(),
            lose_with_case(stats.current_streak.abs()),
        )
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
        Виграно в сумі:       {:>7}\n\
        Програно в сумі:      {:>7}\n\
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
        stats.num_of_wins,
        stats.num_of_losses,
        stats.total_won,
        stats.total_lost,
        stats.longest_winning_streak,
        stats.longest_losing_streak,
        current_streak
    )
}

pub fn group_stats(group_stats: GroupStats) -> String {
    let mut result = format!("*Статистика групи*\n```{}\n", group_stats.group_name);
    let longest_username = group_stats
        .stats
        .iter()
        .map(|stat| stat.username.len())
        .max()
        .unwrap_or(0);
    for stat in group_stats.stats {
        result.push_str(&format!(
            "{:width$} {:>5}\n",
            stat.username + ":",
            stat.balance,
            width = longest_username
        ));
    }
    result.push_str("```");
    result
}

fn win_with_case(n: i32) -> &'static str {
    if n > 20 {
        return win_with_case(n % 10);
    }
    match n {
        1 => "перемога",
        2..=4 => "перемоги",
        _ => "перемог",
    }
}

fn lose_with_case(n: i32) -> &'static str {
    if n > 20 {
        return lose_with_case(n % 10);
    }
    match n {
        1 => "поразка",
        2..=4 => "поразки",
        _ => "поразок",
    }
}

pub fn casino_welcome() -> (String, String) {
    let text = adapt_for_markdown(
        "Я бачу, ти вирішив випробувати свою удачу в казино. Що ж, прямуй за мною... ".to_string(),
    );
    let image_url =
        "https://vrwf71w421.ufs.sh/f/Mb9Zy1a6B3fQTVV8sDLHXoq2JsGady4L3cntPhFrezAMZ1kl".to_string();
    (text, image_url)
}

pub fn casino_arrival() -> (String, String) {
    let text = adapt_for_markdown(
    "Ось ми й дісталися до казино. Тепер залишилося лише відчинити двері й зробити перший крок у світ ризику та азарту... але пам’ятай, ти можеш і не повернутися."
    .to_string());
    let image_url =
        "https://vrwf71w421.ufs.sh/f/Mb9Zy1a6B3fQ8EEacjfufjuG6RN3BPKe7SqXbFxp4oWgDzwL".to_string();
    (text, image_url)
}

pub fn generate_win_message(bet_amount: i32, new_balance: i32) -> String {
    let messages = [
        "Леді Фортуна посміхається тобі! Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
        "Ти справжній везунчик! Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Продовжуй в тому ж дусі!",
        "Ти продав свою душу дияволу, але це варте того! Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не забувай, що справжній гравець ніколи не здавається!",
        "Невже ти відкрив секрет великої перемоги? Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
        "...і він виграв! Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
        "Дідько його бери, як він це робить! Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
        "Не вірю очам, це що, знову перемога? Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
        "Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
        "Коли він народився, увесь світ прошептав його ім'я - переможець! Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
        "Хакарі б пишався тобою! Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
        "Зажди! Навчи мене! Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
        "Ти виграв, ти великий, ти найкращий! Ти виграв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Так тримати!",
    ];

    let mut rng = rand::thread_rng();
    let message = messages.choose(&mut rng).unwrap();
    message
        .replace("{bet_amount}", &bet_amount.to_string())
        .replace("{new_balance}", &new_balance.to_string())
}

pub fn generate_lose_message(bet_amount: i32, new_balance: i32) -> String {
    let messages = [
        "Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "Що таке? Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "ХА-ХА, ОЦЕ ЛУЗЕР! Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "Забув помолитися перед грою? Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "Яка ж шкода! Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "Невезуча мавпа! Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "Сьогодні не твій день. Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "ХАХВАХВХАХАВХ ЦЕ Ж ТИ? Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "ЯКЕ ЖАЛЮГІДНЕ! Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "Ти програв, ти в нуліну, твоя мама тебе не любить, твій тато тебе не любить, твої друзі тебе не люблять, твій кіт тебе не любить, твій сусід тебе не любить, твій бос тебе не любить! Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
        "Лісова мавпа... Ти програв ставку розміром {bet_amount} поваги і тепер у тебе {new_balance} поваги. Не здавайся, у тебе ще є шанс відігратися!",
    ];

    let mut rng = rand::thread_rng();
    let message = messages.choose(&mut rng).unwrap();
    message
        .replace("{bet_amount}", &bet_amount.to_string())
        .replace("{new_balance}", &new_balance.to_string())
}
