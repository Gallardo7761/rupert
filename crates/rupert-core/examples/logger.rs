use rupert_core::logging::{RLogger, RLoggerContext};

fn main() {
    let core = RLogger::new(RLoggerContext::Core);
    let discord = RLogger::new(RLoggerContext::Discord);
    let telegram = RLogger::new(RLoggerContext::Telegram);

    core.info("esto es un info");
    core.warn("esto es un warn");

    discord.info("bot listo");
    telegram.info("polling arrancado");

    // un error de verdad, pa que haya algo que pintar en rojo
    if let Err(e) = "abc".parse::<i32>() {
        discord.error("no pude parsear el número", &e);
    }
}
