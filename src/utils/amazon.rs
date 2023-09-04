
extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};


const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`').add(b'&').add(b'|');

pub fn construct_amazon_url(query: &str) -> String {
    if query == "am" {
        "https://amazon.co.uk".to_string()
    } else {
        // Assume the other match is # sometext and search
        construct_amazon_search_url(&query[3..])
    }
}

pub fn construct_amazon_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let amazon_search_url = format!("https://amazon.co.uk/s?k={}", encoded_query);

    amazon_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_amazon_url() {
        let fake_query = "am";
        assert_eq!(construct_amazon_url(fake_query), "https://amazon.co.uk");
    }

    #[test]
    fn test_construct_amazon_url_query() {
        let fake_query = "am hello world";
        assert_eq!(
            construct_amazon_url(fake_query),
            "https://amazon.co.uk/s?k=hello%20world"
        );
    }

    #[test]
    fn test_construct_amazon_url_query_with_special_characters() {
        let fake_query = "am test&best";

        assert_eq!(
            construct_amazon_url(fake_query),
            "https://amazon.co.uk/s?k=test%26best"
        )
    }
}
