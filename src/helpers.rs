use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SPAN_TAG_RE: Regex = Regex::new(r"(<[a-z/]{1,}>)").unwrap();
}

pub fn replace_span_tags(s: &str) -> String {
    SPAN_TAG_RE.replace_all(s, "**").to_string()
}
