//! # Commands to search for a Marvel Snap cards or locations.
//! 
//! ## This implementation features:
//! 
//! - `search_for()`
//!     - Default search mechanism to find a single card (non-case-sensitive)
//! 
//! # Important Note
//! Poise currently consumes docstrings for commands so you'll need to check
//! the repo pages individually for command-specific documentation.

use crate::prelude::{make_card_map, parse_cards, Context, Error};
use poise::serenity_prelude::MessageBuilder;



/// Basic search function to find a card base on its name. 
/// 
/// As with all commands, this should never be called directly 
/// but isntead passed to the bot to register during setup in
/// 
/// ## Arguments
///
/// * `ctx` - Context object passed from poise bot in [`main()`]
/// * `card_name` - A string of the card name being searched
#[poise::command(slash_command, prefix_command)]
pub async fn search_for(ctx: Context<'_>, card_name: String) -> Result<(), Error> {
    let card_map = make_card_map(parse_cards()?);
    let response = match card_map.get(&card_name) {
        Some(found_card) => MessageBuilder::new()
            .push("Card found!\n")
            .push(format!("Name:   {}\n", found_card.name))
            .push(format!("Power:  {}\n", found_card.power))
            .push(format!("Energy: {}", found_card.cost))
            .build(),
        None => format!("No cards found with name `{}`", card_name),
    };

    ctx.say(response).await?;
    Ok(())
}
