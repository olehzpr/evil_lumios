use crate::models::{
    queue::{QueueModel, QueueUserModel, QueueUserWithUserModel},
    user::UserModel,
};

pub enum QueueType {
    Regular,
    Mixed,
    Priority,
}

pub fn get_emoji(queue_type: QueueType) -> &'static str {
    match queue_type {
        QueueType::Regular => "🟢",
        QueueType::Mixed => "🟡",
        QueueType::Priority => "🔴",
    }
}

pub fn start_message(name: String, queue_type: QueueType) -> String {
    let e = get_emoji(queue_type);
    format!(
        "{} *{}* {}\n\nНатисність _Join 📡_, щоб приєднатись",
        e, name, e
    )
}

pub fn regular_queue(queue: &QueueModel, users: Vec<QueueUserWithUserModel>) -> String {
    let queue_type = if queue.is_mixed.is_some() {
        QueueType::Mixed
    } else {
        QueueType::Regular
    };
    let e = get_emoji(queue_type);
    let mut message = format!("{} *{}* {}\n\n", e, queue.title, e);
    let required_characters = users.len().to_string().len();
    for (i, user) in users.iter().enumerate() {
        message.push_str(&format!(
            "{:width$} \\- {} \\(@{}\\)\n",
            i + 1,
            user.name,
            user.username,
            width = required_characters
        ));
    }
    message
}

pub fn priority_queue(
    queue: &QueueModel,
    queue_users: Vec<QueueUserModel>,
    users: Vec<UserModel>,
) -> String {
    let e = get_emoji(QueueType::Priority);
    let mut message = format!("{} *{}* {}\n\n", e, queue.title, e);
    let required_characters = users.len().to_string().len();
    for (i, user) in users.iter().enumerate() {
        let user_info = queue_users.iter().find(|u| u.user_id == user.id);
        if user_info.is_none() {
            continue;
        }
        let user_info = user_info.unwrap();
        let index = if user_info.is_frozen.unwrap_or(false) {
            "❄️".to_string()
        } else {
            (i + 1).to_string()
        };
        let priority = match user_info.priority {
            Some(priority) => priority.to_string(),
            None => "*".to_string(),
        };
        message.push_str(&format!(
            "{:width$} |{}| \\- {} \\(@{}\\)\n",
            index,
            priority,
            user.name,
            user.username,
            width = required_characters
        ));
    }
    message
}

pub fn notification(user: &QueueUserWithUserModel, queue: &QueueModel) -> String {
    format!(
        "{} – твоя черга відповідати в черзі '{}'",
        user.name, queue.title
    )
}
