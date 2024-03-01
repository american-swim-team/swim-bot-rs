mod discord;
mod api;
mod database;
mod config;

use std::sync::Arc;
use std::fs;
use std::process::exit;
use fern;
use log::LevelFilter;
use toml;

use serde::Deserialize;

use poise::serenity_prelude as serenity;

#[derive(Debug, Clone)]
pub struct Data {
    pub database: Arc<database::Database>,
    pub config: Config,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub log: config::Log,
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

    // Setup logging
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(config.log.level)
        .level_for("tracing::span", LevelFilter::Off)
        .level_for("serenity::gateway::shard", LevelFilter::Off)
        .chain(if config.log.stdout {
            Box::new(std::io::stdout()) as Box<dyn std::io::Write + Send>
        } else {
            Box::new(std::io::sink()) // If stdout is false, don't log to stdout
        })
        .chain(fern::log_file(&config.log.file_output).unwrap())
        .apply().unwrap();

    log::debug!("Setup logging...");

    let _data = Arc::new(Data {
        database: Arc::new(database::Database::new(config.database.clone()).await.unwrap()),
        config: config.clone(),
    });

    let options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(config.discord.prefix.clone().into()),
            // edit_tracker: Some(poise::EditTracker::for_timespan(std::time::Duration::from_secs(config.discord.edit_track_timespan))),
            ..Default::default()
        },
        commands: vec![discord::commands::register(), discord::commands::help(), discord::commands::ping(), discord::commands::link(), discord::commands::steamid(), discord::commands::score()],
        event_handler: |ctx, event| { // Modified the closure to take only two arguments
            Box::pin(discord::event_handler::event_handler(ctx, event)) // Removed the unnecessary arguments
        },
        pre_command: |ctx| Box::pin(async move {
            log::debug!("Executing command {}...", ctx.command().qualified_name);
        }),
        post_command: |ctx| Box::pin(async move {
            log::info!("Executed command {}!", ctx.command().qualified_name);
        }),
        ..Default::default()
    };

    let mut client = serenity::ClientBuilder::new(&config.discord.token, serenity::GatewayIntents::all())
        .framework(poise::Framework::new(options))
        .data(_data.clone())
        .await
        .expect("Failed to create client");

    let (http, cache) = (client.http.clone(), client.cache.clone());

    let app_state = api::models::AppState {
        http,
        cache,
        database: _data.database.clone(),
        config: config.clone(),
    };

    let routes = api::combined_routes(app_state); // Assuming app_state contains the necessary shared state

    // Start web API
    tokio::spawn(async move {
        warp::serve(routes).run(config.api.address()).await;
    });
    log::debug!("Started web API!");
    log::debug!("Discord, database and api setup complete!");
        
    // Let's run
    log::info!("Starting bot...");
    client.start().await.unwrap();

}
