use poise::FrameworkError;
use std::error::Error as StdError;

pub async fn on_error(error: FrameworkError<'_, _, Box<(dyn StdError + std::marker::Send + Sync + 'static)>>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            log::info!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                log::error!("Error in default error handler: {:?}", e);
            }
        }
    }
}