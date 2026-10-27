use chrono::Local;
use serenity::all::{CreateEmbed, CreateEmbedFooter};

const COLOR: u32 = 0xffe710;

pub fn base() -> CreateEmbed {
    let footer = format!("Vito // {}", Local::now().format("%H:%M:%S"));
    CreateEmbed::new()
        .color(COLOR)
        .footer(CreateEmbedFooter::new(footer))
}
