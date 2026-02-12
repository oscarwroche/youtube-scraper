# YouTube Comment Scraper

Download comments from a YouTube video and export them to CSV. This is a small desktop app that saves your API key locally so you only enter it once.

## Download
Go to the latest Release and download the installer for your OS:
- Latest Releases: https://github.com/oscarwroche/youtube-scraper/releases/latest
- Windows: `.msi` (or `nsis` installer)
- macOS: `.app` bundle
- Linux: `.AppImage` or `.deb`

## Get A YouTube API Key
This app uses the official YouTube Data API. You’ll need a free API key from Google Cloud.

1. Open Google Cloud Console and create a new project (or select one): https://console.cloud.google.com/
2. Enable **YouTube Data API v3** for that project: https://console.cloud.google.com/apis/library/youtube.googleapis.com
3. Create an **API key** under **APIs & Services → Credentials**: https://console.cloud.google.com/apis/credentials
4. (Recommended) Restrict the key to prevent abuse.

## How It Works
1. Open the app and paste your YouTube Data API key (saved locally).
2. Paste a video URL or ID.
3. Click Run.
4. The CSV is created in the same folder as the app.

## Where The CSV Is Saved
The file is named `<videoId>.csv` and overwrites any existing file with the same name.

## API Key Storage
The app stores your key in a local config file (plain text). Location varies by OS:
- macOS: `~/Library/Application Support/<bundle-id>/config.json`
- Windows: `%APPDATA%\\<bundle-id>\\config.json`
- Linux: `~/.config/<bundle-id>/config.json`

Bundle ID is set in `src-tauri/tauri.conf.json`.

## Contributing
You’ll need Rust installed.

Build the CLI:
```bash
cargo build --release
```

Run the desktop app:
```bash
cargo install tauri-cli
cargo tauri dev
```

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
