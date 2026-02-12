use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// YouTube video URL or ID
    #[arg(value_name = "VIDEO")]
    video: String,

    /// YouTube Data API key (overrides env)
    #[arg(long = "api-key")]
    api_key: Option<String>,

    /// Output CSV path (defaults to <videoId>.csv)
    #[arg(long = "out")]
    out: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let args = Args::parse();

    let api_key = args
        .api_key
        .or_else(|| std::env::var("YOUTUBE_API_KEY").ok())
        .ok_or_else(|| anyhow!("Missing API key. Use --api-key or set YOUTUBE_API_KEY."))?;

    let out_path = youtube_comment_scraper::scrape_to_csv(&api_key, &args.video, args.out).await?;
    println!("Wrote CSV to {}", out_path.display());
    Ok(())
}
