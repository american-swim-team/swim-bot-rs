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
            ctx.reply(format!("Failed to parse steamid: {}", e)).await?;
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
        ctx.reply(format!("Linked steamid {} to discord user {}", steamid, discordid)).await?;
        database.insert_ids(steamid, discordid).await?;
    } else {
        ctx.reply(format!("Replacing existing steamid with {}", steamid)).await?;
        database.insert_ids(steamid, discordid).await?;
    }    
    Ok(())
}

/// Fetches the steamid linked to a discord user
#[poise::command(slash_command, prefix_command)]
pub async fn steamid(ctx: Context<'_>) -> Result<(), Error> {
    let database = &ctx.data().database;
    let discordid = ctx.author().id.0 as i64;
    let steamid = match database.fetch_steamid(discordid).await {
        Ok(steamid) => steamid,
        Err(e) => {
            dbg!(e);
            0
        }
    };
    if steamid == 0 {
        ctx.reply(format!("No steamid linked to discord user {}", discordid)).await?;
    } else {
        ctx.reply(format!("Steamid {} linked to discord user {}", steamid, discordid)).await?;
    }    
    Ok(())
}