use tokio::time::{sleep, Duration};
use std::sync::Arc;

use crate::{Data, Error};
use poise::serenity_prelude as ser;

pub async fn event_handler<'a>(ctx: &poise::serenity_prelude::Context, event: &'a ser::FullEvent, data: &Data) -> Result<(), Error> {
    match event {
        ser::FullEvent::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name);

            let _data = data.clone();
            let _http = Arc::clone(&ctx.http);

            tokio::spawn(async move {
                let http = _http.clone();
                let data_clone = _data.clone();

                loop {
                    // Your function to run every minute
                    your_function(&http, &data_clone).await;

                    // Sleep for 1 minute
                    sleep(Duration::from_secs(60)).await;
                }
            });
        },
        _ => {}
    }
    Ok(())
}

async fn your_function(http: &Arc<poise::serenity_prelude::Http>, data: &Data) {
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

    for (title, channel_id, db_query) in leaderboards {
        let channel = match ser::ChannelId::from(channel_id as u64).to_channel(http).await {
            Ok(channel) => channel.guild().unwrap(),
            Err(e) => {
                dbg!(e);
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
            match ser::GuildId::from(data.config.discord.guild).member(http, ser::UserId::from(*user as u64)).await {
                Ok(member) => top_users.push_str(&format!("{}. {}: {}\n", count, member.user.name, score)),
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
            continue;
        } else {
            let embed = ser::CreateEmbed::default()
                .title(title)
                .description(top_users)
                .to_owned();

            let msg = ser::EditMessage::default()
                .embeds(vec![embed]);

            let _ = messages.first_mut().unwrap().edit(http, msg).await;
            continue;
        }
    }
}
