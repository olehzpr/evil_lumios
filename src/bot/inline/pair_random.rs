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

    for pair in CUSTOM {
        let full_username = format!("@{}", username);
        let pair = get_pair(*pair, &full_username, q.query.trim());
        if let Some(answer) = pair {
            let emoji =
                &config.emoji[hashed_rand(&[username, &q.query]) as usize % config.emoji.len()];
            let percent = hashed_rand(&[username, &q.query]) % 101;
            let answer = answer
                .to_string()
                .replace("{emoji}", emoji)
                .replace("{name}", &q.query)
                .replace("{percent}", &percent.to_string());

            return InlineQueryResultArticle::new(
                config.id.clone(),
                message,
                InputMessageContent::Text(InputMessageContentText::new(answer)),
            )
            .description(config.description.clone())
            .thumbnail_url(config.img_url.parse().unwrap());
        }
    }

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

#[rustfmt::skip]
const CUSTOM: &[ [&str; 3] ] = &[
    // 1	Бандура Артем Ібрагімович @CabianoFaruana
    ["@CabianoFaruana", "@olehzpr", "Ви з Олегом підходите на 69% 😏😏😏. Тільки міцна чоловіча дружба, нічого більше"],
    ["@CabianoFaruana", "псж", "ПСЖ — найкращий клуб! 🔥⚽"],
    ["@CabianoFaruana", "корнага", "Ви з Корнагою підходите на 🔥 101% 🔥"],
    // 2	Войтюк Анастасія Сергіївна @wiastia
    ["@wiastia", "чай", "Ви з чаєм підходите на 100% 🍵. Тобі б дійсно не завадило попити трохи чаю"],
    // 3	Гавриш Олександр Дмитрович @ParisHavrysh
    // 4	Лісовиченко Кирило Олександрович @kyryloloshka
    ["@kyryloloshka", "таракани", "🪳 Ви з тараканами підходите один одному на 100% 🪳. Гаряча тараканесса з 8-го гуртожитку передає тобі привіт 😘😘😘"],
    ["@kyryloloshka", "рефіл пепсі", "Ви з рефілом не підходите один одному. Краще йди в макдональдс 🍔"],
    // 5	Гаращук Ната Юріївна @o_o_n_n_0_0
    // 6	Герасимчук Денис Миколайович -
    // 7	Глинка Марія Ігорівна @deffkaaaa
    // 8	Горох Богдан Дмитрович @ikeepcalm
    ["@ikeepcalm", "java", "Ви з java не підходите, краще вчи rust"],
    ["@ikeepcalm", "lumios", "Можливо ти хотів сказати evil lumios 😈😈😈"],
    ["@ikeepcalm", "evil lumios", "Ви з Evil Lumios підходите на 🥰 100% 🥰"],
    ["@ikeepcalm", "evil_lumios", "Ви з Evil Lumios підходите на 🥰 100% 🥰"],
    // 9	Дрозд Назарій Скібіденкович @DrozdyW
    // 10	Запара Олег Сергійович @olehzpr
    ["@olehzpr", "test", "test"],
    ["@olehzpr", "@CabianoFaruana", "Ви з Артемчиком підходите на 69% 😏😏😏. Тільки міцна чоловіча дружба, нічого більше"],
    // 11	Зелюк Юлія Максимівна @darpfnis
    // 12	Карпенко Кирило Олександрович @cyrylocar
    // 13	Карпець Даніїл Сергійович @dskarpets
    // 14	Кітаєва Катерина Олександрівна -
    // 15	Клобуков Ярослав Олександрович @yarosirgend
    // 16	Личко Олександр Павлович @lychkoalexander
    ["@lychkoalexander", "@pupkakub", "❤️‍🔥 Ви з Лерою підходите один одному на 10000% ❤️‍🔥"],
    ["@lychkoalexander", "іді нахуй", "сам іді нахуй"],
    // 17	Мазурик Марія Ігорівна @frogpaket
    ["@frogpaket", "суші", "Ви з суші підходите на 100% 🍣, але знай міру, не можна стільки їсти"],
    // 18	Мазченко Валерія Віталіївна @pupkakub
    ["@pupkakub", "@lychkoalexander", "❤️‍🔥 Ви з Саньою підходите один одному на 10000% ❤️‍🔥"],
    // 19	Малій Андрій Володимирович @andremalille
    // 20	Мельник Максим Ігорович @DDDUUUCCK
    // 21	Міхєєв Богдан Вадимович @Bohdan_MI
    // 22	Островицька Юлія Олегівна @ostrjul
    // 23	Пасюра Андрій Юрійович @Pasiura_Andrii
    // 24	Полюхович Максим Едуардович @Marshmallllows
    // 25	Свистун Артем Сергійович @haivop
    // 26	Серкелі Артур Вадимович @Artychanal
    // 27	Цимбал Костянтин Євгенович @golova_le_silrade
    // 28	Юрченко Богдан Валерійович @b1dess
];

fn get_pair<'a>(pair: [&'a str; 3], username: &'a str, query: &'a str) -> Option<&'a str> {
    if pair[0] == username && pair[1] == query {
        return Some(pair[2]);
    }

    None
}
