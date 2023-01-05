use crate::commands::*;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub struct PingCommand;
impl PingCommand {
    pub fn new(self) -> Self {
        self
    }
}
impl RegisterableAsSlashCommand for PingCommand {
    fn new() -> Self
    where
        Self: Sized,
    {
        PingCommand {}
    }

    fn name(&self) -> String {
        "ping".to_string()
    }
    fn about(&self) -> String {
        "Pong!".to_string()
    }
    fn run(&self, _options: &[CommandDataOption]) -> CommandResult {
        Ok(CommandResponse::StringResponse("Pong!".to_string()))
    }
}
