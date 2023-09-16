extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_howlongtobeat_url(query: &str) -> String {
    if query == "hltb" {
        "https://howlongtobeat.com".to_string()
    } else {
        // Assume the other match is hltb sometext and search
        construct_howlongtobeat_search_url(&query[5..])
    }
}



pub fn construct_howlongtobeat_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let howlongtobeat_search_url = format!("https://howlongtobeat.com/?q={}",
                                     encoded_query);

    howlongtobeat_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_howlongtobeat_url() {
        let fake_query = "hltb";
        assert_eq!(construct_howlongtobeat_url(fake_query),
                   "https://howlongtobeat.com");
    }

    #[test]
    fn test_construct_howlongtobeat_url_query() {
        let fake_query = "hltb hello world";
        assert_eq!(
            construct_howlongtobeat_url(fake_query),
            "https://howlongtobeat.com/?q=hello%20world"
        );
    }

} 