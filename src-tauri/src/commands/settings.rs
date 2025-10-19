use tauri::command;
use tauri::AppHandle;
use std::fs;
use std::path::PathBuf;

fn get_settings_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("Failed to get app data directory")?;
    
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    Ok(app_dir.join("settings.json"))
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    openai_api_key: Option<String>,
}

#[command]
pub async fn save_api_key(
    app_handle: AppHandle,
    api_key: String,
) -> Result<(), String> {
    let settings_path = get_settings_path(&app_handle)?;
    
    let settings = Settings {
        openai_api_key: Some(api_key),
    };
    
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| e.to_string())?;
    
    fs::write(settings_path, json)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[command]
pub async fn get_api_key(
    app_handle: AppHandle,
) -> Result<Option<String>, String> {
    let settings_path = get_settings_path(&app_handle)?;
    
    if !settings_path.exists() {
        return Ok(None);
    }
    
    let json = fs::read_to_string(settings_path)
        .map_err(|e| e.to_string())?;
    
    let settings: Settings = serde_json::from_str(&json)
        .map_err(|e| e.to_string())?;
    
    Ok(settings.openai_api_key)
}

#[command]
pub async fn validate_api_key(api_key: String) -> Result<bool, String> {
    // Make a minimal API call to verify the key
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.openai.com/v1/models")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(response.status().is_success())
}
