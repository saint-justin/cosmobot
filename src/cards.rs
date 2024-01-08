//! A set of Snap card data management helper functions.

#![allow(dead_code)]
use self::card_types::Card;
use self::card_types::Cards;
use std::collections::HashMap;
use std::fs;

pub mod card_types;

/// Parses card JSON blob into Cards type
pub fn parse_cards() -> serde_json::Result<Cards> {
    let raw_json =
        fs::read_to_string("./snap-data/cards.json").expect("Should have been able to read path");
    let cards = serde_json::from_str::<Cards>(&raw_json)?;
    Ok(cards)
}

/// Creates a hashmap from an input of Cards type
pub fn make_card_map(input: Cards, include_spoilers: bool) -> HashMap<String, Card> {
    let mut map = HashMap::new();
    for card in input.cards {
        let card_released = card.source.to_ascii_lowercase() != "none";
        if card_released || (!card_released && include_spoilers) {
            map.insert(card.name.clone().to_ascii_lowercase(), card.clone());
        }
    }

    map
}