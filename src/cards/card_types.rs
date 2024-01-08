use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cards {
    pub cards: Vec<Card>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub cid: u32,
    pub name: String,
    pub r#type: String,
    pub cost: i32,
    pub power: i32,
    pub ability: String,
    pub flavor: String,
    pub art: String,
    pub alternate_art: String,
    pub url: String,
    pub status: String,
    pub carddefid: String,
    pub variants: Vec<Variant>,
    pub source: String,
    pub source_slug: String,
    pub tags: Vec<TagElement>,
    pub rarity: String,
    pub rarity_slug: String,
    pub difficulty: String,
    pub sketcher: String,
    pub inker: String,
    pub colorist: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)] // Naming needed for easy parsing
pub struct Variant {
    cid: u32,
    vid: i32,
    art: String,
    art_filename: String,
    rarity: String,
    rarity_slug: String,
    variant_order: String,
    status: String,
    full_description: String,
    inker: String,
    sketcher: String,
    colorist: String,
    possession: StringOrNumber,
    usage_count: StringOrNumber,
    ReleaseDate: StringOrNumber,
    UseIfOwn: String,
    PossesionShare: String,
    UsageShare: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagElement {
    tag_id: StringOrNumber,
    tag: String,
    tag_slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Int(u32),
}
