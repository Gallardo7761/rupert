use serenity::{
    all::{
        CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
        CreateInteractionResponseMessage,
    },
    async_trait,
};

use crate::{
    commands::{CommandResult, VCommand, options, reply},
    embeds,
};

pub struct Avatar;

#[async_trait]
impl VCommand for Avatar {
    fn name(&self) -> &str {
        "avatar"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Muestra el avatar de alguien")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "De quién (por defecto, el tuyo)",
            ))
    }

    async fn run(&self, ctx: &Context, cmd: &CommandInteraction) -> CommandResult {
        let user =
            options::get_user(&cmd.data.options(), "user").unwrap_or_else(|| cmd.user.clone());

        let embed = embeds::base()
            .title(format!("Avatar de {}", user.name))
            .image(user.face());

        reply::send(
            ctx,
            cmd,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await
    }
}
