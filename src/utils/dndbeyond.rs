extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_dndbeyond_url(query: &str) -> String {
    if &query[3..] == " ch"{
        "https://dndbeyond.com/characters".to_string()
    }
    else if &query[3..] == " ca" {
        "https://dndbeyond.com/my-campaigns".to_string()
    }
    else
    {
        "https://dndbeyond.com".to_string()
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_construct_dndbeyond_url() {
        let fake_query = "dnd";
        assert_eq!(construct_dndbeyond_url(fake_query),
        "https://dndbeyond.com");
    }

    #[test]
    fn test_construct_dndbeyond_characters_url() {
        let fake_query = "dnd ch";
        assert_eq!(construct_dndbeyond_url(fake_query),
        "https://dndbeyond.com/characters");
    }

    #[test]
    fn test_construct_dndbeyond_campaigns_url() {
        let fake_query = "dnd ca";
        assert_eq!(construct_dndbeyond_url(fake_query),
        "https://dndbeyond.com/my-campaigns");
    }

}