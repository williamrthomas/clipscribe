use serde::{Deserialize, Serialize};

/// VTT cue structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VttCue {
    pub start_timestamp: String,  // HH:MM:SS.mmm
    pub end_timestamp: String,
    pub text: String,
}
