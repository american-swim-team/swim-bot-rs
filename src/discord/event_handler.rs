use poise::Event;
// use tokio_cron::{Scheduler, Job};
use std::sync::Arc;

use poise::serenity_prelude as serenity;

use serenity::model::webhook::Webhook;
use crate::{Data, Error};
// use crate::database::Database;

// async fn leaderboards(ctx: &Arc<serenity::Context>, database: &Data) {
//     let http = ctx.http;
//     let database = &database.database;
//     // fetch leaderboards from the database
//     let leaderboards = match database.query("select title, webhookurl, dbquery from leaderboards", &[]).await {
//         Ok(leaderboards) => leaderboards,
//         Err(e) => {
//             dbg!(e);
//             return;
//         }
//     };

//     for leaderboard in leaderboards {
//         let title: String = leaderboard.get(0);
//         let webhookurl: String = leaderboard.get(1);
//         let dbquery: String = leaderboard.get(2);

//         // setup webhook
//         let webhook = match Webhook::from_url(&http, &webhookurl).await {
//             Ok(webhook) => webhook,
//             Err(e) => {
//                 dbg!(e);
//                 continue; // continue to the next leaderboard if webhook fetch fails
//             }
//         };

//         let scores = match database.query(&dbquery, &[]).await {
//             Ok(scores) => scores,
//             Err(e) => {
//                 dbg!(e);
//                 return;
//             }
//         };

//         let mut embeds: Vec<serenity::builder::CreateEmbed> = Vec::new();

//         // create and add embeds for each score
//         for (i, score) in scores.iter().enumerate() {
//             let discordid: i64 = score.get(0);
//             let score_value: i64 = score.get(1);

//             // fetch discord user
//             let user = match http.get_user(discordid as u64).await {
//                 Ok(user) => user,
//                 Err(e) => {
//                     dbg!(e);
//                     continue; // continue to the next score if user fetch fails
//                 }
//             };

//             let embed = serenity::builder::CreateEmbed::default()
//                 .title(format!("{}. {} - {}", i + 1, user.name, score_value))
//                 .thumbnail(user.avatar_url().unwrap_or_default());

//             embeds.push(embed);
//         }

//         // send the embeds to the webhook
//         let builder = serenity::builder::ExecuteWebhook::default().content(format!("Leaderboard for {}", title)).embeds(embeds);
//         match webhook.execute(&http, false, builder).await {
//             Ok(_) => {},
//             Err(e) => {
//                 dbg!(e);
//                 continue; // continue to the next leaderboard if webhook send fails
//             }
//         }
//     }
// }


pub async fn event_handler(ctx: &serenity::client::Context, event: &Event<'_>, _framework: poise::FrameworkContext<'_, Data, Error>, data: &Data) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name);

        //     let mut scheduler = Scheduler::utc();

        //     let ctx_clone = Arc::new(ctx.clone());
        //     scheduler.add(Job::new("*/2 24 * * * *", move || {
        //         leaderboards(&ctx_clone, data)
        //     }));

        },
        _ => {}
    }
    Ok(())
}