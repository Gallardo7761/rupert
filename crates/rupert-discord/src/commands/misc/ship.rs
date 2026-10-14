use rupert_core::fun::ship_percentage;
use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
        CreateInteractionResponseMessage, Mentionable,
    },
    async_trait,
};

use crate::{
    commands::{CommandResult, RCommand, options, reply},
    embeds,
};

pub struct Ship;

fn progress_bar(percent: u8) -> String {
    let filled = percent as usize * 10 / 100;
    format!("{}{}", "█".repeat(filled), "░".repeat(10 - filled))
}

#[async_trait]
impl RCommand for Ship {
    fn name(&self) -> &str {
        "ship"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Shipea a dos personas A, B")
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "a", "Primera persona")
                    .required(true),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::User, "b", "Segunda persona")
                    .required(true),
            )
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let options = cmd.data.options();

        let message = match (
            options::get_user(&options, "a"),
            options::get_user(&options, "b"),
        ) {
            (Some(a), Some(b)) => {
                let pct = ship_percentage(a.id.get(), b.id.get());
                let embed = embeds::base().title("💘 Ship").description(format!(
                    "{} x {}\n\n{} **{pct}%**",
                    a.mention(),
                    b.mention(),
                    progress_bar(pct)
                ));
                CreateInteractionResponseMessage::new().embed(embed)
            }
            _ => reply::error_message("Necesito dos personas"),
        };

        reply::send(ctx, cmd, message).await
    }
}
