use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SPAN_TAG_RE: Regex = Regex::new(r"(<[a-z/]{1,}>)").unwrap();
}

pub fn replace_tags(s: &str) -> String {
    SPAN_TAG_RE.replace_all(s, "**").to_string()
}

#[cfg(test)]
mod helper_tests {
    use crate::prelude::replace_tags;

    #[test]
    fn replace_tags_works() {
        assert_eq!(replace_tags("<span>test</span>"), "**test**".to_owned());
        assert_eq!(replace_tags("<em>test</em>"), "**test**".to_owned());
        assert_eq!(replace_tags("<b>test</b>"), "**test**".to_owned());
        assert_eq!(replace_tags("<i>test</i>"), "**test**".to_owned());
        assert_eq!(replace_tags("<strong>test</strong>"), "**test**".to_owned());
    }
}