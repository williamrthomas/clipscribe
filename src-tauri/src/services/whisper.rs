use reqwest::Client;
use std::path::Path;
use std::fs;
use tauri::api::process::{Command, CommandEvent};

pub struct WhisperService;

impl WhisperService {
    /// Extract audio from video and transcribe using OpenAI Whisper
    pub async fn transcribe_video(
        api_key: &str,
        video_path: &str,
        progress_callback: impl Fn(String),
    ) -> Result<String, String> {
        // Step 1: Extract audio from video using FFmpeg
        progress_callback("Extracting audio from video...".to_string());
        let audio_path = Self::extract_audio(video_path).await?;
        
        // Step 2: Transcribe audio using Whisper API
        progress_callback("Transcribing audio with Whisper AI...".to_string());
        let vtt_content = Self::transcribe_audio(api_key, &audio_path).await?;
        
        // Step 3: Save VTT file
        let vtt_path = Self::save_vtt_file(video_path, &vtt_content)?;
        
        // Step 4: Clean up temporary audio file
        let _ = fs::remove_file(&audio_path);
        
        progress_callback("Transcript generated successfully!".to_string());
        Ok(vtt_path)
    }
    
    /// Extract audio from video to temporary MP3 file
    async fn extract_audio(video_path: &str) -> Result<String, String> {
        println!("=== Whisper: Audio Extraction ===");
        println!("Video path: {}", video_path);
        
        let video_path_obj = Path::new(video_path);
        let audio_path = video_path_obj
            .parent()
            .ok_or("Invalid video path")?
            .join(format!(
                "{}_temp_audio.mp3",
                video_path_obj.file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or("Invalid filename")?
            ));
        
        let audio_path_str = audio_path.to_str().ok_or("Invalid path")?;
        println!("Audio output: {}", audio_path_str);
        
        let args = vec![
            "-i", video_path,
            "-vn",
            "-acodec", "libmp3lame",
            "-ar", "16000",
            "-ac", "1",
            "-b:a", "64k",
            "-y",
            audio_path_str,
        ];
        
        println!("FFmpeg audio extraction args: {:?}", args);
        
        // Use FFmpeg to extract audio
        let (mut rx, _child) = Command::new_sidecar("ffmpeg")
            .map_err(|e| {
                let err_msg = format!("Failed to find FFmpeg: {}", e);
                println!("ERROR: {}", err_msg);
                err_msg
            })?
            .args(&args)
            .spawn()
            .map_err(|e| {
                let err_msg = format!("Failed to spawn FFmpeg for audio extraction: {}", e);
                println!("ERROR: {}", err_msg);
                err_msg
            })?;
        
        println!("FFmpeg audio extraction spawned successfully");
        
        // Wait for completion
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    println!("Audio extraction STDOUT: {}", line);
                }
                CommandEvent::Stderr(line) => {
                    println!("Audio extraction STDERR: {}", line);
                }
                CommandEvent::Error(error) => {
                    println!("Audio extraction error: {}", error);
                    return Err(format!("FFmpeg error: {}", error));
                }
                CommandEvent::Terminated(payload) => {
                    println!("Audio extraction terminated: code={:?}, signal={:?}", 
                            payload.code, payload.signal);
                    if payload.code != Some(0) {
                        return Err(format!("FFmpeg failed with code: {:?}", payload.code));
                    }
                    println!("Audio extraction completed successfully");
                    break;
                }
                _ => {}
            }
        }
        
        Ok(audio_path_str.to_string())
    }
    
    /// Transcribe audio file using OpenAI Whisper API
    async fn transcribe_audio(api_key: &str, audio_path: &str) -> Result<String, String> {
        let client = Client::new();
        
        // Read audio file
        let audio_data = fs::read(audio_path)
            .map_err(|e| format!("Failed to read audio file: {}", e))?;
        
        // Whisper API has 25MB limit - check file size
        if audio_data.len() > 25 * 1024 * 1024 {
            return Err("Audio file too large (max 25MB). Try a shorter video.".to_string());
        }
        
        // Create multipart form
        let form = reqwest::multipart::Form::new()
            .part(
                "file",
                reqwest::multipart::Part::bytes(audio_data)
                    .file_name("audio.mp3")
                    .mime_str("audio/mpeg")
                    .map_err(|e| format!("Failed to create form part: {}", e))?,
            )
            .text("model", "whisper-1")
            .text("response_format", "vtt")
            .text("language", "en"); // Can be made configurable
        
        let response = client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Whisper API request failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Whisper API error ({}): {}", status, error_text));
        }
        
        // Get VTT content
        let vtt_content = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        Ok(vtt_content)
    }
    
    /// Save VTT content to file next to video
    fn save_vtt_file(video_path: &str, vtt_content: &str) -> Result<String, String> {
        let video_path_obj = Path::new(video_path);
        let vtt_path = video_path_obj
            .parent()
            .ok_or("Invalid video path")?
            .join(format!(
                "{}.vtt",
                video_path_obj.file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or("Invalid filename")?
            ));
        
        fs::write(&vtt_path, vtt_content)
            .map_err(|e| format!("Failed to save VTT file: {}", e))?;
        
        Ok(vtt_path.to_str().ok_or("Invalid path")?.to_string())
    }
}
