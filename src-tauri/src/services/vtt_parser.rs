use crate::models::VttCue;
use regex::Regex;
use std::fs;

pub struct VttParser;

impl VttParser {
    pub fn parse(file_path: &str) -> Result<Vec<VttCue>, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read VTT file: {}", e))?;
        
        // Verify WEBVTT header
        if !content.starts_with("WEBVTT") {
            return Err("Invalid VTT file: missing WEBVTT header".to_string());
        }
        
        let mut cues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        // Regex for timestamp line: 00:01:14.500 --> 00:01:18.200
        let timestamp_regex = Regex::new(
            r"^(\d{2}:\d{2}:\d{2}\.\d{3})\s*-->\s*(\d{2}:\d{2}:\d{2}\.\d{3})"
        ).unwrap();
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Skip empty lines and identifiers
            if line.is_empty() || line.parse::<u32>().is_ok() {
                i += 1;
                continue;
            }
            
            // Check if this is a timestamp line
            if let Some(captures) = timestamp_regex.captures(line) {
                let start = captures.get(1).unwrap().as_str().to_string();
                let end = captures.get(2).unwrap().as_str().to_string();
                
                // Collect text lines until we hit an empty line or another timestamp
                let mut text_lines = Vec::new();
                i += 1;
                
                while i < lines.len() {
                    let text_line = lines[i].trim();
                    if text_line.is_empty() {
                        break;
                    }
                    // Check if it's another timestamp
                    if timestamp_regex.is_match(text_line) {
                        break;
                    }
                    text_lines.push(text_line);
                    i += 1;
                }
                
                cues.push(VttCue {
                    start_timestamp: start,
                    end_timestamp: end,
                    text: text_lines.join(" "),
                });
            } else {
                i += 1;
            }
        }
        
        Ok(cues)
    }
    
    /// Convert VTT timestamp (HH:MM:SS.mmm) to FFmpeg format (HH:MM:SS)
    pub fn vtt_to_ffmpeg_timestamp(vtt_time: &str) -> String {
        // Remove milliseconds: 00:01:14.500 -> 00:01:14
        vtt_time.split('.').next().unwrap_or(vtt_time).to_string()
    }
    
    /// Find the VTT cue closest to a given timestamp
    pub fn find_closest_cue<'a>(cues: &'a [VttCue], target_time: &str) -> Option<&'a VttCue> {
        // Convert target to seconds for comparison
        let target_seconds = Self::timestamp_to_seconds(target_time)?;
        
        cues.iter()
            .min_by_key(|cue| {
                let cue_seconds = Self::timestamp_to_seconds(&cue.start_timestamp).unwrap_or(0);
                ((cue_seconds as i32) - (target_seconds as i32)).abs()
            })
    }
    
    pub fn timestamp_to_seconds(timestamp: &str) -> Option<u32> {
        let parts: Vec<&str> = timestamp.split(':').collect();
        if parts.len() != 3 {
            return None;
        }
        
        let hours: u32 = parts[0].parse().ok()?;
        let minutes: u32 = parts[1].parse().ok()?;
        let seconds: u32 = parts[2].split('.').next()?.parse().ok()?;
        
        Some(hours * 3600 + minutes * 60 + seconds)
    }
    
    /// Get full transcript as plain text
    pub fn get_full_transcript(cues: &[VttCue]) -> String {
        cues.iter()
            .map(|cue| format!("[{}] {}", cue.start_timestamp, cue.text))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
