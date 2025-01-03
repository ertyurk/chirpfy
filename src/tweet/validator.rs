use crate::error::{ChirpifyError, Result};

// Reasonable limit to prevent API abuse (25,000 chars is Twitter's limit)
const MAX_INPUT_LENGTH: usize = 5_000;

pub fn validate_tweet(tweet: &str) -> Result<()> {
    if tweet.is_empty() {
        return Err(ChirpifyError::EmptyTweet);
    }

    let length = tweet.chars().count();
    if length > MAX_INPUT_LENGTH {
        return Err(ChirpifyError::TweetTooLong(length));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_tweet() {
        assert!(matches!(validate_tweet(""), Err(ChirpifyError::EmptyTweet)));
    }

    #[test]
    fn test_validate_long_tweet() {
        let long_tweet = "a".repeat(MAX_INPUT_LENGTH + 1);
        assert!(matches!(
            validate_tweet(&long_tweet),
            Err(ChirpifyError::TweetTooLong(_))
        ));
    }

    #[test]
    fn test_validate_valid_tweet() {
        assert!(validate_tweet("This is a valid tweet").is_ok());
    }

    #[test]
    fn test_validate_long_but_valid_tweet() {
        let tweet = "a".repeat(5_000);
        assert!(validate_tweet(&tweet).is_ok());
    }
}
