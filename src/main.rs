mod discord;
mod api;
mod database;

use std::sync::Arc;
use std::fs;
use std::process::exit;
use std::collections::HashSet;
use toml;

use serde::Deserialize;

use poise::serenity_prelude as serenity;

pub struct Data {
    pub database: Arc<database::Database>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub discord: discord::config::DiscordConfig,
    pub api: api::config::APIConfig,
    pub database: database::config::DatabaseConfig,
}

#[tokio::main]
async fn main() {
    // Read the config file
    let contents = match fs::read_to_string("config.toml") {
        Ok(config) => config,
        Err(e) => {
            println!("Failed to read config file: {}", e);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(e) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from config.toml `{}`", e);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    // Setup the discord bot
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.discord.prefix.clone()),
                edit_tracker: Some(poise::EditTracker::for_timespan(std::time::Duration::from_secs(config.discord.edit_track_timespan))),
                ..Default::default()
            },
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(discord::event_handler::event_handler(_ctx, event, _framework, _data))
            },
            pre_command: |ctx| Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            }),
            post_command: |ctx| Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            }),
            ..Default::default()
        })
        .token(config.discord.token.clone())
        .intents(serenity::GatewayIntents::all())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                let address = config.api.address().clone();

                // Register commands globally
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // Setup database
                let database = database::Database::new(config.database.clone()).await;
                let database = match database {
                    Ok(d) => {
                        println!("Connected to database!");
                        Arc::new(d)
                    }
                    Err(e) => {
                        eprintln!("Failed to connect to database: {}", e);
                        exit(1);
                    }
                };

                // Setup API
                let app_state = api::models::AppState {
                    discord: ctx.http.clone(),
                    database: database.clone(),
                    config: config.clone(),
                };
                let routes = api::combined_routes(app_state); // Assuming app_state contains the necessary shared state
                
                // Setup commands
                let commands = vec![discord::commands::ping(), discord::commands::link(), discord::commands::steamid()];
                let current_command_names: HashSet<String> = commands.iter()
                    .map(|cmd| cmd.name.to_string())
                    .collect();

                // Delete global commands that are not present in current setup
                let global_commands = ctx.http.get_global_application_commands().await?;
                for command in global_commands {
                    if !current_command_names.contains(&command.name) {
                        ctx.http.delete_global_application_command(*command.id.as_u64()).await?;
                    }
                }
                // Delete guild-specific commands that are not present in current setup
                let guild_id = poise::serenity_prelude::GuildId(config.discord.guild);
                let guild_commands = ctx.http.get_guild_application_commands(*guild_id.as_u64()).await?;
                for command in guild_commands {
                    if !current_command_names.contains(&command.name) {
                        ctx.http.delete_guild_application_command(*guild_id.as_u64(), *command.id.as_u64()).await?;
                    }
                }
                match poise::builtins::register_in_guild(&ctx.http, &commands, guild_id).await {
                    Ok(_) => println!("Registered commands in guild!"),
                    Err(e) => {
                        println!("Failed to register commands in guild: {}", e);
                        exit(1);
                    }
                }

                // Start web API
                tokio::spawn(async move {
                    warp::serve(routes).run(address).await;
                });
                println!("API started on {}", address);

                Ok(Data {database})
            })
        });

    // Let's run
    println!("Starting discord bot!");
    framework.run().await.expect("Failed to start discord bot!");

}
