use anyhow::{anyhow, Context, Result};
use clap::Parser;
use csv::Writer;
use reqwest::Client;
use serde::Deserialize;
use std::fs::File;
use std::path::PathBuf;
use url::Url;

const API_BASE: &str = "https://www.googleapis.com/youtube/v3";

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

#[derive(Debug)]
struct CommentRow {
    comment_id: String,
    parent_id: String,
    video_id: String,
    author: String,
    author_channel_id: String,
    published_at: String,
    like_count: i64,
    text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentThreadListResponse {
    items: Option<Vec<CommentThreadItem>>,
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentThreadItem {
    snippet: Option<CommentThreadSnippet>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentThreadSnippet {
    top_level_comment: Option<TopLevelComment>,
    total_reply_count: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TopLevelComment {
    id: Option<String>,
    snippet: Option<CommentSnippet>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentListResponse {
    items: Option<Vec<CommentItem>>,
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentItem {
    id: Option<String>,
    snippet: Option<CommentSnippet>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentSnippet {
    author_display_name: Option<String>,
    author_channel_id: Option<AuthorChannelId>,
    published_at: Option<String>,
    like_count: Option<i64>,
    text_display: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthorChannelId {
    value: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    let api_key = args
        .api_key
        .or_else(|| std::env::var("YOUTUBE_API_KEY").ok())
        .ok_or_else(|| anyhow!("Missing API key. Use --api-key or set YOUTUBE_API_KEY."))?;

    let video_id = parse_video_id(&args.video).ok_or_else(|| {
        anyhow!("Could not parse a YouTube video ID from input: {}", args.video)
    })?;

    let out_path = args
        .out
        .unwrap_or_else(|| PathBuf::from(format!("{}.csv", video_id)));

    let client = Client::new();
    let rows = fetch_comment_threads(&client, &api_key, &video_id).await?;
    write_csv(&out_path, &rows).context("writing CSV")?;

    println!("Wrote {} rows to {}", rows.len(), out_path.display());
    Ok(())
}

async fn fetch_comment_threads(client: &Client, api_key: &str, video_id: &str) -> Result<Vec<CommentRow>> {
    let mut rows: Vec<CommentRow> = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let mut params = vec![
            ("part", "snippet"),
            ("videoId", video_id),
            ("maxResults", "100"),
            ("textFormat", "plainText"),
            ("key", api_key),
        ];
        if let Some(token) = page_token.as_deref() {
            params.push(("pageToken", token));
        }

        let url = format!("{}/commentThreads", API_BASE);
        let resp = client.get(&url).query(&params).send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("HTTP {}: {}", status, body));
        }

        let data: CommentThreadListResponse = resp.json().await?;
        if let Some(items) = data.items {
            for item in items {
                let snippet = match item.snippet {
                    Some(value) => value,
                    None => continue,
                };
                let top = match snippet.top_level_comment {
                    Some(value) => value,
                    None => continue,
                };
                let top_id = match top.id {
                    Some(id) if !id.is_empty() => id,
                    _ => continue,
                };

                if let Some(snippet) = top.snippet {
                    rows.push(CommentRow {
                        comment_id: top_id.clone(),
                        parent_id: String::new(),
                        video_id: video_id.to_string(),
                        author: snippet.author_display_name.unwrap_or_default(),
                        author_channel_id: snippet.author_channel_id.and_then(|c| c.value).unwrap_or_default(),
                        published_at: snippet.published_at.unwrap_or_default(),
                        like_count: snippet.like_count.unwrap_or(0),
                        text: snippet.text_display.unwrap_or_default(),
                    });
                }

                let reply_count = snippet.total_reply_count.unwrap_or(0);

                if reply_count > 0 {
                    let mut replies = fetch_replies(client, api_key, &top_id, video_id).await?;
                    rows.append(&mut replies);
                }
            }
        }

        if let Some(token) = data.next_page_token {
            page_token = Some(token);
        } else {
            break;
        }
    }

    Ok(rows)
}

async fn fetch_replies(
    client: &Client,
    api_key: &str,
    parent_id: &str,
    video_id: &str,
) -> Result<Vec<CommentRow>> {
    let mut rows: Vec<CommentRow> = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let mut params = vec![
            ("part", "snippet"),
            ("parentId", parent_id),
            ("maxResults", "100"),
            ("textFormat", "plainText"),
            ("key", api_key),
        ];
        if let Some(token) = page_token.as_deref() {
            params.push(("pageToken", token));
        }

        let url = format!("{}/comments", API_BASE);
        let resp = client.get(&url).query(&params).send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("HTTP {}: {}", status, body));
        }

        let data: CommentListResponse = resp.json().await?;
        if let Some(items) = data.items {
            for item in items {
                let snippet = match item.snippet {
                    Some(value) => value,
                    None => continue,
                };
                rows.push(CommentRow {
                    comment_id: item.id.unwrap_or_default(),
                    parent_id: parent_id.to_string(),
                    video_id: video_id.to_string(),
                    author: snippet.author_display_name.unwrap_or_default(),
                    author_channel_id: snippet.author_channel_id.and_then(|c| c.value).unwrap_or_default(),
                    published_at: snippet.published_at.unwrap_or_default(),
                    like_count: snippet.like_count.unwrap_or(0),
                    text: snippet.text_display.unwrap_or_default(),
                });
            }
        }

        if let Some(token) = data.next_page_token {
            page_token = Some(token);
        } else {
            break;
        }
    }

    Ok(rows)
}

fn write_csv(out_path: &PathBuf, rows: &[CommentRow]) -> Result<()> {
    let file = File::create(out_path)?;
    let mut wtr = Writer::from_writer(file);

    wtr.write_record([
        "comment_id",
        "parent_id",
        "video_id",
        "author",
        "author_channel_id",
        "published_at",
        "like_count",
        "text",
    ])?;

    for row in rows {
        wtr.write_record([
            row.comment_id.as_str(),
            row.parent_id.as_str(),
            row.video_id.as_str(),
            row.author.as_str(),
            row.author_channel_id.as_str(),
            row.published_at.as_str(),
            &row.like_count.to_string(),
            row.text.as_str(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

fn parse_video_id(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    if is_probable_video_id(trimmed) && !trimmed.contains("http") {
        return Some(trimmed.to_string());
    }

    let url = Url::parse(trimmed).ok()?;

    if url.host_str().unwrap_or("").contains("youtu.be") {
        let id = url.path().trim_start_matches('/');
        if is_probable_video_id(id) {
            return Some(id.to_string());
        }
    }

    if let Some(v) = url.query_pairs().find(|(k, _)| k == "v").map(|(_, v)| v) {
        if is_probable_video_id(&v) {
            return Some(v.to_string());
        }
    }

    let segments: Vec<_> = url.path_segments().map(|s| s.collect()).unwrap_or_default();
    if let Some(pos) = segments.iter().position(|s| *s == "shorts") {
        if let Some(id) = segments.get(pos + 1) {
            if is_probable_video_id(id) {
                return Some((*id).to_string());
            }
        }
    }

    None
}

fn is_probable_video_id(value: &str) -> bool {
    let len = value.len();
    len >= 6 && len <= 32 && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
