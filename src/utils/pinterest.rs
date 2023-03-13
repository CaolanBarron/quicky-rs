extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_pinterest_url(query: &str) -> String {
    if query == "pi" {
        "https://pinterest.com".to_string()
    } else {
        // Assume the other match is # sometext and search
        construct_pinterest_search_url(&query[3..])
    }
}



pub fn construct_pinterest_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let pinterest_search_url = format!("https://pinterest.com/search/pins/?q={}",
        encoded_query);
    
    pinterest_search_url
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_construct_pinterest_url() {
        let fake_query = "pi";
        assert_eq!(construct_pinterest_url(fake_query),
        "https://pinterest.com");
    }

    #[test]
    fn test_construct_pinterest_url_query() {
        let fake_query = "pi hello world";
        assert_eq!(
            construct_pinterest_url(fake_query),
            "https://pinterest.com/search/pins/?q=hello%20world"
        );
    }

}