extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<')
    .add(b'>').add(b'`');

pub fn construct_reddit_url(query: &str) ->String {

    if query == "r" {
        //base url
        "https://reddit.com".to_string()
    }
    else if &query[..3] == "r /"{
        // sub url
        construct_reddit_sub_url(&query[3..])
    }
    else if &query[..3] == "r @" {
        // profile url
        construct_reddit_profile_url(&query[3..])
    }
    else {
        // search url
        construct_reddit_search_url(&query[2..])
    }
}

// Construct reddit sub url
fn construct_reddit_sub_url(sub: &str) -> String {
    format!("https://reddit.com/r/{}", sub)
}

// Construct reddit search url
fn construct_reddit_search_url(query: &str) -> String{
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    
    let reddit_search_url = format!("https://reddit.com/search/?q={}",
        encoded_query);
    
    reddit_search_url

}

// Construct reddit profile url
fn construct_reddit_profile_url(profile: &str) -> String{
    format!("https://reddit.com/user/{}", profile)
}

#[cfg(test)]
mod tests {
    use super::construct_reddit_url;

    // base url test
    #[test]
    fn test_construct_reddit_url() {
        let fake_query = "r";
        assert_eq!(
            construct_reddit_url(fake_query),
            "https://reddit.com"
        );
    }

    // sub url test
    #[test]
    fn test_construct_reddit_sub_url() {
        let fake_query = "r /programming";

        assert_eq!(
            construct_reddit_url(fake_query),
            "https://reddit.com/r/programming"
        );
    }

    // search url test
    #[test]
    fn test_construct_reddit_search_url() {
        let fake_query = "r todays news";

        assert_eq!(
            construct_reddit_url(fake_query),
            "https://reddit.com/search/?q=todays%20news"
        );
    }

    // profile url test
    #[test]
    fn test_construct_reddit_profile_url() {
        let fake_query = "r @LeftTales";

        assert_eq!(
            construct_reddit_url(fake_query),
            "https://reddit.com/user/LeftTales"
        )
    }


}
