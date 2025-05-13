use crate::{
    bot::handler::HandlerResult,
    db::{self, stats::transfer_reaction_points},
    models::user::UserModel,
    redis::RedisCache,
    state::State,
};
use teloxide::types::{MessageReactionUpdated, ReactionType, UserId};

pub async fn handle_reaction(msg: MessageReactionUpdated, state: State) -> HandlerResult {
    let new_reaction = find_new_reaction(msg.old_reaction, msg.new_reaction);

    tracing::debug!("New reaction: {:?}", new_reaction);

    let points = get_reaction_points(&new_reaction);
    let sender = msg.user.unwrap();
    let receiver = match state.redis.get_message(msg.chat.id, msg.message_id)?.from {
        Some(user) => user,
        None => return Ok(()),
    };
    let sender = get_user(&state, sender.id).await?;
    let receiver = get_user(&state, receiver.id).await?;

    if sender.id == receiver.id {
        return Ok(());
    }

    transfer_reaction_points(&state.db, sender.id, receiver.id, points).await?;

    Ok(())
}

async fn get_user(state: &State, user_id: UserId) -> anyhow::Result<UserModel> {
    if let Ok(user) = state.redis.get_user(user_id) {
        Ok(user)
    } else {
        let user = db::user::get_user_by_account_id(&state, user_id).await?;
        state.redis.store_user(user.clone())?;
        Ok(user)
    }
}

fn find_new_reaction(old_list: Vec<ReactionType>, new_list: Vec<ReactionType>) -> ReactionType {
    for reaction in new_list {
        if !old_list.contains(&reaction) {
            return reaction;
        }
    }

    return ReactionType::Emoji {
        emoji: "😴".to_string(), // а що б ви робили в цій ситуації?
    };
}

/// Reaction emoji. Currently, it can be one of "👍", "👎", "❤", "🔥",
/// "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉", "🤩",
/// "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳",
/// "❤‍🔥", "🌚", "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨",
/// "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻",
/// "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡",
/// "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘",
/// "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"
fn get_reaction_points(reaction: &ReactionType) -> i32 {
    let reaction = reaction.emoji().unwrap().as_str();
    match reaction {
        "👍" => 5,
        "👎" => -5,
        "❤" => 10,
        "🔥" => 10,
        "🥰" => 5,
        "👏" => 5,
        "😁" => 5,
        "🤔" => 5,
        "🤯" => 5,
        "😱" => 5,
        "🤬" => -5,
        "😢" => -5,
        "🎉" => 5,
        "🤩" => 5,
        "🤮" => -10,
        "💩" => -10, // за цю реакцію рейтинг зміматиметься тому хто її ставить
        "🙏" => 5,
        "👌" => 5,
        "🕊" => 5,
        "🤡" => -5,
        "🥱" => 0,
        "🥴" => 0,
        "😍" => 10,
        "🐳" => -50, // пранк 😈
        "❤‍🔥" => 5,
        "🌚" => 5,
        "🌭" => 5,
        "💯" => 5, // а могло б буть 100 😭😭😭
        "🤣" => 5,
        "⚡" => 5,
        "🍌" => 5,
        "🏆" => 100000, // фул ахуй
        "💔" => -5,
        "🤨" => 5,
        "😐" => 5,
        "🍓" => 5,
        "🍾" => 5,
        "💋" => 5,
        "🖕" => -10,
        "😈" => 5,
        "😴" => 0, // використовуватиму цю як дефолтну
        "😭" => -5,
        "🤓" => 5,
        "👻" => 5,
        "👨‍💻" => -1, // це для кирилокара
        "👀" => 5,
        "🎃" => 15, // личко іді нахуй
        "🙈" => 5,
        "😇" => 5,
        "😨" => 0,
        "🤝" => 5,
        "✍" => 5,
        "🤗" => 5,
        "🫡" => 5,
        "🎅" => 5,
        "🎄" => 5,
        "☃" => 5,
        "💅" => -5, // слейчики опустять одне одного
        "🤪" => 5,
        "🗿" => 2,
        "🆒" => 10,
        "💘" => 10,
        "🙉" => 5,
        "🦄" => 5,
        "😘" => 5,
        "💊" => 5,
        "🙊" => 5,
        "😎" => 5,
        "👾" => 3, // трошки занерфить
        "🤷‍♂" => 0,
        "🤷" => 0,
        "🤷‍♀" => 0,
        "😡" => -2,
        _ => 15, // бонус донатерам
    }
}
