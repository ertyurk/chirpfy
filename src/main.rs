mod agent;
mod error;
mod tweet;

use clap::Parser;
use error::Result;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// The tweet to refine
    tweet: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Validate the tweet
    tweet::validate_tweet(&cli.tweet)?;

    // Refine the tweet
    let refined = tweet::refine_tweet(&cli.tweet).await?;

    // Print the refined tweet
    println!("{}", refined);

    Ok(())
}
