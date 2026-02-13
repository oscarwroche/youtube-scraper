# YouTube Comment Scraper

Download comments from any YouTube video to a CSV file.

## How to Install (No Developer Experience Needed)
1. Go to the GitHub repo page.
2. Click **Code → Download ZIP**.
3. Unzip the file (Windows: right‑click the ZIP → **Extract All**).
4. Open Chrome and go to `chrome://extensions`.
5. Turn on **Developer mode** (top right).
6. Click **Load unpacked** and select the unzipped folder.
7. Pin the extension.

## How to Use
1. Open a YouTube video.
2. Click the extension icon.
3. Paste your YouTube Data API key and click **Save** (stored locally).
4. Click **Download CSV**.

Your file will download as `<videoId>.csv`.

## Windows Tips
- If Chrome blocks the extension, click **Keep** or **Allow**.
- If you can’t find the downloaded CSV, check your **Downloads** folder.

## Get a YouTube API Key
1. Open Google Cloud Console: https://console.cloud.google.com/
2. Enable **YouTube Data API v3**: https://console.cloud.google.com/apis/library/youtube.googleapis.com
3. Create an **API key**: https://console.cloud.google.com/apis/credentials

## Notes
- Replies are included.
- Large videos can take time and use API quota.
