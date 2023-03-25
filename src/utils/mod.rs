pub mod amazon;
pub mod github;
pub mod google;
pub mod pinterest;
pub mod protonmail;
pub mod reddit;
pub mod twitter;
pub mod youtube;
pub mod linkedin;
pub mod stackoverflow;

// This functions attempts to parse the command from the users input
pub fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string.contains(' ') {
        // This space is required to split the string
        let index_of_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_space];
    }
    // or return the string as is
    &query_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        // Test with command only
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let actual = get_command_from_query_string("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}
