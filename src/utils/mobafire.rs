extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`').add(b'&').add(b'|');

pub fn construct_mobafire_url(query: &str) -> String {
    if query == "mf" {
        "https://www.mobafire.com".to_string()
    } else {
        construct_mobafire_champion_url(&query[3..])
    }
}

fn construct_mobafire_champion_url(champion: &str) -> String {
    format!("https://www.mobafire.com/league-of-legends/champions/{}", champion)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_mobafire_url() {
        let fake_query = "mf";
        assert_eq!(
            construct_mobafire_url(fake_query), 
            "https://www.mobafire.com"
        )
    }
    #[test]
    fn test_construct_mobafire_champion_url() {
        let fake_query = "mf jhin";
        assert_eq!(
            construct_mobafire_url(fake_query),
            "https://www.mobafire.com/league-of-legends/champions/jhin"
        )
    }
}