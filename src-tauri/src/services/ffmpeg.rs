use crate::models::{ValidatedClip, ClipProgress};
use std::path::Path;
use std::fs;
use tauri::api::process::{Command, CommandEvent};

pub struct FFmpegService;

impl FFmpegService {
    /// Generate clips from a video file
    pub async fn generate_clips<F>(
        video_path: String,
        clips: Vec<ValidatedClip>,
        progress_callback: F,
    ) -> Result<String, String>
    where
        F: Fn(ClipProgress),
    {
        // Create output directory
        let video_path_obj = Path::new(&video_path);
        let video_name = video_path_obj
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid video filename")?;
        
        let output_dir = video_path_obj
            .parent()
            .ok_or("Cannot determine output directory")?
            .join(format!("{}_Clips", video_name));
        
        fs::create_dir_all(&output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
        
        // Process each clip
        for (index, clip) in clips.iter().enumerate() {
            let output_file = output_dir.join(format!(
                "{}_{}.mp4",
                index + 1,
                clip.sanitized_filename
            ));
            
            Self::extract_clip(
                &video_path,
                &clip.start_time,
                &clip.end_time,
                output_file.to_str().unwrap(),
            ).await?;
            
            progress_callback(ClipProgress {
                current: index + 1,
                total: clips.len(),
            });
        }
        
        Ok(output_dir.to_str().unwrap().to_string())
    }
    
    async fn extract_clip(
        input_path: &str,
        start_time: &str,
        end_time: &str,
        output_path: &str,
    ) -> Result<(), String> {
        // LOG: Command details
        println!("=== FFmpeg Clip Extraction ===");
        println!("Input: {}", input_path);
        println!("Start: {}", start_time);
        println!("End: {}", end_time);
        println!("Output: {}", output_path);
        
        // Use H.264 encoding instead of stream copy to support all codecs (ProRes, etc.)
        let args = vec![
            "-i", input_path,
            "-ss", start_time,
            "-to", end_time,
            "-c:v", "libx264",      // H.264 video codec (universal compatibility)
            "-preset", "fast",       // Encoding speed (fast, medium, slow)
            "-crf", "23",            // Quality: 18-28 (lower = better, 23 = default)
            "-c:a", "aac",           // AAC audio codec
            "-b:a", "192k",          // Audio bitrate
            "-movflags", "+faststart", // Enable streaming
            "-y",                    // Overwrite output
            output_path,
        ];
        
        println!("FFmpeg args: {:?}", args);
        
        let (mut rx, _child) = Command::new_sidecar("ffmpeg")
            .map_err(|e| {
                let err_msg = format!("Failed to find FFmpeg binary: {}", e);
                println!("ERROR: {}", err_msg);
                err_msg
            })?
            .args(&args)
            .spawn()
            .map_err(|e| {
                let err_msg = format!("Failed to spawn FFmpeg: {}", e);
                println!("ERROR: {}", err_msg);
                err_msg
            })?;
        
        println!("FFmpeg process spawned successfully");
        
        // Monitor process output
        let mut stdout_lines = Vec::new();
        let mut stderr_lines = Vec::new();
        
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    println!("FFmpeg STDOUT: {}", line);
                    stdout_lines.push(line.clone());
                }
                CommandEvent::Stderr(line) => {
                    println!("FFmpeg STDERR: {}", line);
                    stderr_lines.push(line.clone());
                }
                CommandEvent::Error(error) => {
                    println!("FFmpeg CommandEvent::Error: {}", error);
                    return Err(format!("FFmpeg error: {}", error));
                }
                CommandEvent::Terminated(payload) => {
                    println!("FFmpeg terminated with code: {:?}", payload.code);
                    println!("Signal: {:?}", payload.signal);
                    
                    if payload.code != Some(0) {
                        // Log full stderr for debugging
                        println!("=== FULL STDERR ===");
                        for line in &stderr_lines {
                            println!("{}", line);
                        }
                        println!("=== END STDERR ===");
                        
                        return Err(format!(
                            "FFmpeg exited with code: {:?}. Check console logs for details.",
                            payload.code
                        ));
                    }
                    println!("FFmpeg completed successfully");
                    break;
                }
                _ => {
                    println!("FFmpeg other event: {:?}", event);
                }
            }
        }
        
        Ok(())
    }
}
