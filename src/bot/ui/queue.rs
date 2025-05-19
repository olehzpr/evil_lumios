use crate::{
    bot::ui::utils::adapt_for_markdown,
    models::queue::{QueueModel, QueueUserWithUserModel},
};

pub enum QueueType {
    Regular,
    Mixed,
    Priority,
}

pub fn title(name: &String) -> String {
    format!("\\>\\>\\> *{}* <<<\n\n", adapt_for_markdown(&name),)
}

pub fn regular_queue(queue: &QueueModel, users: Vec<QueueUserWithUserModel>) -> String {
    let mut message = title(&queue.title);
    let required_characters = users.len().to_string().len();
    for (i, user) in users.iter().enumerate() {
        message.push_str(&format!(
            "{:width$} \\- {} \\(@{}\\)\n",
            i + 1,
            adapt_for_markdown(&user.name),
            adapt_for_markdown(&user.username),
            width = required_characters
        ));
    }
    message
}

pub fn priority_queue(queue: &QueueModel, users: Vec<QueueUserWithUserModel>) -> String {
    let mut message = title(&queue.title);
    let required_characters = users.len().to_string().len();
    for (i, user) in users.iter().enumerate() {
        let index = if user.is_frozen.unwrap_or(false) {
            "❄️".to_string()
        } else {
            (i + 1).to_string()
        };
        let priority = match user.priority {
            Some(priority) => priority.to_string(),
            None => "0".to_string(),
        };
        message.push_str(&format!(
            "{:width$} \\[{}\\] \\- {} \\(@{}\\)\n",
            index,
            priority,
            adapt_for_markdown(&user.name),
            adapt_for_markdown(&user.username),
            width = required_characters
        ));
    }
    message
}

pub fn notification(user: &QueueUserWithUserModel, queue: &QueueModel) -> String {
    format!(
        "{} – твоя черга відповідати в черзі '{}'",
        adapt_for_markdown(&user.name),
        adapt_for_markdown(&queue.title)
    )
}
