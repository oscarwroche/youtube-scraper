# YouTube Comment Scraper (Chrome Extension)

Download YouTube comments to CSV directly from your browser.

## Install (Developer Mode)
1. Open Chrome and go to `chrome://extensions`.
2. Enable **Developer mode** (top right).
3. Click **Load unpacked** and select this folder.
4. Pin the extension.

## Use
1. Open a YouTube video.
2. Click the extension icon.
3. Paste your YouTube Data API key and click **Save** (stored locally).
4. Click **Download CSV**.

The extension downloads `<videoId>.csv`.

## Get a YouTube API Key
1. Google Cloud Console: https://console.cloud.google.com/
2. Enable YouTube Data API v3: https://console.cloud.google.com/apis/library/youtube.googleapis.com
3. Create an API key: https://console.cloud.google.com/apis/credentials

## Notes
- Replies are included.
- Large videos may take time and consume API quota.
