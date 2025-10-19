use crate::services::WhisperService;
use crate::commands::settings::get_api_key;
use tauri::{command, AppHandle, Manager};

#[command]
pub async fn generate_transcript_from_video(
    app_handle: AppHandle,
    video_path: String,
) -> Result<String, String> {
    // Get API key
    let api_key = get_api_key(app_handle.clone()).await?
        .ok_or("No API key configured. Please add your OpenAI API key in Settings.")?;
    
    // Generate transcript with progress updates
    let vtt_path = WhisperService::transcribe_video(
        &api_key,
        &video_path,
        |message| {
            let _ = app_handle.emit_all("transcription-progress", message);
        },
    ).await?;
    
    Ok(vtt_path)
}
