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

use poise::serenity_prelude::{GatewayIntents};
use shuttle_runtime::__internals::Context;

mod cards;
mod commands;
mod helpers;

mod prelude {
    pub use crate::cards::*;
    pub use crate::commands::fetch::fetch;
    pub use crate::helpers::{replace_tags, update_json};

    pub struct Data {}
    pub type Error = Box<dyn std::error::Error + Send + Sync>;
    pub type Context<'a> = poise::Context<'a, Data, Error>;
}

use prelude::*;

#[shuttle_runtime::main]
pub async fn axum (
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![fetch()],
            ..Default::default()
        })
        .token(discord_token)
        .intents(GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    println!("Poise framework defined, starting bot");
    framework.run().await.unwrap();

    todo!();
}
