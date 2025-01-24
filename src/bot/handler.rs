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
        general,
        inline::answer_inline_query,
        queues, stats,
        timetable::{self, external::receive_timetable_entry_link},
    },
    config::{commands::Command, state::StateMachine},
};

pub fn handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let reaction_handler =
        Update::filter_message_reaction_updated().endpoint(stats::reactions::handle_reaction);

    let catch_all_handler = dptree::endpoint(|update: Update| async move {
        tracing::info!("Unhandled update: {:?}", update.kind);
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    });

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
        // stats
        .branch(case![Command::Stats].endpoint(stats::commands::stats))
        .branch(case![Command::Me].endpoint(stats::commands::me))
        .branch(case![Command::Wheel].endpoint(stats::commands::wheel))
        .branch(case![Command::Gamble].endpoint(stats::commands::gamble))
        .branch(case![Command::GambleAll].endpoint(stats::commands::gamble_all));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(
            case![StateMachine::ReceiveEditTimetableEntry { id }]
                .endpoint(receive_timetable_entry_link),
        )
        .branch(dptree::endpoint(|| async { Ok(()) })); // ignore all other messages

    let inline_handler = Update::filter_inline_query().endpoint(answer_inline_query);

    dptree::entry()
        .branch(inline_handler)
        .branch(
            dialogue::enter::<Update, InMemStorage<StateMachine>, StateMachine, _>()
                .branch(message_handler)
                .branch(reaction_handler),
        )
        .branch(catch_all_handler)
}
