pub mod admin;
pub mod math;
pub mod misc;
pub mod options;
pub mod reply;

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateCommandOption, Permissions, ResolvedOption,
};
use serenity::async_trait;
use std::error::Error;

pub type CommandResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

#[async_trait]
pub trait RCommand: Send + Sync {
    fn name(&self) -> &str;
    fn permission(&self) -> Option<Permissions> {
        None
    }
    fn register(&self) -> CreateCommand;
    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult;
}

#[async_trait]
pub trait RSubcommand: Send + Sync {
    fn name(&self) -> &str;
    fn permission(&self) -> Permissions;
    /// What the bot itself needs in the channel. By default, the same as the user.
    fn bot_permission(&self) -> Permissions {
        self.permission()
    }
    fn register(&self) -> CreateCommandOption;
    async fn run(
        &self,
        ctx: &Context,
        cmd: &CommandInteraction,
        options: &[ResolvedOption<'_>],
    ) -> CommandResult;
}

pub fn get_all() -> Vec<Box<dyn RCommand>> {
    vec![
        // admin
        Box::new(admin::Admin::new()),
        Box::new(admin::say::Say),
        Box::new(admin::poll::Poll),
        // math
        Box::new(math::prime::Prime),
        Box::new(math::calc::Calc),
        // misc
        Box::new(misc::ping::Ping),
        Box::new(misc::dice::Dice),
        Box::new(misc::coin::CoinFlip),
        Box::new(misc::banana::Banana),
        Box::new(misc::avatar::Avatar),
        Box::new(misc::ship::Ship),
    ]
}
