//! # Cosmo: A Marvel Snap Card Discord Bot
//! 
//! Cosmo is a Discord bot that allows users to look up information
//! about Marvel Snap cards and locations. Cosmo is still early in 
//! development. 
//! 
//! See the feature list below for what is and isn't currently implemented:
//! 
//! ### Public Features:
//! - None yet (but many coming soon!)
//! 
//! ### Beta Features:
//! - Individual card searches
//!     - Exact name searches
//! 
//! ### Planned Features:
//! - Spoiler filtering
//!     - Spoilers-included card searches
//!     - Spoilers-included location searches
//! - Individual card searches
//!     - Fuzzy-name search recommendations
//! - Location searches
//!     - Exact-name searches
//!     - Fuzzy-name search recommendations
//! - Multi-card searches
//!     - Searches by name
//!     - Searches by keyword
//!     - Searches by power
//!     - Searches by cost
//!     - Searches by artist (inker)
//!     - Searches by artist (colorist)
//!     - Sorting by attribute (cost/power/name/etc.)
//! 
//! # Important Note
//! Poise currently consumes docstrings for commands so you'll need to check
//! the repo pages individually for command-specific source documentation.

use poise::serenity_prelude::{GatewayIntents, User};

mod aws_helpers;
mod cards;
mod search_for;
mod prelude {
    pub use crate::cards::*;
    pub use crate::search_for::search_for;

    pub struct Data {}
    pub type Error = Box<dyn std::error::Error + Send + Sync>;
    pub type Context<'a> = poise::Context<'a, Data, Error>;
}

use prelude::*;

#[tokio::main]
async fn main() {
    // Bot token from Secrets Manager
    let token = aws_helpers::get_bot_token()
        .await
        .expect("Error getting bot API token");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                age(), 
                search_for()
            ],
            ..Default::default()
        })
        .token(token)
        .intents(GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    println!("Poise framework defined, starting bot");
    framework.run().await.unwrap();
}

/// Placeholder test command to make sure Poise is working as intended
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
