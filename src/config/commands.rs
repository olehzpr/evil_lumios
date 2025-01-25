use std::env;

use teloxide::{
    payloads::SetMyCommandsSetters,
    prelude::{Requester, ResponseResult},
    types::Message,
    utils::command::BotCommands,
    Bot,
};

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "snake_case", parse_with = "split")]
pub enum Command {
    // General
    #[command(description = "Почати роботу з ботом")]
    Start,

    #[command(description = "Відкрити довідку користувача")]
    Help,

    // Queues
    #[command(description = "Створити чергу")]
    Queue,

    #[command(description = "Створити мішану чергу")]
    Mixed,

    // Schedule
    #[command(description = "Імпортувати існуюючий розклад")]
    Import,

    #[command(description = "Редагувати розклад")]
    EditTimetable,

    #[command(description = "Показати розклад на сьогодні")]
    Today,

    #[command(description = "Показати розклад на завтра")]
    Tomorrow,

    #[command(description = "Показати розклад тижня")]
    Week,

    #[command(description = "Посилання на занняття що йде зараз")]
    Now,

    #[command(description = "Показати наступне заняття")]
    Next,

    // Stats
    #[command(description = "Показати статистику")]
    Stats,

    #[command(description = "Зайти в казино")]
    Casino,

    #[command(description = "Переглянути свою статистику")]
    Me,

    #[command(description = "Запустити колесо фортуни")]
    Wheel,

    #[command(description = "Команда для лудоманів")]
    Gamble,

    #[command(description = "Команда для повних лудоманів")]
    GambleAll,
}

impl Command {
    pub async fn set_bot_commands(bot: &Bot) -> ResponseResult<()> {
        use teloxide::types::{BotCommand, BotCommandScope};
        let commands = Command::descriptions()
            .to_string()
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, " — ");
                Some(BotCommand {
                    command: parts.next()?[1..].to_string(),
                    description: parts.next()?.to_string(),
                })
            })
            .collect::<Vec<BotCommand>>();

        bot.set_my_commands(commands)
            .scope(BotCommandScope::AllGroupChats)
            .await?;

        Ok(())
    }

    pub fn filter(msg: &Message) -> Option<Command> {
        let bot_username = env::var("BOT_USERNAME").unwrap_or_else(|_| {
            tracing::error!("Environment variable BOT_USERNAME is not set");
            String::new()
        });
        if let Some(text) = msg.text() {
            if text.starts_with('/') {
                return Command::parse(text, &bot_username).ok();
            }
        }
        None
    }
}
