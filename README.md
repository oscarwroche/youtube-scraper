# YouTube Comment Scraper (Rust)

This CLI fetches YouTube comments for a video and exports them to a CSV file named `<videoId>.csv`. It supports API key via flag or environment variable and accepts either a full URL or a raw video ID.

## Requirements
- Rust toolchain (for building once)
- YouTube Data API v3 key

## Build

```bash
cargo build --release
```

The executable will be at `target/release/youtube_comment_scraper`.

## Usage

```bash
# API key via env
export YOUTUBE_API_KEY="your_api_key_here"
./target/release/youtube_comment_scraper "https://www.youtube.com/watch?v=dQw4w9WgXcQ"

# API key via flag
./target/release/youtube_comment_scraper --api-key "your_api_key_here" dQw4w9WgXcQ
```

The script writes `dQw4w9WgXcQ.csv` to the current directory and overwrites it if it exists.

## Distribution (GitHub Releases)
This repo includes a GitHub Actions workflow that builds standalone binaries for:
- Windows (x64)
- macOS (x64 + arm64)
- Linux (x64)

### One-time setup
1. Create a new GitHub repo (public or private).
2. Add it as `origin` and push:

```bash
git remote add origin <YOUR_REPO_URL>
git branch -M main
git push -u origin main
```

### Versioning and releases
We use semantic version tags like `v1.0.0`. When you push a tag, GitHub Actions builds binaries and attaches them to a GitHub Release.

```bash
git tag v1.0.0
git push origin v1.0.0
```

Your brother can then download the Windows zip from the Release page and run:

```powershell
youtube_comment_scraper.exe --api-key "YOUR_KEY" https://www.youtube.com/watch?v=dQw4w9WgXcQ
```

### Notes
- The workflow lives at `.github/workflows/release.yml`.
- Each tag creates or updates a Release with built artifacts.

## Output Columns
- `comment_id`
- `parent_id` (empty for top-level comments)
- `video_id`
- `author`
- `author_channel_id`
- `published_at`
- `like_count`
- `text`

## Notes
- This uses the official YouTube Data API. Some videos may have comments disabled.
- Replies are fetched via the `comments` endpoint, which may increase API usage.
