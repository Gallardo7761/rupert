pub mod misc;

use serenity::all::{CommandInteraction, Context, CreateCommand};
use serenity::async_trait;
use std::error::Error;

pub type CommandResult = Result<(), Box<dyn Error + Send + Sync>>;

#[async_trait]
pub trait RCommand: Send + Sync {
    fn name(&self) -> &str;
    fn register(&self) -> CreateCommand;
    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult;
}

pub fn get_all() -> Vec<Box<dyn RCommand>> {
    vec![Box::new(misc::ping::Ping)]
}
