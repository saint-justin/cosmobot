use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

use crate::prelude::card_types::CardDataResponse;

const CARD_DATA_URI: &str = "https://marvelsnapzone.com/getinfo/?searchtype=cards&searchcardstype=true";

lazy_static! {
    static ref SPAN_TAG_RE: Regex = Regex::new(r"(<[a-z/]{1,}>)").unwrap();
}

pub fn replace_tags(s: &str) -> String {
    SPAN_TAG_RE.replace_all(s, "**").to_string()
}

pub async fn update_json() -> Result<(), Reqwest::Error> {
    let raw_json = reqwest::get(CARD_DATA_URI)
        .await?
        .text()
        .await?;

    let response = serde_json::from_str::<CardDataResponse>(&raw_json)?;
    let cards_str = serde_json::to_string(&response.success).unwrap();

    println!("Writing data of length {} to string at location /snap-data/cards.json", raw_json.len());
    fs::write("./snap-data/cards.json", cards_str).expect("Unable to write to file");

    Ok(())
}

#[cfg(test)]
mod helper_tests {
    use crate::prelude::replace_tags;

    use super::update_json;

    #[test]
    fn replace_tags_works() {
        assert_eq!(replace_tags("<span>test</span>"), "**test**".to_owned());
        assert_eq!(replace_tags("<em>test</em>"), "**test**".to_owned());
        assert_eq!(replace_tags("<b>test</b>"), "**test**".to_owned());
        assert_eq!(replace_tags("<i>test</i>"), "**test**".to_owned());
        assert_eq!(replace_tags("<strong>test</strong>"), "**test**".to_owned());
    }

    #[test]
    fn manually_update_json() {
        let _ = update_json();
    }
}