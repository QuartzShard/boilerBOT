use crate::commands::*;
use serenity::model::prelude::interaction::{
    application_command::CommandDataOption, InteractionResponseType,
};

pub struct PingCommand;
impl RegisterableAsSlashCommand for PingCommand {
    fn name(&self) -> String {
        "ping".to_string()
    }
    fn about(&self) -> String {
        "Pong!".to_string()
    }
    fn run(&self, _options: &[CommandDataOption]) -> CommandResult {
        let mut response = CreateInteractionResponse::default();
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|res| res.content("Pong!"));
        Ok(response)
    }
}
