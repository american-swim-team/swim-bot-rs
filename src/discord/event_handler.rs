use tokio::time::{sleep, Duration};
use std::sync::Arc;
use std::ops::Deref;

use crate::{Data, Error};
use poise::serenity_prelude as ser;

pub async fn event_handler<'a>(ctx: &poise::serenity_prelude::Context, event: &'a ser::FullEvent, data: &Data) -> Result<(), Error> {
    match event {
        ser::FullEvent::CacheReady { guilds } => {
            println!("Cache ready, bot fully functional.");

            let _data = data.clone();
            let _ctx = ctx.clone();

            tokio::spawn(async move {
                let data_clone = _data.clone();
                let ctx = _ctx.clone();

                loop {
                    // Your function to run every minute
                    update_leaderboards(&ctx, &data_clone).await;

                    // Sleep for 1 minute
                    sleep(Duration::from_secs(60)).await;
                }
            });
        },
        _ => {}
    }
    Ok(())
}

async fn update_leaderboards(ctx: &poise::serenity_prelude::Context, data: &Data) {
    let http = &ctx.http;
    let cache = &ctx.cache;
    // Fetch leaderboards from database
    let leaderboards: Vec<(String, i64, String)> = match data.database.query("SELECT * FROM leaderboards", &[]).await {
        Ok(leaderboards) => leaderboards
            .into_iter()
            .map(|row| {
                let title: String = row.get("title");
                let channel_id: i64 = row.get("channel");
                let db_query: String = row.get("query");
                (title, channel_id, db_query)
            })
            .collect(),
        Err(e) => {
            dbg!(e);
            return;
        }
    };

    ctx.shard.chunk_guild(ser::GuildId::new(*&data.config.discord.guild), None, false, ser::ChunkGuildFilter::None, None);
    let guild = cache.guild(*&data.config.discord.guild).expect("Couldnt find guild.").deref().to_owned();

    for (title, channel_id, db_query) in leaderboards {
        let channel = match ser::ChannelId::from(channel_id as u64).to_channel(http).await {
            Ok(channel) => channel.guild().unwrap(),
            Err(e) => {
                dbg!(e);
                continue;
            }
        };

        let role: (poise::serenity_prelude::Role, poise::serenity_prelude::RoleId) = match data.database.query_one("SELECT discord_role FROM leaderboards WHERE channel = $1", &[&channel_id]).await {
            Ok(row) => {
                let role_id = match row.try_get::<usize, i64>(0) {
                    Ok(value) => poise::serenity_prelude::RoleId::from(value as u64),
                    Err(e) => {
                        dbg!(e);
                        continue;
                    },
                };
                let role = cache.role(guild.id, role_id).as_deref().unwrap().to_owned();
                (role, role_id)
            },
            Err(_e) => {
                continue;
            }
        };

        let scores: Vec<(i64, i64)> = match data.database.query(&db_query, &[]).await {
            Ok(scores) => scores
                .into_iter()
                .map(|row| {
                    let steamid: i64 = row.get("discordid");
                    let score: i64 = row.get("score");
                    (steamid, score)
                })
                .collect(),
            Err(e) => {
                dbg!(e);
                continue;
            }
        };

        let mut top_users = String::new();
        let mut count = 1;

        for (user, score) in scores.iter() {
            match guild.member(http, ser::UserId::from(*user as u64)).await {
                Ok(member) => {
                    top_users.push_str(&format!("{}. {}: {}\n", count, member.user.name, score));
                    if !member.roles.contains(&role.1) {
                        match member.add_role(http, &role.1).await {
                            Ok(_) => {
                                println!("Added role {} to user {}", role.0.name, member.user.name);
                            }
                            Err(e) => {
                                dbg!(e);
                                continue;
                            }
                        }
                    }
                },
                Err(e) => {
                    dbg!(e);
                    top_users.push_str(&format!("{}. Unknown: {}\n", count, score));
                    continue;
                }
            };
            count += 1;
        }

        let mut messages = match channel.messages(http, ser::GetMessages::default().limit(1)).await {
            Ok(messages) => messages,
            Err(e) => {
                dbg!(e);
                continue;
            }
        };

        if messages.len() < 1 {
            let embed = ser::CreateEmbed::default()
                .title(title)
                .description(top_users)
                .to_owned();

            let msg = ser::CreateMessage::default()
                .embeds(vec![embed]);

            let _ = channel.send_message(http, msg).await;
        } else {
            let embed = ser::CreateEmbed::default()
                .title(title)
                .description(top_users)
                .to_owned();

            let msg = ser::EditMessage::default()
                .embeds(vec![embed]);

            let _ = messages.first_mut().unwrap().edit(http, msg).await;
        }

        for (_, member) in guild.members.iter() {
            if !&member.roles.contains(&role.1) {
                continue;
            }
            if !scores.iter().any(|(discord_id, _)| *discord_id == i64::from(member.user.id)) {
                match member.remove_role(http, &role.1).await {
                    Ok(_) => {
                        println!("Removed role {} from user {}", role.0.name, member.user.name);
                    }
                    Err(e) => {
                        dbg!(e);
                        return;
                    }
                }
            }
        };
    }
}
