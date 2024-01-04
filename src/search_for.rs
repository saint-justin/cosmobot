//! # Commands to search for a Marvel Snap cards or locations.
//! 
//! ## This implementation features:
//! 
//! - `search_for()`
//!     - Default search mechanism to find a single card (non-case-sensitive)

use std::{cmp::Ordering, collections::HashMap};

use crate::prelude::{make_card_map, parse_cards, Context, Error, card_types::Card};
use poise::serenity_prelude::MessageBuilder;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use itertools::Itertools;


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
        Some(found_card) => 
        {
            // Handle exact match found
            MessageBuilder::new()
                .push("Card found!\n")
                .push(format!("Name:   {}\n", found_card.name))
                .push(format!("Power:  {}\n", found_card.power))
                .push(format!("Energy Cost: {}\n", found_card.cost))
                .push(format!("Ability: {}", found_card.ability))
                .build()
        },
        None => {
            match try_fuzzy_search(&card_name, &card_map) {
                Some(fuzzy_matches) => {
                    let mut msg = MessageBuilder::new();
                    msg.push("Potential match(es) found:");
                    for fuzzy_match_card in fuzzy_matches {
                        msg.push(format!("\n{}", card_map.get(&fuzzy_match_card).unwrap().name));
                    }
                    msg.build()
                },
                
                None => format!("No matches found for card `{}`", card_name)
            }
        },
    };

    ctx.say(response).await?;
    Ok(())
}

fn try_fuzzy_search(card_name: &str, card_map: &HashMap<String, Card>) -> Option<Vec<String>> {
    // Try a fuzzy search first
    let matcher = SkimMatcherV2::default();
    let mut matching_cards = card_map.keys()
        .filter_map(|key| {
            if let Some(score) = matcher.fuzzy_match(key, &card_name) {
                return Some((score, key));
            } 
            None
        })
        .collect_vec();

    if matching_cards.len() == 0 {
        return None
    }

    // Sort by 
    matching_cards.sort_by(|a, b| if a.0 < b.0 { Ordering::Greater } else { Ordering::Less });
    println!("\nFound {} matches", matching_cards.len());
    matching_cards.iter().for_each(|(score, key)| {
        println!("Match of {} to {:3} => {}", card_name, score, key);
    });

    Some(matching_cards.iter().map(|m| m.1.to_owned()).collect_vec())
}
