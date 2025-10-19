use crate::models::{ClipSuggestion, ValidatedClip};
use crate::services::{VttParser, OpenAIService};
use crate::commands::settings::get_api_key;
use tauri::{command, AppHandle};

// Need Clone for ClipSuggestion in validation logging

#[command]
pub async fn analyze_transcript_for_clips(
    app_handle: AppHandle,
    transcript_path: String,
    _video_path: String,
    user_context: Option<String>,
) -> Result<Vec<ValidatedClip>, String> {
    // 1. Get API key
    let api_key = get_api_key(app_handle).await?
        .ok_or("No API key configured. Please add your OpenAI API key in Settings.")?;
    
    // 2. Parse VTT file
    let vtt_cues = VttParser::parse(&transcript_path)?;
    
    // 3. Format VTT with full structure (timestamps, cue numbers) for GPT-5
    let formatted_vtt = VttParser::get_formatted_vtt(&vtt_cues);
    
    println!("=== Analyzing Transcript ===");
    println!("VTT cues: {}", vtt_cues.len());
    println!("User context: {}", user_context.as_deref().unwrap_or("None"));
    
    // 4. Call OpenAI GPT-5-mini
    let raw_clips = OpenAIService::analyze_transcript(
        &api_key,
        &formatted_vtt,
        user_context.as_deref(),
    ).await?;
    
    // 5. Validate and map timestamps to actual VTT cues
    println!("=== Validating {} Suggested Clips ===", raw_clips.len());
    
    let validated_clips: Vec<ValidatedClip> = raw_clips
        .into_iter()
        .filter_map(|clip| {
            let result = validate_and_map_clip(clip.clone(), &vtt_cues);
            if result.is_none() {
                println!("⚠️  Rejected clip: {} ({} -> {})", clip.title, clip.start_time, clip.end_time);
            }
            result
        })
        .collect();
    
    println!("✅ Validated {} clips", validated_clips.len());
    
    Ok(validated_clips)
}

fn validate_and_map_clip(
    clip: ClipSuggestion,
    vtt_cues: &[crate::models::VttCue],
) -> Option<ValidatedClip> {
    // Find the closest VTT cues for start and end times
    let start_cue = VttParser::find_closest_cue(vtt_cues, &clip.start_time)?;
    let end_cue = VttParser::find_closest_cue(vtt_cues, &clip.end_time)?;
    
    // Convert to FFmpeg format (remove milliseconds)
    let start_time = VttParser::vtt_to_ffmpeg_timestamp(&start_cue.start_timestamp);
    let end_time = VttParser::vtt_to_ffmpeg_timestamp(&end_cue.end_timestamp);
    
    // Verify end is after start
    if VttParser::timestamp_to_seconds(&end_time)? 
        <= VttParser::timestamp_to_seconds(&start_time)? {
        return None;
    }
    
    // Sanitize filename
    let sanitized_filename = sanitize_filename(&clip.title);
    
    Some(ValidatedClip {
        id: uuid::Uuid::new_v4().to_string(),
        title: clip.title,
        start_time,
        end_time,
        sanitized_filename,
        is_selected: true,
    })
}

fn sanitize_filename(title: &str) -> String {
    title
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}
