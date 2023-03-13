extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_proton_url(query: &str) -> String {
    if query == "pr" {
        "https://proton.me".to_string()
    } else {
        // Assume the other match is pr sometext and search
        construct_proton_search_url(&query[3..])
    }
}



pub fn construct_proton_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let proton_search_url = format!("https://proton.me/u/0/all-mail#keyword={}",
        encoded_query);
    
    proton_search_url
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_construct_proton_url() {
        let fake_query = "pr";
        assert_eq!(construct_proton_url(fake_query),
        "https://proton.me");
    }

    #[test]
    fn test_construct_proton_url_query() {
        let fake_query = "pr hello world";
        assert_eq!(
            construct_proton_url(fake_query),
            "https://proton.me/u/0/all-mail#keyword=hello%20world"
        );
    }

}