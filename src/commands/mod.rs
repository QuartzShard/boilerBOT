use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::Command;
use serenity::model::{id::GuildId, prelude::interaction::application_command::CommandDataOption};
use serenity::prelude::{Context, TypeMapKey};
use std::collections::HashMap;
use std::fmt;
use std::result::Result;

use tracing::{event, info, Level};

// Error handling for commands
pub type CommandResult = Result<CommandResponse<String>, CommandError>;
pub enum CommandError {
    FailedToSend,
    FailedToExecute,
}
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = match self {
            CommandError::FailedToSend => "Failed to send",
            CommandError::FailedToExecute => "Failed to execute",
        };
        write!(f, "{}", formatted)
    }
}
pub enum CommandResponse<String> {
    StringResponse(String),
}

// Command must implement RegisterableAsSlashCommand
pub trait RegisterableAsSlashCommand {
    fn new() -> Self
    where
        Self: Sized;
    fn name(&self) -> String;
    fn about(&self) -> String;
    fn run(&self, options: &[CommandDataOption]) -> CommandResult;
    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command.name(self.name()).description(self.about())
    }
}

pub struct CommandList;
impl TypeMapKey for CommandList {
    type Value = HashMap<String, CommandBox>;
}
pub type CommandBox = Box<dyn RegisterableAsSlashCommand + Sync + Send>;

macro_rules! register {
    (($struct: expr, $commands: expr, $command_map: expr)) => {
        $commands.create_application_command(|command| {
            let name = $struct.new();
            name.register(command);
            info!("Registered command {}.", name.name());
            $command_map.insert(name.name(), Box::new(name) as CommandBox);
            command
        })
    };
    (($struct: expr, $commands: expr, $command_map: expr), $($structs: expr, $commandss: expr, $command_maps: expr),+) => {
        register!($struct, $commands, $command_map);
        register!(structs, $commandss, $command_maps),+
    };
}
// This is where you should register your commands with the bot.
// You need to mod <name> the command for the struct to be in scope.
// Then, inside of either of the |commands| {} blocks, you can use the register! macro to register
// the command
mod ping;
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
            register!((ping::PingCommand, commands, command_map));
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
