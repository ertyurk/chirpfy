use crate::agent::TweetAgent;
use crate::error::Result;

pub async fn refine_tweet(tweet: &str) -> Result<String> {
    let agent = TweetAgent::new()?;
    let refined = agent.refine(tweet).await?;
    Ok(refined)
} 