pub mod commands;
pub mod external;
pub mod schedule;
pub mod utils;

pub type HandlerResult = anyhow::Result<(), Box<dyn std::error::Error + Send + Sync>>;
