const { invoke } = window.__TAURI__.tauri;

const apiKeyInput = document.getElementById("apiKey");
const saveKeyButton = document.getElementById("saveKey");
const keyStatus = document.getElementById("keyStatus");
const videoUrlInput = document.getElementById("videoUrl");
const runButton = document.getElementById("run");
const runStatus = document.getElementById("runStatus");
const resultCard = document.getElementById("resultCard");
const outputPathInput = document.getElementById("outputPath");
const openFileButton = document.getElementById("openFile");

async function loadKey() {
  try {
    const key = await invoke("get_api_key");
    if (key) {
      apiKeyInput.value = key;
      keyStatus.textContent = "API key loaded.";
    }
  } catch (err) {
    keyStatus.textContent = `Could not load key: ${err}`;
  }
}

saveKeyButton.addEventListener("click", async () => {
  const key = apiKeyInput.value.trim();
  if (!key) {
    keyStatus.textContent = "Enter an API key before saving.";
    return;
  }
  keyStatus.textContent = "Saving...";
  try {
    await invoke("set_api_key", { apiKey: key });
    keyStatus.textContent = "Saved.";
  } catch (err) {
    keyStatus.textContent = `Save failed: ${err}`;
  }
});

runButton.addEventListener("click", async () => {
  const key = apiKeyInput.value.trim();
  const video = videoUrlInput.value.trim();
  if (!key) {
    runStatus.textContent = "API key is required.";
    return;
  }
  if (!video) {
    runStatus.textContent = "Please enter a video URL or ID.";
    return;
  }
  runStatus.textContent = "Running...";
  resultCard.hidden = true;

  try {
    const outPath = await invoke("run_scrape", { apiKey: key, video });
    outputPathInput.value = outPath;
    resultCard.hidden = false;
    runStatus.textContent = "Done.";
  } catch (err) {
    runStatus.textContent = `Failed: ${err}`;
  }
});

openFileButton.addEventListener("click", async () => {
  const path = outputPathInput.value.trim();
  if (!path) return;
  try {
    await invoke("open_path", { path });
  } catch (err) {
    runStatus.textContent = `Open failed: ${err}`;
  }
});

loadKey();
