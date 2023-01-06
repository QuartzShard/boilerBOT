mod commands;
mod config;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::HashMap;

use tracing::{event, Level};

use commands::{CommandResponse};
use config::Config;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let data = ctx.data.write().await;
        let command_map = data.get::<commands::CommandList>().unwrap();
        if let Interaction::ApplicationCommand(command) = interaction {
            event!(Level::DEBUG, "Calling {}", &command.data.name);
            let command_func = match command_map.get(&command.data.name) {
                Some(cmd) => cmd,
                None => {
                    return event!(
                        Level::ERROR,
                        "Command {} not found in hashmap, called in {}",
                        &command.data.name,
                        &command.guild_id.unwrap().0
                    );
                }
            };
            event!(
                Level::DEBUG,
                "Retrieved from hashmap: {}",
                &command_func.name()
            );
            let command_response = match command_func.run(&command.data.options) {
                Ok(response) => match response {
                    CommandResponse::StringResponse(s) => s,
                },
                Err(why) => why.to_string(),
            };
            match command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.content(command_response))
                })
                .await
            {
                Ok(_) => (),
                Err(why) => event!(Level::ERROR, "Can't respond: {}", why),
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        event!(Level::INFO, "Logged in as {}", ready.user.name);
        commands::register_commands(&ctx).await;
        event!(Level::INFO, "Init complete, Bot Ready.");
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Use Config::from_env() if not using a config.toml
    let config = match Config::read_from_file("./config.toml".to_string()) {
        Some(c) => c,
        None => {
            println!("Can't read your config. Make sure you have a `config.toml` in this directory that you can access.");
            return;
        }
    };

    let token = config.discord.token;
    let framework = StandardFramework::new().configure(|c| c.prefix("]"));
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<config::TestGuildID>(config.test.test_guild_id)
        .type_map_insert::<commands::CommandList>(HashMap::default())
        .await
        .expect("Can't make client.");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended {:?}", why));
    });

    tokio::signal::ctrl_c().await.expect("Shutdown Error");
    println!("Recieved Interrupt, stopping");
}
