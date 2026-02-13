const apiKeyInput = document.getElementById("apiKey");
const saveKeyButton = document.getElementById("saveKey");
const keyStatus = document.getElementById("keyStatus");
const videoUrlInput = document.getElementById("videoUrl");
const runButton = document.getElementById("run");
const runStatus = document.getElementById("runStatus");

function setStatus(el, msg) {
  el.textContent = msg;
}

async function loadApiKey() {
  const { apiKey } = await chrome.storage.local.get("apiKey");
  if (apiKey) {
    apiKeyInput.value = apiKey;
    setStatus(keyStatus, "API key loaded.");
  }
}

async function getActiveTabUrl() {
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  return tab?.url || "";
}

async function preloadActiveUrl() {
  const url = await getActiveTabUrl();
  if (url && !videoUrlInput.value) {
    videoUrlInput.value = url;
  }
}

saveKeyButton.addEventListener("click", async () => {
  const apiKey = apiKeyInput.value.trim();
  if (!apiKey) {
    setStatus(keyStatus, "Enter an API key first.");
    return;
  }
  await chrome.storage.local.set({ apiKey });
  setStatus(keyStatus, "Saved.");
});

runButton.addEventListener("click", async () => {
  const apiKey = apiKeyInput.value.trim();
  let video = videoUrlInput.value.trim();

  if (!apiKey) {
    setStatus(runStatus, "API key is required.");
    return;
  }

  if (!video) {
    video = await getActiveTabUrl();
  }

  if (!video) {
    setStatus(runStatus, "Enter a YouTube URL or open a video tab.");
    return;
  }

  setStatus(runStatus, "Downloading comments...");

  chrome.runtime.sendMessage(
    { type: "SCRAPE", apiKey, video },
    (response) => {
      if (chrome.runtime.lastError) {
        setStatus(runStatus, chrome.runtime.lastError.message);
        return;
      }
      if (!response || !response.ok) {
        setStatus(runStatus, response?.error || "Failed.");
        return;
      }
      setStatus(runStatus, `Saved ${response.count} comments.`);
    }
  );
});

loadApiKey();
preloadActiveUrl();
