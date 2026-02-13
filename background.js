const API_BASE = "https://www.googleapis.com/youtube/v3";

chrome.runtime.onMessage.addListener((message, _sender, sendResponse) => {
  if (message?.type === "SCRAPE") {
    scrape(message.apiKey, message.video)
      .then((count) => sendResponse({ ok: true, count }))
      .catch((err) => sendResponse({ ok: false, error: String(err?.message || err) }));
    return true;
  }
});

async function scrape(apiKey, videoInput) {
  const videoId = parseVideoId(videoInput);
  if (!videoId) throw new Error("Could not parse a video ID.");

  const rows = await fetchAllComments(apiKey, videoId);
  const csv = toCsv(rows);
  const blob = new Blob([csv], { type: "text/csv" });
  const url = URL.createObjectURL(blob);

  await chrome.downloads.download({
    url,
    filename: `${videoId}.csv`,
    saveAs: true
  });

  setTimeout(() => URL.revokeObjectURL(url), 10000);
  return rows.length;
}

async function fetchAllComments(apiKey, videoId) {
  const rows = [];
  let pageToken = "";

  do {
    const params = new URLSearchParams({
      part: "snippet",
      videoId,
      maxResults: "100",
      textFormat: "plainText",
      key: apiKey
    });
    if (pageToken) params.set("pageToken", pageToken);

    const data = await fetchJson(`${API_BASE}/commentThreads?${params}`);

    for (const item of data.items || []) {
      const top = item?.snippet?.topLevelComment?.snippet;
      const topId = item?.snippet?.topLevelComment?.id;
      if (top && topId) {
        rows.push(makeRow(topId, "", videoId, top));
      }

      const replyCount = item?.snippet?.totalReplyCount || 0;
      if (replyCount > 0 && topId) {
        const replies = await fetchReplies(apiKey, topId, videoId);
        rows.push(...replies);
      }
    }

    pageToken = data.nextPageToken || "";
  } while (pageToken);

  return rows;
}

async function fetchReplies(apiKey, parentId, videoId) {
  const rows = [];
  let pageToken = "";

  do {
    const params = new URLSearchParams({
      part: "snippet",
      parentId,
      maxResults: "100",
      textFormat: "plainText",
      key: apiKey
    });
    if (pageToken) params.set("pageToken", pageToken);

    const data = await fetchJson(`${API_BASE}/comments?${params}`);

    for (const item of data.items || []) {
      const snip = item?.snippet;
      if (!snip) continue;
      rows.push(makeRow(item?.id || "", parentId, videoId, snip));
    }

    pageToken = data.nextPageToken || "";
  } while (pageToken);

  return rows;
}

function makeRow(commentId, parentId, videoId, snip) {
  return {
    comment_id: commentId,
    parent_id: parentId,
    video_id: videoId,
    author: snip.authorDisplayName || "",
    author_channel_id: snip.authorChannelId?.value || "",
    published_at: snip.publishedAt || "",
    like_count: snip.likeCount || 0,
    text: snip.textDisplay || ""
  };
}

async function fetchJson(url) {
  const res = await fetch(url);
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`HTTP ${res.status}: ${text}`);
  }
  return res.json();
}

function toCsv(rows) {
  const header = [
    "comment_id",
    "parent_id",
    "video_id",
    "author",
    "author_channel_id",
    "published_at",
    "like_count",
    "text"
  ];

  const lines = [header.join(",")];
  for (const row of rows) {
    const values = header.map((k) => csvEscape(row[k]));
    lines.push(values.join(","));
  }
  return lines.join("\n");
}

function csvEscape(value) {
  const str = String(value ?? "");
  if (/[\n",]/.test(str)) {
    return `"${str.replace(/"/g, "\"\"")}"`;
  }
  return str;
}

function parseVideoId(input) {
  const trimmed = input.trim();
  if (!trimmed) return null;

  if (/^[a-zA-Z0-9_-]{6,}$/.test(trimmed) && !trimmed.includes("http")) {
    return trimmed;
  }

  try {
    const url = new URL(trimmed);
    if (url.hostname.includes("youtu.be")) {
      const id = url.pathname.replace("/", "");
      return id || null;
    }
    if (url.searchParams.has("v")) {
      return url.searchParams.get("v");
    }
    const parts = url.pathname.split("/").filter(Boolean);
    const shortsIndex = parts.indexOf("shorts");
    if (shortsIndex >= 0 && parts[shortsIndex + 1]) {
      return parts[shortsIndex + 1];
    }
  } catch {
    return null;
  }

  return null;
}
