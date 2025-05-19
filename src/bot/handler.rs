use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateFilterExt, UpdateHandler,
    },
    dptree,
    types::Update,
};

use crate::{
    bot::{
        callbacks::handle_callback,
        general,
        inline::answer_inline_query,
        queues, stats,
        timetable::{self, external::receive_timetable_entry_link},
    },
    config::{commands::Command, state::StateMachine},
    repositories::{
        chat_repository::create_chat_if_not_exists, user_repository::create_user_if_not_exists,
    },
    state::State,
};
use dptree::case;

use super::timetable::external::receive_timetable_entry_link_from_message;

pub type HandlerResult = anyhow::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let reaction_handler =
        Update::filter_message_reaction_updated().endpoint(stats::reactions::handle_reaction);

    let callback_handler = Update::filter_callback_query().endpoint(handle_callback);

    let command_handler = teloxide::filter_command::<Command, _>()
        // general
        .branch(
            case![StateMachine::Start]
                .branch(case![Command::Help].endpoint(general::commands::help))
                .branch(case![Command::Start].endpoint(general::commands::start)),
        )
        .branch(case![Command::Help].endpoint(general::commands::help))
        // timetable
        .branch(case![Command::Week].endpoint(timetable::commands::week))
        .branch(case![Command::Today].endpoint(timetable::commands::today))
        .branch(case![Command::Tomorrow].endpoint(timetable::commands::tomorrow))
        .branch(case![Command::Now].endpoint(timetable::commands::now))
        .branch(case![Command::Next].endpoint(timetable::commands::next))
        .branch(case![Command::Import].endpoint(timetable::commands::import))
        .branch(case![Command::EditTimetable].endpoint(timetable::commands::edit_timetable))
        // queues
        .branch(case![Command::Queue].endpoint(queues::commands::queue))
        .branch(case![Command::Mixed].endpoint(queues::commands::mixed))
        .branch(case![Command::PriorityQueue].endpoint(queues::commands::priority_queue))
        // stats
        .branch(case![Command::Stats].endpoint(stats::commands::stats))
        .branch(case![Command::Casino].endpoint(stats::commands::casino))
        .branch(case![Command::Me].endpoint(stats::commands::me))
        .branch(case![Command::Wheel].endpoint(stats::commands::wheel))
        .branch(case![Command::Gamble].endpoint(stats::commands::gamble))
        .branch(case![Command::GambleAll].endpoint(stats::commands::gamble_all));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(
            case![StateMachine::ReceiveEditTimetableEntryFromMessage {
                id,
                chat_id,
                message_id,
            }]
            .branch(dptree::endpoint(receive_timetable_entry_link_from_message)),
        )
        .branch(
            case![StateMachine::ReceiveEditTimetableEntry { id }]
                .endpoint(receive_timetable_entry_link),
        )
        .branch(dptree::endpoint(general::message_handler::handler));

    let inline_handler = Update::filter_inline_query().endpoint(answer_inline_query);

    dptree::entry()
        .filter_map_async(preprocess_update)
        .branch(inline_handler)
        .branch(
            dialogue::enter::<Update, InMemStorage<StateMachine>, StateMachine, _>()
                .branch(message_handler)
                .branch(reaction_handler),
        )
        .branch(callback_handler)
}

async fn preprocess_update(update: Update, state: State) -> Option<(Update, State)> {
    if let (Some(chat), Some(user)) = (update.chat(), update.from()) {
        if let Err(err) = create_chat_if_not_exists(&state, chat).await {
            tracing::error!("Failed to create chat: {:?}", err);
        }
        if let Err(err) = create_user_if_not_exists(&state, user, chat).await {
            tracing::error!("Failed to create user: {:?}", err);
        }
    }

    Some((update, state))
}
