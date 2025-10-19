use crate::models::{ValidatedClip, ProcessingResult};
use crate::services::FFmpegService;
use tauri::{command, AppHandle, Manager};

#[command]
pub async fn generate_clips(
    app_handle: AppHandle,
    video_path: String,
    clips: Vec<ValidatedClip>,
) -> Result<ProcessingResult, String> {
    let clips_to_generate: Vec<ValidatedClip> = clips
        .into_iter()
        .filter(|c| c.is_selected)  // Only generate selected clips
        .collect();
    
    if clips_to_generate.is_empty() {
        return Err("No clips selected".to_string());
    }
    
    let clip_count = clips_to_generate.len();
    
    // Progress tracking using Tauri events
    let output_dir = FFmpegService::generate_clips(
        video_path,
        clips_to_generate,
        |progress| {
            let _ = app_handle.emit_all("clip-progress", progress);
        },
    ).await?;
    
    Ok(ProcessingResult {
        output_directory: output_dir,
        clip_count,
    })
}

#[command]
pub async fn open_in_file_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}
