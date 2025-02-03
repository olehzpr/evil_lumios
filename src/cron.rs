use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{bot::timetable::schedule::timetable_notifications, state::State};

pub async fn cron_loop(state: State) -> anyhow::Result<()> {
    let scheduler = JobScheduler::new().await?;

    let notifications = Job::new_async("0 * * * * *", move |_uuid, _lock| {
        Box::pin(timetable_notifications(state.clone()))
    })?;

    scheduler.add(notifications).await?;

    scheduler.start().await?;

    Ok(())
}
