use poise::serenity_prelude as serenity;
use crate::{Context, Error};
use poise::command;

use poise::reply;

async fn reply(ctx: &Context<'_>, message: String) -> Result<(), Error> {
    Ok(_ = ctx.send(reply::CreateReply::default()
            .embed(poise::serenity_prelude::CreateEmbed::default()
                .title("swim> bot")
                .description(message)
                .color(serenity::Colour::from_rgb(255, 255, 255))
            ).ephemeral(true)
        ).await?)
}

/// Responds with pong
#[command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

/// Admin menu to register application commands
#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Shows the help menu, or help for a specific command
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Type ?help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

/// Links a steamid to a discord user
#[poise::command(slash_command, prefix_command)]
pub async fn link(ctx: Context<'_>, steamid: i64) -> Result<(), Error> {
    let database = &ctx.data().database;
    let discordid = i64::from(ctx.author().id);
    let existing_discordid = match database.query_one("SELECT discordid FROM steamids WHERE steamid = $1", &[&discordid]).await {
        Ok(discordid) => if discordid.len() > 0 { discordid.get(0) } else { 0 },
        Err(e) => {
            log::error!("Failed to fetch discordid: {}", e);
            0
        }
    };
    database.execute(
        "INSERT INTO steamids (steamid, discordid) VALUES ($1, $2)
        ON CONFLICT (discordid)
        DO UPDATE SET steamid = EXCLUDED.steamid;
        ",
        &[&steamid, &discordid],
    ).await?;
    if existing_discordid == 0 {
        reply(&ctx, format!("Linked steamid: {}", steamid)).await?;
    } else {
        reply(&ctx, format!("Replacing existing steamid with: {}", steamid)).await?;
    }    
    Ok(())
}

/// Fetch linked steamid
#[poise::command(slash_command, prefix_command)]
pub async fn steamid(ctx: Context<'_>, user: Option<serenity::model::user::User>) -> Result<(), Error> {
    let database = &ctx.data().database;
    let user = match user {
        Some(user) => if ctx.author().member.as_ref().map_or(false, |m| m.permissions.map_or(false, poise::serenity_prelude::Permissions::administrator)) {
            user
        } else {
            reply(&ctx, "You don't have permission to view other users steamids.".to_string()).await?;
            return Ok(())
        },
        None => ctx.author().clone(),
    };
    let discordid = i64::from(user.id);
    let steamid: i64 = match database.query_one("SELECT steamid FROM steamids WHERE discordid = $1", &[&discordid]).await {
        Ok(steamid) => if steamid.len() > 0 { steamid.get(0) } else { 0 },
        Err(e) => {
            log::error!("Failed to fetch steamid: {}", e);
            0
        }
    };
    if steamid == 0 {
        reply(&ctx, format!("No steamid linked to discord user: {}", user.name)).await?;
    } else {
        reply(&ctx, format!("Steamid {} linked to discord user {}", steamid, user.name)).await?;
    }    
    Ok(())
}

/// Fetch highscore with optional parameters
#[poise::command(slash_command, prefix_command)]
pub async fn score(ctx: Context<'_>, user: Option<serenity::model::user::User>) -> Result<(), Error> {
    let database = &ctx.data().database;
    let discordid = match user {
        Some(user) => i64::from(user.id),
        None => i64::from(ctx.author().id),
    };
    let steamid = match database.query_one("SELECT steamid FROM steamids WHERE discordid = $1", &[&discordid]).await {
        Ok(row) => {
            let steamid: i64 = row.get(0);
            steamid
        },
        Err(e) => {
            log::error!("Failed to fetch steamid: {}", e);
            0
        }
    };
    if steamid == 0 {
        reply(&ctx, "There's no steamid linked to this discord user.".to_string()).await?;
        return Ok(())
    }
    let (score, car, track) = match database.query_one("SELECT score, car, track FROM cutup WHERE steamid = $1 ORDER BY score DESC LIMIT 1", &[&steamid]).await {
        Ok(row) => {
            let score: i64 = row.get(0);
            let car: String = row.get(1);
            let track: String = row.get(2);
            (score, car, track)
        },
        Err(_e) => {
            reply(&ctx, format!("No highscore found for steamid: {}", steamid)).await?;
            return Ok(())
        }
    };
    let placing = match database.query_one("SELECT COUNT(*) FROM cutup WHERE score > $1", &[&score]).await {
        Ok(row) => {
            let count: i64 = row.get(0);
            count + 1
        },
        Err(e) => {
            log::error!("Failed to fetch placing: {}", e);
            0
        }
    };
    reply(&ctx, format!("Your highscore is {} on {} with {}. You are currently in {} place.", score, track, car, placing)).await?;
    Ok(())
}