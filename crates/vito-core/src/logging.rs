use chrono::Local;
use colored::{ColoredString, Colorize};
use std::error::Error;

const CTX_WIDTH: usize = 4; // TELEGRAM
const LEVEL_WIDTH: usize = 5; // ERROR

#[derive(Debug, Clone, Copy)]
pub enum VLoggerContext {
    Core,
    Discord,
    Telegram,
}

pub struct VLogger {
    ctx: VLoggerContext,
}

impl VLogger {
    pub fn new(ctx: VLoggerContext) -> Self {
        Self { ctx }
    }

    pub fn info(&self, message: &str) {
        let level = format!("{:<w$}", "INFO", w = LEVEL_WIDTH).green().bold();
        println!("{}", self.line(level, message));
    }

    pub fn warn(&self, message: &str) {
        let level = format!("{:<w$}", "WARN", w = LEVEL_WIDTH).yellow().bold();
        println!("{}", self.line(level, message));
    }

    pub fn debug(&self, message: &str) {
        let level = format!("{:<w$}", "DEBUG", w = LEVEL_WIDTH)
            .bright_purple()
            .bold();
        println!("{}", self.line(level, message));
    }

    pub fn error(&self, message: &str, error: &dyn Error) {
        let level = format!("{:<w$}", "ERROR", w = LEVEL_WIDTH).red().bold();
        eprintln!(
            "{}\n  {} {}",
            self.line(level, message),
            "└─".bright_black(),
            error.to_string().red()
        );
    }

    fn line(&self, level: ColoredString, message: &str) -> String {
        format!(
            "({}) {} {} {}: {}",
            Local::now().format("%H:%M:%S").to_string().bright_black(),
            self.ctx_tag(),
            "|".to_string().bright_black(),
            level,
            message
        )
    }

    fn ctx_tag(&self) -> ColoredString {
        let name = &format!(
            "{:<w$}",
            format!("{:?}", self.ctx).to_uppercase(),
            w = CTX_WIDTH
        )[..4];

        match self.ctx {
            VLoggerContext::Core => name.magenta(),
            VLoggerContext::Discord => name.blue(),
            VLoggerContext::Telegram => name.cyan(),
        }
    }
}
