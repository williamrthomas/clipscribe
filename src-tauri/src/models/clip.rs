use serde::{Deserialize, Serialize};

/// OpenAI response structure
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClipSuggestion {
    pub title: String,
    pub start_time: String,  // HH:MM:SS or MM:SS
    pub end_time: String,
}

/// Validated clip (ready for FFmpeg)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidatedClip {
    pub id: String,
    pub title: String,
    #[serde(rename = "startTime")]
    pub start_time: String,       // FFmpeg-compatible format
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "sanitizedFilename")]
    pub sanitized_filename: String,
    #[serde(rename = "isSelected")]
    pub is_selected: bool,
}

/// Processing result
#[derive(Serialize)]
pub struct ProcessingResult {
    pub output_directory: String,
    pub clip_count: usize,
}

/// Progress event payload
#[derive(Clone, Serialize)]
pub struct ClipProgress {
    pub current: usize,
    pub total: usize,
}
