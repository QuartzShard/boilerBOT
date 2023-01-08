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
                    .name("stringarg")
                    .description("String")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("userarg")
                    .description("User")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("rolearg")
                    .description("Role")
                    .kind(CommandOptionType::Role)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("intarg")
                    .description("Integer")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("boolarg")
                    .description("Bool")
                    .kind(CommandOptionType::Boolean)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("mentionablearg")
                    .description("tag")
                    .kind(CommandOptionType::Mentionable)
                    .required(true)
            })
    }
    fn run(&self, options: &[CommandDataOption]) -> CommandResult {
        let mut args: HashMap<String, &CommandDataOptionValue> = HashMap::new();
        let args = self.map_opts(options, &mut args)?;
        let name = match args
            .get("mentionablearg")
            .ok_or(CommandError::ArgumentError("no arg?".to_string()))?
        {
            CommandDataOptionValue::Role(r) => r.name.clone(),
            _ => "No role".to_string(),
        };
        let mut response = CreateInteractionResponse::default();
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|res| res.embed(|emb| emb.title(name)));
        Ok(response)
    }
}
