use chrono::Local;
use serenity::all::{CreateEmbed, CreateEmbedFooter};

const COLOR: u32 = 0xf74c00;

pub fn base() -> CreateEmbed {
    let footer = format!("Rupert // {}", Local::now().format("%H:%M:%S"));
    CreateEmbed::new()
        .color(COLOR)
        .footer(CreateEmbedFooter::new(footer))
}
