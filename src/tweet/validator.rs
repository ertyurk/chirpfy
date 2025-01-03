use crate::error::{ChirpifyError, Result};

const MAX_TWEET_LENGTH: usize = 280;

pub fn validate_tweet(tweet: &str) -> Result<()> {
    if tweet.is_empty() {
        return Err(ChirpifyError::EmptyTweet);
    }

    let length = tweet.chars().count();
    if length > MAX_TWEET_LENGTH {
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
        let long_tweet = "a".repeat(281);
        assert!(matches!(
            validate_tweet(&long_tweet),
            Err(ChirpifyError::TweetTooLong(281))
        ));
    }

    #[test]
    fn test_validate_valid_tweet() {
        assert!(validate_tweet("This is a valid tweet").is_ok());
    }
}
