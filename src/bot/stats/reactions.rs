use crate::{bot::handler::HandlerResult, db::StateWithConnection, state::State};
use teloxide::types::{Me, MessageReactionUpdated, ReactionType, Update, User};

pub async fn handle_reaction(
    message_reaction: MessageReactionUpdated,
    state: State,
) -> HandlerResult {
    let new_reaction =
        find_new_reaction(message_reaction.old_reaction, message_reaction.new_reaction);
    tracing::debug!("New reaction: {:?}", new_reaction);
    let points = get_reaction_points(&new_reaction);
    let conn = &mut state.conn().await;
    let user_that_gave_reaction = message_reaction.user;
    let message_id = message_reaction.message_id;

    // tracing::debug!(
    //     "User that gave reaction: {:?}, user that received reaction: {:?}",
    //     user_that_gave_reaction,
    //     user_that_received_reaction
    // );
    if new_reaction.emoji().unwrap().as_str() == "💩" {
        return Ok(());
    }
    Ok(())
}

fn find_new_reaction(old_list: Vec<ReactionType>, new_list: Vec<ReactionType>) -> ReactionType {
    for reaction in new_list {
        if !old_list.contains(&reaction) {
            return reaction;
        }
    }

    return ReactionType::Emoji {
        emoji: "😴".to_string(),
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
