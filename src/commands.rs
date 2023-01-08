#![allow(dead_code)]
use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::json::Value;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::{id::GuildId, prelude::interaction::application_command::CommandDataOption};
use serenity::prelude::{Context, TypeMapKey};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::result::Result;

use tracing::{event, info, Level};
// Error handling for commands
pub type CommandResult<'a> = Result<CreateInteractionResponse<'a>, CommandError>;
#[derive(Debug)]
pub enum CommandError {
    FailedToSend(String),
    FailedToExecute(String),
    ArgumentError(String),
}
impl Error for CommandError {
    fn description(&self) -> &str {
        match &*self {
            Self::FailedToSend(s) => &s,
            Self::FailedToExecute(s) => &s,
            Self::ArgumentError(s) => &s,
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = match self {
            CommandError::FailedToSend(s) => s,
            CommandError::FailedToExecute(s) => s,
            CommandError::ArgumentError(s) => s,
        };
        write!(f, "{}", formatted)
    }
}

// Command must implement RegisterableAsSlashCommand
pub trait RegisterableAsSlashCommand {
    fn name(&self) -> String;
    fn about(&self) -> String;
    fn options<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        command
    }
    fn run(&self, options: &[CommandDataOption]) -> CommandResult;
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command.name(self.name()).description(self.about());
        self.options(command);
        command
    }
    fn map_opts<'a>(
        &'a self,
        options: &'a [CommandDataOption],
        args: &'a mut HashMap<String, &'a CommandDataOptionValue>,
    ) -> Result<&'a mut HashMap<String, &'a CommandDataOptionValue>, CommandError> {
        for option in options {
            args.insert(
                option.name.clone(),
                option
                    .resolved
                    .as_ref()
                    .ok_or(CommandError::ArgumentError("Value not found".to_string()))?,
            );
        }
        Ok(args)
    }
}

pub struct CommandList;
impl TypeMapKey for CommandList {
    type Value = HashMap<String, CommandBox>;
}
pub type CommandBox = Box<dyn RegisterableAsSlashCommand + Sync + Send>;

// Macros for command registration
macro_rules! register {
    (($struct: expr, $commands: expr, $command_map: expr)) => {
        $commands.create_application_command(|command| {
            let name = $struct;
            name.register(command);
            info!("Registered command {}.", name.name());
            $command_map.insert(name.name(), Box::new(name) as CommandBox);
            command
        })
    };
    ( ($struct: expr, $commands: expr, $command_map: expr), $( ($structs: expr, $commandss: expr, $command_maps: expr) ),+ ) => {
        register!(($struct, $commands, $command_map));
        register!($(($structs, $commandss, $command_maps)),+)
    };
}
// This is where you should register your commands with the bot.
// You need to mod <name> the command for the struct to be in scope.
// Then, inside of either of the |commands| {} blocks, you can use the register! macro to register
// the command
mod ping;
mod test_command;
pub async fn register_commands(ctx: &Context) {
    event!(Level::DEBUG, "Registering commands");
    let mut data = ctx.data.write().await;
    event!(Level::DEBUG, "Got data");
    let test_guild_id = data.get::<crate::config::TestGuildID>().unwrap_or(&0u64);
    let test_guild_id: u64 = test_guild_id.clone();
    event!(Level::DEBUG, "Got guildID {}", &test_guild_id);
    let command_map = data
        .get_mut::<CommandList>()
        .expect("Can't find commandlist!");
    event!(Level::DEBUG, "Got commandlist");
    // Not zero when bound to a guild for testing
    if test_guild_id != 0u64 {
        let guild_id = GuildId(test_guild_id);
        let _guild_commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            //register!((name::StructName, commands, command_map))
            register!(
                (ping::PingCommand, commands, command_map),
                (test_command::TestCommand, commands, command_map)
            );
            commands
        })
        .await;
    } else {
        let _global_commands = Command::set_global_application_commands(&ctx.http, |commands| {
            //register!((name::StructName, commands, command_map))
            commands
        });
    }
}
