use poise::serenity_prelude as serenity;
use crate::{Context, Error};
use poise::command;

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
    let discordid = ctx.author().id.0 as i64;
    let existing_discordid = match database.query_one("SELECT discordid FROM steamids WHERE steamid = $1", &[&discordid]).await {
        Ok(discordid) => if discordid.len() > 0 { discordid.get(0) } else { 0 },
        Err(e) => {
            dbg!(e);
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
        ctx.send(|u| {u.content(format!("Linked steamid: {}", steamid)).ephemeral(true)}).await?;
    } else {
        ctx.send(|u| {u.content(format!("Replacing existing steamid with: {}", steamid)).ephemeral(true)}).await?;
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
            ctx.send(|u| {u.content("You don't have permission to view other users steamids.").ephemeral(true)}).await?;
            return Ok(())
        },
        None => ctx.author().clone(),
    };
    let discordid = i64::from(user.id);
    let steamid: i64 = match database.query_one("SELECT steamid FROM steamids WHERE discordid = $1", &[&discordid]).await {
        Ok(steamid) => if steamid.len() > 0 { steamid.get(0) } else { 0 },
        Err(e) => {
            dbg!(e);
            0
        }
    };
    if steamid == 0 {
        ctx.send(|u| {u.content(format!("No steamid linked to discord user: {}", user.name)).ephemeral(true)}).await?;
    } else {
        ctx.send(|u| {u.content(format!("Steamid {} linked to discord user {}", steamid, user.name)).ephemeral(true)}).await?;
    }    
    Ok(())
}

/// Fetch highscore with optional parameters
#[poise::command(slash_command, prefix_command)]
pub async fn score(ctx: Context<'_>, user: Option<serenity::model::user::User>) -> Result<(), Error> {
    let database = &ctx.data().database;
    let discordid = match user {
        Some(user) => i64::from(user.id),
        None => ctx.author().id.0 as i64,
    };
    let steamid = match database.query_one("SELECT steamid FROM steamids WHERE discordid = $1", &[&discordid]).await {
        Ok(row) => {
            let steamid: i64 = row.get(0);
            steamid
        },
        Err(e) => {
            dbg!(e);
            0
        }
    };
    if steamid == 0 {
        ctx.send(|u| {u.content("There's no steamid linked to this discord user.").ephemeral(true)}).await?;
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
            ctx.send(|u| {u.content(format!("No highscore found for steamid: {}", steamid)).ephemeral(true)}).await?;
            return Ok(())
        }
    };
    let placing = match database.query_one("SELECT COUNT(*) FROM cutup WHERE score > $1", &[&score]).await {
        Ok(row) => {
            let count: i64 = row.get(0);
            count + 1
        },
        Err(e) => {
            dbg!(e);
            0
        }
    };
    ctx.send(|u| {u.content(format!("Your highscore is {} on {} with {}. You are currently in {} place.", score, track, car, placing)).ephemeral(true)}).await?;
    Ok(())
}