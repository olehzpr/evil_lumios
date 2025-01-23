use teloxide::prelude::ResponseResult;

pub mod chat;
pub mod event_handler;
pub mod externsions;
pub mod general;
pub mod handler;
pub mod inline;
pub mod queues;
pub mod stats;
pub mod timetable;
pub mod ui;
pub mod users;
pub mod utils;

pub type Result = ResponseResult<()>;
