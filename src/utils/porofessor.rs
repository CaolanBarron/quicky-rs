extern crate percent_encoding;
use percent_encoding::{AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`').add(b'&').add(b'|');

pub fn construct_porofessor_url(query: &str) -> String {
    if query == "poro" {
        "https://porofessor.gg".to_string()
    } else {
        construct_porofessor_profile_url(&query[5..])
    }
}

fn construct_porofessor_profile_url(profile: &str) -> String {
    format!("https://www.leagueofgraphs.com/summoner/euw/{}", profile)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_porofessor_url() {
        let fake_query = "poro";
        assert_eq!(construct_porofessor_url(fake_query), "https://porofessor.gg")
    }

    #[test]
    fn test_construct_porofessor_profile_url(){
        let fake_query = "poro myprofile";
        assert_eq!(construct_porofessor_url(fake_query), "https://www.leagueofgraphs.com/summoner/euw/myprofile")
    }
}