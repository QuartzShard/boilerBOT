use crate::commands::*;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::interaction::InteractionResponseType;
pub struct TestCommand;
impl RegisterableAsSlashCommand for TestCommand {
    fn name(&self) -> String {
        "test2".to_string()
    }
    fn about(&self) -> String {
        "A test command. May do strange things".to_string()
    }
    fn options<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        tracing::debug!("Setting options for TestCommand...");
        command
            .create_option(|option| {
                option
                    .name("Other")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("Val")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    }
    fn run(&self, options: &[CommandDataOption]) -> CommandResult {
        info!("{:?}", options);
        let args = self.map_opts(options)?;
        let name = args
            .get("val")
            .ok_or(CommandError::ArgumentError(String::from(
                "Missing argument",
            )))?
            .as_str()
            .ok_or(CommandError::ArgumentError(String::from(
                "Missing argument",
            )))?
            .to_string();
        let mut response = CreateInteractionResponse::default();
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|res| res.embed(|emb| emb.title(name)));
        Ok(response)
    }
}
