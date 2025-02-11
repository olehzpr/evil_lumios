pub mod commands;
pub mod message_handler;

#[derive(Debug)]
pub enum StartCommand {
    Start,
    EditTimetable { entry_id: i32 },
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
            ["/start casino"] => Some(Self::Casino),
            _ => None,
        }
    }
}
