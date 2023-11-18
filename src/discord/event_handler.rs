use poise::Event;
use crate::{Data, Error};

pub async fn event_handler(ctx: &serenity::client::Context, event: &Event<'_>, _framework: poise::FrameworkContext<'_, Data, Error>, data: &Data) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}