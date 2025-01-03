use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChirpifyError {
    #[error("Input is too long: {0} characters (max 5,000 to prevent API abuse)")]
    TweetTooLong(usize),

    #[error("Tweet is empty")]
    EmptyTweet,

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("OpenAI API error: {0}")]
    OpenAIError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ChirpifyError>;
