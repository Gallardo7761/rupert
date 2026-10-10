use serenity::all::{CreateEmbed, CreateEmbedFooter};

const COLOR: u32 = 0xf74c00;

pub fn base() -> CreateEmbed {
    CreateEmbed::new()
        .color(COLOR)
        .footer(CreateEmbedFooter::new("Rupert"))
}