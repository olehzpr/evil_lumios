use teloxide::prelude::ResponseResult;

pub mod callbacks;
pub mod event_handler;
pub mod general;
pub mod handler;
pub mod inline;
pub mod queues;
pub mod stats;
pub mod timetable;
pub mod ui;
pub mod utils;

pub type Result = ResponseResult<()>;
