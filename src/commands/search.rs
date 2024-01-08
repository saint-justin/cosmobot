//! # Commands to search for a Marvel Snap cards or locations.
//!
//! ## This implementation features:
//!
//! - `search_for()`
//!     - Default search mechanism to find a single card (non-case-sensitive)

use crate::prelude::replace_tags;
use crate::prelude::{card_types::Card, make_card_map, parse_cards, Context, Error};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

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
pub async fn fetch(ctx: Context<'_>, card_name: String) -> Result<(), Error> {
    let card_map = make_card_map(parse_cards()?, false);
    match card_map.get(&card_name.to_ascii_lowercase()) {
        Some(found_card) => {
            reply_with_card(ctx, found_card).await?;
            Ok(())
        }
        None => match try_fuzzy_search(card_name.to_ascii_lowercase().clone(), &card_map) {
            Some(fuzzy_matches) => {
                if fuzzy_matches.len() > 10 {
                    let mut msg = fuzzy_matches[0..10].iter().fold(
                        format!("No exact matches for `{}`, did you mean one of these?", card_name.to_ascii_lowercase()),
                        |acc, fuzzy_name| {
                            format!("{}\n`{}`", acc, card_map.get(fuzzy_name).unwrap().name)
                        },
                    );
                    msg += &format!("\nor {} others.", fuzzy_matches.len() - 10);
                    ctx.reply(msg).await?;
                    Ok(())
                } else {
                    let msg = fuzzy_matches.iter().fold(
                        format!("No exact matches for `{}`, did you mean one of these?", card_name),
                        |acc, fuzzy_name| {
                            format!("{}\n`{}`", acc, card_map.get(fuzzy_name).unwrap().name)
                        },
                    );
                    ctx.reply(msg).await?;
                    Ok(())
                }

                
            }

            None => {
                println!("No matches for card [{}]", card_name.clone());
                ctx.say(format!("No matches found for card `{}`", card_name.clone()))
                    .await?;
                Ok(())
            }
        },
    }
}

fn try_fuzzy_search(card_name: String, card_map: &HashMap<String, Card>) -> Option<Vec<String>> {
    // Try a fuzzy search first
    let matcher = SkimMatcherV2::default();
    let mut matching_cards = card_map
        .keys()
        .filter_map(|key| {
            if let Some(score) = matcher.fuzzy_match(key, &card_name) {
                return Some((score, key));
            }
            None
        })
        .collect_vec();

    if matching_cards.len() == 0 {
        return None;
    }

    // Sort by match quality
    matching_cards.sort_by(|a, b| {
        if a.0 < b.0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    Some(matching_cards.iter().map(|m| m.1.to_owned()).collect_vec())
}

async fn reply_with_card(ctx: Context<'_>, card: &Card) -> Result<(), Error> {
    ctx.send(|reply| {
        reply
            .embed(|embed| {
                embed
                    .title(card.name.clone())
                    .image(card.art.clone())
                    .field("Cost", card.cost, true)
                    .field("Power", card.power, true)
                    .field("Ability", get_ability(card), false)
                    .url(card.url.clone())
            })
            .ephemeral(false)
    })
    .await?;

    Ok(())
}

fn get_ability(card: &Card) -> String {
    if card.ability.len() != 0 {
        replace_tags(&card.ability)
    } else {
        format!("*{}*", card.flavor)
    }
}
