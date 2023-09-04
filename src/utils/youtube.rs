use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`').add(b'&').add(b'|');

pub fn construct_youtube_url(query: &str) -> String {
    if query == "yt" {
        "https://youtube.com".to_string()
    } else if &query[..4] == "yt @" {
        format!("https://youtube.com/user/{}", &query[4..])
    } else {
        // Assume the other match is yt sometext and search
        construct_youtube_search_url(&query[3..])
    }
}

pub fn construct_youtube_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    println!("{}", encoded_query);
    let youtube_search_url = format!("https://youtube.com/results?search_query={}", encoded_query);

    youtube_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_youtube_url() {
        let fake_query = "yt";
        assert_eq!(construct_youtube_url(fake_query), "https://youtube.com");
    }

    #[test]
    fn test_construct_youtube_profile_url() {
        let fake_query = "yt @Destiny";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://youtube.com/user/Destiny"
        );
    }

    #[test]
    fn test_construct_youtube_query_url() {
        let fake_query = "yt hello world";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://youtube.com/results?search_query=hello%20world"
        );
    }
}
