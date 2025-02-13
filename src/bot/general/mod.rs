use teloxide::types::{ChatId, MessageId};

pub mod commands;
pub mod message_handler;

#[derive(Debug)]
pub enum StartCommand {
    Start,
    EditTimetable {
        entry_id: i32,
    },
    EditTimetableFromMessage {
        entry_id: i32,
        chat_id: ChatId,
        message_id: MessageId,
    },
    Casino,
}

impl StartCommand {
    pub fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('_').collect();
        println!("{:?}", parts);
        match parts.as_slice() {
            ["/start edit-timetable", entry_id] => {
                let entry_id = entry_id.parse().ok()?;
                Some(Self::EditTimetable { entry_id })
            }
            ["/start edit-timetable-from-message", entry_id, chat_id, message_id] => {
                let entry_id = entry_id.parse().ok()?;
                let chat_id = ChatId(chat_id.parse().ok()?);
                let message_id = MessageId(message_id.parse().ok()?);
                Some(Self::EditTimetableFromMessage {
                    entry_id,
                    chat_id,
                    message_id,
                })
            }
            ["/start casino"] => Some(Self::Casino),
            _ => None,
        }
    }
}
