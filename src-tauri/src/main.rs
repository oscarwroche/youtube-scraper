use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_config_dir;
use tauri::api::shell;

#[derive(Serialize, Deserialize, Default)]
struct AppConfig {
    api_key: Option<String>,
}

fn config_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_id = app_handle.config().tauri.bundle.identifier.clone();
    let mut dir = app_config_dir(app_handle.config()).unwrap_or_else(|| PathBuf::from("."));
    dir.push(app_id);
    dir.push("config.json");
    dir
}

fn load_config(app_handle: &tauri::AppHandle) -> AppConfig {
    let path = config_path(app_handle);
    let data = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_config(app_handle: &tauri::AppHandle, config: &AppConfig) -> Result<(), String> {
    let path = config_path(app_handle);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_api_key(app_handle: tauri::AppHandle) -> Option<String> {
    load_config(&app_handle).api_key
}

#[tauri::command]
fn set_api_key(app_handle: tauri::AppHandle, api_key: String) -> Result<(), String> {
    let mut config = load_config(&app_handle);
    config.api_key = Some(api_key);
    save_config(&app_handle, &config)
}

#[tauri::command]
async fn run_scrape(app_handle: tauri::AppHandle, api_key: String, video: String) -> Result<String, String> {
    if api_key.trim().is_empty() {
        return Err("API key is required".to_string());
    }
    let out_path = youtube_comment_scraper::scrape_to_csv(&api_key, &video, None)
        .await
        .map_err(|e| e.to_string())?;

    // Return absolute path for convenience
    let abs = fs::canonicalize(out_path).map_err(|e| e.to_string())?;
    Ok(abs.to_string_lossy().to_string())
}

#[tauri::command]
fn open_path(app_handle: tauri::AppHandle, path: String) -> Result<(), String> {
    shell::open(&app_handle.shell_scope(), path, None).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_api_key, set_api_key, run_scrape, open_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
