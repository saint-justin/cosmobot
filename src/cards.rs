#![allow(dead_code)]
use std::collections::HashMap;
use std::fs;
use std::env::current_dir;
use self::card_types::Card;
use self::card_types::Cards;

mod card_types;

/// Parses card JSON blob into Cards type
pub fn parse_cards() -> serde_json::Result<Cards> {
    println!("Current Dir: {:?}", current_dir().unwrap());
    let raw_json = fs::read_to_string("./snap-data/cards.json")
        .expect("Should have been able to read path");
    let cards = serde_json::from_str::<Cards>(&raw_json)?;
    Ok(cards)
}

/// Creates a hashmap from an input of Cards type
pub fn make_card_map(input: Cards) -> HashMap<String, Card> {
    let mut map = HashMap::new();
    for card in input.cards {
        map.insert(card.name.clone(), card.clone());
    }
    map
}
