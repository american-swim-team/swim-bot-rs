use crate::{Context, Error};

/// Responds with pong
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

/// Links a steamid to a discord user
#[poise::command(slash_command, prefix_command)]
pub async fn link(ctx: Context<'_>, steamid: String) -> Result<(), Error> {
    let database = &ctx.data().database;
    // try to cast to integer, if it fails message user and return
    let steamid = match steamid.parse::<i64>() {
        Ok(steamid) => steamid,
        Err(e) => {
            ctx.send(|u| {u.content(format!("Failed to parse steamid: {}", e)).ephemeral(true)}).await?;
            return Ok(())
        }
    };
    let discordid = ctx.author().id.0 as i64;
    let existing_discordid = match database.fetch_discordid(steamid).await {
        Ok(discordid) => discordid,
        Err(e) => {
            dbg!(e);
            0
        }
    };
    if existing_discordid == 0 {
        ctx.send(|u| {u.content(format!("Linked steamid: {}", steamid)).ephemeral(true)}).await?;
        database.insert_ids(steamid, discordid).await?;
    } else {
        ctx.send(|u| {u.content(format!("Replacing existing steamid with: {}", steamid)).ephemeral(true)}).await?;
        database.insert_ids(steamid, discordid).await?;
    }    
    Ok(())
}

/// Fetches the steamid linked to a discord user
#[poise::command(slash_command, prefix_command)]
pub async fn steamid(ctx: Context<'_>) -> Result<(), Error> {
    let database = &ctx.data().database;
    let author = ctx.author();
    let discordid = author.id.0 as i64;
    let steamid = match database.fetch_steamid(discordid).await {
        Ok(steamid) => steamid,
        Err(e) => {
            dbg!(e);
            0
        }
    };
    if steamid == 0 {
        ctx.send(|u| {u.content(format!("No steamid linked to discord user: {}", author.name)).ephemeral(true)}).await?;
    } else {
        ctx.send(|u| {u.content(format!("Steamid {} linked to discord user {}", steamid, author.name)).ephemeral(true)}).await?;
    }    
    Ok(())
}

/// if user is bot owner, removes all global commands
#[poise::command(slash_command, prefix_command, hide_in_help, owners_only)]
pub async fn remove_global_commands(ctx: Context<'_>) -> Result<(), Error> {
    let http = &ctx.http();
    let global_commands = http.get_global_application_commands().await?;
    for command in global_commands {
        http.delete_global_application_command(*command.id.as_u64()).await?;
    }
    Ok(())
}