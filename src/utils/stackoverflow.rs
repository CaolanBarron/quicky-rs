extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_stackoverflow_url(query: &str) -> String {
    if query == "sof" {
        "https://stackoverflow.com".to_string()
    } else {
        // Assume the other match is # sometext and search
        construct_stackoverflow_search_url(&query[4..])
    }
}

pub fn construct_stackoverflow_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let stackoverflow_search_url = format!("https://stackoverflow.com/search?q={}", encoded_query);

    stackoverflow_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_stackoverflow_url() {
        let fake_query = "sof";
        assert_eq!(
            construct_stackoverflow_url(fake_query),
            "https://stackoverflow.com"
        );
    }

    #[test]
    fn test_construct_stackoverflow_url_query() {
        let fake_query = "sof hello world";
        assert_eq!(
            construct_stackoverflow_url(fake_query),
            "https://stackoverflow.com/search?q=hello%20world"
        );
    }
}
