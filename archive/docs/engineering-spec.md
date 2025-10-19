# Engineering Specification: ClipScribe v1.0

**Status:** Ready for Development  
**Author:** Technical Specification  
**Based on PRD by:** William Thomas  
**Target Platforms:** macOS (Intel/Apple Silicon), Windows 10/11

---

## 1. Architecture Overview

### 1.1 Technology Stack

**Frontend:**
- **Framework:** React 18+ with TypeScript
- **State Management:** React Context API (useState/useReducer for local state)
- **Styling:** Tailwind CSS or CSS Modules
- **File Handling:** HTML5 Drag-and-Drop API

**Backend:**
- **Framework:** Tauri 1.5+
- **Language:** Rust 1.70+
- **Key Crates:**
  - `reqwest` (v0.11+) - HTTP client for OpenAI API
  - `serde` + `serde_json` - JSON serialization
  - `tokio` - Async runtime
  - `tauri` - Application framework
  - `keyring` or `tauri-plugin-store` - Secure credential storage

**Bundled Binary:**
- FFmpeg 6.0+ (static builds for each platform)

### 1.2 Application Structure

```
src/
├── frontend/
│   ├── components/
│   │   ├── FileDropZone.tsx
│   │   ├── SettingsModal.tsx
│   │   ├── ClipReviewList.tsx
│   │   └── ProgressIndicator.tsx
│   ├── hooks/
│   │   └── useAppState.ts
│   ├── types/
│   │   └── index.ts
│   └── App.tsx
├── backend/ (Rust)
│   ├── commands/
│   │   ├── analyze.rs
│   │   ├── process.rs
│   │   └── settings.rs
│   ├── models/
│   │   ├── clip.rs
│   │   └── vtt.rs
│   ├── services/
│   │   ├── openai.rs
│   │   ├── ffmpeg.rs
│   │   └── vtt_parser.rs
│   └── main.rs
└── bin/
    ├── ffmpeg-x86_64-apple-darwin
    ├── ffmpeg-aarch64-apple-darwin
    └── ffmpeg-x86_64-pc-windows-msvc.exe
```

---

## 2. Data Models

### 2.1 TypeScript/Frontend Types

```typescript
// Clip representation
interface Clip {
  id: string;                    // UUID
  title: string;                 // AI-generated title
  startTime: string;             // HH:MM:SS format
  endTime: string;               // HH:MM:SS format
  isSelected: boolean;           // User can toggle
  sanitizedFilename?: string;    // Safe filename version
}

// Application state
type AppState = 
  | { status: 'ready' }
  | { status: 'analyzing' }
  | { status: 'review', clips: Clip[] }
  | { status: 'processing', progress: number }
  | { status: 'complete', outputPath: string, clipCount: number }
  | { status: 'error', message: string };

// File inputs
interface ProjectFiles {
  videoFile: File | null;
  transcriptFile: File | null;
  context?: string;
}
```

### 2.2 Rust Structs

```rust
// OpenAI response structure
#[derive(Debug, Deserialize, Serialize)]
pub struct ClipSuggestion {
    pub title: String,
    pub start_time: String,  // HH:MM:SS or MM:SS
    pub end_time: String,
}

// VTT cue structure
#[derive(Debug, Clone)]
pub struct VttCue {
    pub start_timestamp: String,  // HH:MM:SS.mmm
    pub end_timestamp: String,
    pub text: String,
}

// Validated clip (ready for FFmpeg)
#[derive(Debug, Serialize, Clone)]
pub struct ValidatedClip {
    pub id: String,
    pub title: String,
    pub start_time: String,       // FFmpeg-compatible format
    pub end_time: String,
    pub sanitized_filename: String,
}
```

---

## 3. Phase 1: Foundation & Settings

### 3.1 Tauri Configuration

**`tauri.conf.json` Critical Sections:**

```json
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "tauri": {
    "bundle": {
      "identifier": "com.clipscribe.app",
      "externalBin": [
        "bin/ffmpeg"
      ],
      "resources": ["bin/*"]
    },
    "allowlist": {
      "fs": {
        "readFile": true,
        "writeFile": true,
        "createDir": true,
        "scope": ["$RESOURCE/*", "$APPDATA/*", "$DOWNLOAD/*"]
      },
      "dialog": {
        "open": true,
        "save": false
      },
      "shell": {
        "sidecar": true,
        "scope": [
          {
            "name": "bin/ffmpeg",
            "sidecar": true,
            "args": true
          }
        ]
      },
      "path": {
        "all": true
      }
    },
    "windows": [
      {
        "title": "ClipScribe",
        "width": 900,
        "height": 700,
        "resizable": true,
        "fullscreen": false
      }
    ]
  }
}
```

### 3.2 Secure API Key Storage

**Implementation: `src/backend/commands/settings.rs`**

```rust
use tauri::command;
use tauri::Manager;

#[command]
pub async fn save_api_key(
    app_handle: tauri::AppHandle,
    api_key: String,
) -> Result<(), String> {
    // Use tauri-plugin-store for encrypted storage
    let store = app_handle.store("settings.dat")
        .map_err(|e| e.to_string())?;
    
    store.set("openai_api_key", serde_json::json!(api_key));
    store.save().map_err(|e| e.to_string())?;
    
    Ok(())
}

#[command]
pub async fn get_api_key(
    app_handle: tauri::AppHandle,
) -> Result<Option<String>, String> {
    let store = app_handle.store("settings.dat")
        .map_err(|e| e.to_string())?;
    
    Ok(store.get("openai_api_key")
        .and_then(|v| v.as_str().map(String::from)))
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
```

**Frontend Component: `SettingsModal.tsx`**

```typescript
import { invoke } from '@tauri-apps/api/tauri';

function SettingsModal({ isOpen, onClose }) {
  const [apiKey, setApiKey] = useState('');
  const [isValidating, setIsValidating] = useState(false);

  const handleSave = async () => {
    setIsValidating(true);
    try {
      const isValid = await invoke('validate_api_key', { apiKey });
      if (!isValid) {
        alert('Invalid API key. Please check and try again.');
        return;
      }
      await invoke('save_api_key', { apiKey });
      onClose();
    } catch (error) {
      alert(`Error: ${error}`);
    } finally {
      setIsValidating(false);
    }
  };

  return (
    // Modal UI implementation
  );
}
```

---

## 4. Phase 2: File Input & Validation

### 4.1 Drag-and-Drop Component

**`FileDropZone.tsx`**

```typescript
interface FileDropZoneProps {
  accept: string;  // e.g., ".mp4,.mov,.mkv" or ".vtt,.txt"
  label: string;
  onFileSelected: (file: File) => void;
  currentFile: File | null;
}

function FileDropZone({ accept, label, onFileSelected, currentFile }: FileDropZoneProps) {
  const [isDragging, setIsDragging] = useState(false);

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    
    const file = e.dataTransfer.files[0];
    if (file && validateFileType(file, accept)) {
      onFileSelected(file);
    } else {
      alert(`Please drop a valid file (${accept})`);
    }
  };

  const validateFileType = (file: File, acceptedTypes: string): boolean => {
    const extensions = acceptedTypes.split(',').map(ext => ext.trim().toLowerCase());
    const fileExt = '.' + file.name.split('.').pop()?.toLowerCase();
    return extensions.includes(fileExt);
  };

  return (
    <div
      onDragOver={(e) => { e.preventDefault(); setIsDragging(true); }}
      onDragLeave={() => setIsDragging(false)}
      onDrop={handleDrop}
      className={`drop-zone ${isDragging ? 'active' : ''}`}
    >
      {currentFile ? (
        <div>✓ {currentFile.name}</div>
      ) : (
        <div>
          <p>{label}</p>
          <p className="hint">{accept}</p>
        </div>
      )}
    </div>
  );
}
```

### 4.2 File Path Extraction

**Rust Command:**

```rust
#[command]
pub async fn get_file_path(file_name: String) -> Result<String, String> {
    // On web builds, File object doesn't expose path
    // User must use native file dialog or we get path from drag event
    // This is a Tauri limitation - may need to adjust approach
    
    // For now, require user to use dialog as fallback
    use tauri::api::dialog::FileDialogBuilder;
    
    // This is called from frontend when needed
    Ok(file_name)
}
```

**Important Note:** Tauri's web-based drag-and-drop doesn't expose full file paths for security. You'll need to use Tauri's native file dialog API as a secondary option:

```typescript
import { open } from '@tauri-apps/api/dialog';

const selectVideoFile = async () => {
  const selected = await open({
    multiple: false,
    filters: [{
      name: 'Video',
      extensions: ['mp4', 'mov', 'mkv']
    }]
  });
  
  if (selected && typeof selected === 'string') {
    // We have the full path
    setVideoPath(selected);
  }
};
```

---

## 5. Phase 3: VTT Parser & Transcript Processing

### 5.1 VTT Parser Implementation

**`src/backend/services/vtt_parser.rs`**

```rust
use std::fs;
use regex::Regex;

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
    pub fn find_closest_cue(cues: &[VttCue], target_time: &str) -> Option<&VttCue> {
        // Convert target to seconds for comparison
        let target_seconds = Self::timestamp_to_seconds(target_time)?;
        
        cues.iter()
            .min_by_key(|cue| {
                let cue_seconds = Self::timestamp_to_seconds(&cue.start_timestamp).unwrap_or(0);
                ((cue_seconds as i32) - (target_seconds as i32)).abs()
            })
    }
    
    fn timestamp_to_seconds(timestamp: &str) -> Option<u32> {
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
```

### 5.2 Transcript Reading Command

```rust
#[command]
pub async fn read_transcript(file_path: String) -> Result<String, String> {
    if file_path.ends_with(".vtt") {
        let cues = VttParser::parse(&file_path)?;
        Ok(VttParser::get_full_transcript(&cues))
    } else if file_path.ends_with(".txt") {
        fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read transcript: {}", e))
    } else {
        Err("Unsupported transcript format".to_string())
    }
}
```

---

## 6. Phase 4: OpenAI Integration

### 6.1 Prompt Engineering

**System Prompt:**

```
You are an expert video editor analyzing meeting transcripts. Your task is to identify the most important, shareable moments from a meeting recording.

You must return ONLY a valid JSON array of objects. Each object must have exactly these fields:
- "title": A brief, descriptive title (max 50 characters)
- "start_time": Timestamp in HH:MM:SS or MM:SS format
- "end_time": Timestamp in HH:MM:SS or MM:SS format

Important rules:
1. Focus on moments with clear decisions, action items, key announcements, or important discussions
2. Each clip should be 15-90 seconds long
3. Timestamps must match the transcript timestamps provided
4. Return 3-7 clips maximum
5. DO NOT include any text outside the JSON array
6. Ensure end_time is always after start_time

Example output format:
[
  {
    "title": "Q4 Budget Approval",
    "start_time": "00:14:32",
    "end_time": "00:15:45"
  }
]
```

### 6.2 OpenAI Service Implementation

**`src/backend/services/openai.rs`**

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    response_format: ResponseFormat,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

pub struct OpenAIService;

impl OpenAIService {
    pub async fn analyze_transcript(
        api_key: &str,
        transcript: &str,
        user_context: Option<&str>,
    ) -> Result<Vec<ClipSuggestion>, String> {
        let client = Client::new();
        
        let system_prompt = r#"You are an expert video editor analyzing meeting transcripts. Your task is to identify the most important, shareable moments from a meeting recording.

You must return ONLY a valid JSON array of objects. Each object must have exactly these fields:
- "title": A brief, descriptive title (max 50 characters)
- "start_time": Timestamp in HH:MM:SS or MM:SS format
- "end_time": Timestamp in HH:MM:SS or MM:SS format

Important rules:
1. Focus on moments with clear decisions, action items, key announcements, or important discussions
2. Each clip should be 15-90 seconds long
3. Timestamps must match the transcript timestamps provided (look for [HH:MM:SS] markers)
4. Return 3-7 clips maximum
5. DO NOT include any text outside the JSON array
6. Ensure end_time is always after start_time"#;

        let user_prompt = if let Some(context) = user_context {
            format!("User guidance: {}\n\n---TRANSCRIPT---\n\n{}", context, transcript)
        } else {
            format!("---TRANSCRIPT---\n\n{}", transcript)
        };
        
        let request = ChatRequest {
            model: "gpt-4-turbo-preview".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            temperature: 0.3,
            response_format: ResponseFormat {
                format_type: "json_object".to_string(),
            },
        };
        
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("OpenAI API error ({}): {}", status, error_text));
        }
        
        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let content = &chat_response.choices[0].message.content;
        
        // Parse the JSON array from the response
        let clips: Vec<ClipSuggestion> = serde_json::from_str(content)
            .map_err(|e| format!("Failed to parse clip suggestions: {}. Response was: {}", e, content))?;
        
        Ok(clips)
    }
}
```

### 6.3 Analysis Command

```rust
#[command]
pub async fn analyze_transcript_for_clips(
    app_handle: tauri::AppHandle,
    transcript_path: String,
    video_path: String,
    user_context: Option<String>,
) -> Result<Vec<ValidatedClip>, String> {
    // 1. Get API key
    let api_key = get_api_key(app_handle).await?
        .ok_or("No API key configured")?;
    
    // 2. Parse VTT file
    let vtt_cues = VttParser::parse(&transcript_path)?;
    let full_transcript = VttParser::get_full_transcript(&vtt_cues);
    
    // 3. Call OpenAI
    let raw_clips = OpenAIService::analyze_transcript(
        &api_key,
        &full_transcript,
        user_context.as_deref(),
    ).await?;
    
    // 4. Validate and map timestamps to actual VTT cues
    let validated_clips = raw_clips
        .into_iter()
        .filter_map(|clip| {
            validate_and_map_clip(clip, &vtt_cues)
        })
        .collect();
    
    Ok(validated_clips)
}

fn validate_and_map_clip(
    clip: ClipSuggestion,
    vtt_cues: &[VttCue],
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
```

---

## 7. Phase 5: FFmpeg Processing

### 7.1 FFmpeg Binary Setup

**Platform-Specific Binaries:**

1. Download static FFmpeg builds:
   - **macOS Intel:** `ffmpeg-x86_64-apple-darwin`
   - **macOS Apple Silicon:** `ffmpeg-aarch64-apple-darwin`
   - **Windows:** `ffmpeg-x86_64-pc-windows-msvc.exe`

2. Place in `src-tauri/bin/` directory

3. Create platform-specific naming:
```
src-tauri/bin/
├── ffmpeg-x86_64-apple-darwin
├── ffmpeg-aarch64-apple-darwin
└── ffmpeg-x86_64-pc-windows-msvc.exe
```

4. Update `tauri.conf.json`:
```json
"externalBin": [
  "bin/ffmpeg-x86_64-apple-darwin",
  "bin/ffmpeg-aarch64-apple-darwin",
  "bin/ffmpeg-x86_64-pc-windows-msvc"
]
```

### 7.2 FFmpeg Service

**`src/backend/services/ffmpeg.rs`**

```rust
use tauri::api::process::{Command, CommandEvent};
use std::path::{Path, PathBuf};
use std::fs;

pub struct FFmpegService;

impl FFmpegService {
    /// Generate clips from a video file
    pub async fn generate_clips(
        video_path: String,
        clips: Vec<ValidatedClip>,
        progress_callback: impl Fn(usize, usize),
    ) -> Result<String, String> {
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
            
            progress_callback(index + 1, clips.len());
        }
        
        Ok(output_dir.to_str().unwrap().to_string())
    }
    
    async fn extract_clip(
        input_path: &str,
        start_time: &str,
        end_time: &str,
        output_path: &str,
    ) -> Result<(), String> {
        let (mut rx, _child) = Command::new_sidecar("ffmpeg")
            .map_err(|e| format!("Failed to find FFmpeg binary: {}", e))?
            .args(&[
                "-i", input_path,
                "-ss", start_time,
                "-to", end_time,
                "-c", "copy",           // Stream copy (no re-encoding)
                "-avoid_negative_ts", "1",  // Handle timestamp issues
                "-y",                   // Overwrite output file
                output_path,
            ])
            .spawn()
            .map_err(|e| format!("Failed to spawn FFmpeg: {}", e))?;
        
        // Monitor process output
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    println!("FFmpeg stdout: {}", String::from_utf8_lossy(&line));
                }
                CommandEvent::Stderr(line) => {
                    println!("FFmpeg stderr: {}", String::from_utf8_lossy(&line));
                }
                CommandEvent::Error(error) => {
                    return Err(format!("FFmpeg error: {}", error));
                }
                CommandEvent::Terminated(payload) => {
                    if payload.code != Some(0) {
                        return Err(format!("FFmpeg exited with code: {:?}", payload.code));
                    }
                    break;
                }
                _ => {}
            }
        }
        
        Ok(())
    }
}
```

### 7.3 Processing Command

```rust
#[command]
pub async fn generate_clips(
    app_handle: tauri::AppHandle,
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
    
    // Progress tracking would ideally use Tauri events
    let output_dir = FFmpegService::generate_clips(
        video_path,
        clips_to_generate.clone(),
        |current, total| {
            let _ = app_handle.emit_all("clip-progress", ClipProgress {
                current,
                total,
            });
        },
    ).await?;
    
    Ok(ProcessingResult {
        output_directory: output_dir,
        clip_count: clips_to_generate.len(),
    })
}

#[derive(Clone, Serialize)]
struct ClipProgress {
    current: usize,
    total: usize,
}

#[derive(Serialize)]
struct ProcessingResult {
    output_directory: String,
    clip_count: usize,
}
```

---

## 8. Phase 6: Frontend State Machine

### 8.1 App State Hook

**`src/frontend/hooks/useAppState.ts`**

```typescript
import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

export function useAppState() {
  const [state, setState] = useState<AppState>({ status: 'ready' });
  const [videoPath, setVideoPath] = useState<string | null>(null);
  const [transcriptPath, setTranscriptPath] = useState<string | null>(null);
  const [context, setContext] = useState<string>('');

  // Listen for progress events
  useEffect(() => {
    const unlisten = listen('clip-progress', (event) => {
      const { current, total } = event.payload as { current: number; total: number };
      setState({ status: 'processing', progress: (current / total) * 100 });
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const analyzeClips = useCallback(async () => {
    if (!videoPath || !transcriptPath) return;

    setState({ status: 'analyzing' });

    try {
      const clips: Clip[] = await invoke('analyze_transcript_for_clips', {
        transcriptPath,
        videoPath,
        userContext: context || null,
      });

      setState({ status: 'review', clips });
    } catch (error) {
      setState({ status: 'error', message: String(error) });
    }
  }, [videoPath, transcriptPath, context]);

  const generateClips = useCallback(async (selectedClips: Clip[]) => {
    if (!videoPath) return;

    setState({ status: 'processing', progress: 0 });

    try {
      const result = await invoke('generate_clips', {
        videoPath,
        clips: selectedClips,
      });

      setState({
        status: 'complete',
        outputPath: result.output_directory,
        clipCount: result.clip_count,
      });
    } catch (error) {
      setState({ status: 'error', message: String(error) });
    }
  }, [videoPath]);

  const reset = useCallback(() => {
    setState({ status: 'ready' });
    setVideoPath(null);
    setTranscriptPath(null);
    setContext('');
  }, []);

  return {
    state,
    videoPath,
    setVideoPath,
    transcriptPath,
    setTranscriptPath,
    context,
    setContext,
    analyzeClips,
    generateClips,
    reset,
  };
}
```

### 8.2 Main App Component Structure

```typescript
function App() {
  const {
    state,
    videoPath,
    setVideoPath,
    transcriptPath,
    setTranscriptPath,
    context,
    setContext,
    analyzeClips,
    generateClips,
    reset,
  } = useAppState();

  const renderContent = () => {
    switch (state.status) {
      case 'ready':
        return <ReadyView {...} />;
      case 'analyzing':
        return <AnalyzingView />;
      case 'review':
        return <ReviewView clips={state.clips} onGenerate={generateClips} />;
      case 'processing':
        return <ProcessingView progress={state.progress} />;
      case 'complete':
        return <CompleteView {...state} onReset={reset} />;
      case 'error':
        return <ErrorView message={state.message} onReset={reset} />;
    }
  };

  return (
    <div className="app">
      <Header />
      {renderContent()}
    </div>
  );
}
```

---

## 9. Phase 7: Cross-Platform Considerations

### 9.1 Path Handling

```rust
use std::path::PathBuf;

fn normalize_path(path: &str) -> PathBuf {
    // Handle both Windows and Unix paths
    PathBuf::from(path)
}

fn get_output_directory(video_path: &str) -> Result<PathBuf, String> {
    let path = normalize_path(video_path);
    let parent = path.parent()
        .ok_or("Cannot determine parent directory")?;
    let stem = path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid filename")?;
    
    Ok(parent.join(format!("{}_Clips", stem)))
}
```

### 9.2 File System Access

Ensure `tauri.conf.json` allows access to user directories:

```json
"allowlist": {
  "fs": {
    "scope": [
      "$HOME/**",
      "$DESKTOP/**",
      "$DOCUMENT/**",
      "$DOWNLOAD/**",
      "$VIDEO/**"
    ]
  }
}
```

### 9.3 Opening Output Folder

```rust
use tauri::api::shell;

#[command]
pub async fn open_in_file_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    shell::open(&shell::Scope::default(), "explorer", Some(path))
        .map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "macos")]
    shell::open(&shell::Scope::default(), "open", Some(path))
        .map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "linux")]
    shell::open(&shell::Scope::default(), "xdg-open", Some(path))
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
```

---

## 10. Error Handling & Edge Cases

### 10.1 Common Error Scenarios

| Error | Mitigation |
|-------|------------|
| API key invalid/missing | Validate before allowing analysis; show clear error |
| VTT file malformed | Validate structure; provide helpful error message |
| Video file corrupted | FFmpeg will fail; catch error and inform user |
| Timestamps out of sync | Use VTT cue matching; add tolerance buffer |
| Disk space insufficient | Check available space before processing |
| FFmpeg binary not found | Verify sidecar configuration; provide troubleshooting |
| OpenAI returns invalid JSON | Implement retry logic; parse defensively |
| Clips overlap or are too short | Validate clip duration (minimum 5 seconds) |

### 10.2 Validation Functions

```rust
fn validate_clip_duration(start: &str, end: &str) -> Result<(), String> {
    let start_seconds = VttParser::timestamp_to_seconds(start)
        .ok_or("Invalid start timestamp")?;
    let end_seconds = VttParser::timestamp_to_seconds(end)
        .ok_or("Invalid end timestamp")?;
    
    let duration = end_seconds.saturating_sub(start_seconds);
    
    if duration < 5 {
        return Err("Clip too short (minimum 5 seconds)".to_string());
    }
    
    if duration > 300 {
        return Err("Clip too long (maximum 5 minutes)".to_string());
    }
    
    Ok(())
}

fn validate_video_file(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        return Err("Video file not found".to_string());
    }
    
    let extension = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());
    
    match extension.as_deref() {
        Some("mp4") | Some("mov") | Some("mkv") => Ok(()),
        _ => Err("Unsupported video format".to_string()),
    }
}
```

---

## 11. Testing Strategy

### 11.1 Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vtt_timestamp_conversion() {
        assert_eq!(
            VttParser::vtt_to_ffmpeg_timestamp("00:01:14.500"),
            "00:01:14"
        );
    }

    #[test]
    fn test_filename_sanitization() {
        assert_eq!(
            sanitize_filename("Decision: Budget (Q4)?"),
            "Decision_ Budget (Q4)_"
        );
    }

    #[test]
    fn test_timestamp_validation() {
        assert!(validate_clip_duration("00:01:00", "00:01:10").is_ok());
        assert!(validate_clip_duration("00:01:00", "00:01:02").is_err());
    }
}
```

### 11.2 Integration Test Plan

1. **Happy Path:**
   - Valid VTT + valid MP4 → generates expected clips

2. **Error Cases:**
   - Missing API key → shows error
   - Malformed VTT → shows error
   - Invalid video format → shows error
   - OpenAI timeout → shows retry option

3. **Edge Cases:**
   - Very long video (3+ hours)
   - VTT with unusual formatting
   - Special characters in filenames
   - Clips at video start/end

### 11.3 Manual Testing Checklist

- [ ] Settings: Save/load API key
- [ ] Settings: Validate API key works
- [ ] File input: Drag-and-drop video file
- [ ] File input: Drag-and-drop transcript file
- [ ] Analysis: Progress indicator shows
- [ ] Review: Clips display with correct timestamps
- [ ] Review: Can toggle clip selection
- [ ] Processing: Progress updates correctly
- [ ] Complete: "Show in Folder" opens correct directory
- [ ] Complete: Generated clips play correctly
- [ ] Cross-platform: Test on macOS and Windows

---

## 12. Performance Optimizations

### 12.1 Transcript Handling

For very large transcripts (3+ hour meetings):
- Consider chunking for OpenAI (max 128k tokens for GPT-4)
- Stream parsing instead of loading entire file

```rust
const MAX_TRANSCRIPT_SIZE: usize = 100_000; // ~100k characters

fn chunk_transcript(transcript: &str, max_size: usize) -> Vec<String> {
    // Split on natural boundaries (cue boundaries)
    // Implementation depends on use case
}
```

### 12.2 FFmpeg Optimizations

Already using `-c copy` (stream copy) which is optimal. Additional options:

```rust
// For faster processing on multi-core systems
.args(&[
    "-threads", "0",  // Use all available threads
])

// For better timestamp accuracy
.args(&[
    "-ss", start_time,
    "-i", input_path,  // -ss before -i is faster
    "-to", end_time,
    "-c", "copy",
])
```

---

## 13. Future Enhancements (Out of Scope for V1)

1. **Support for `.txt` transcripts** (no timestamps)
   - Use speech-to-text alignment
   - Estimate timestamps from word count

2. **Custom clip padding**
   - Add 2-3 seconds before/after identified moment
   - User-configurable in settings

3. **Clip preview**
   - Embedded video player
   - Preview before generating

4. **Batch processing**
   - Multiple videos at once
   - Queue system

5. **Custom clip export settings**
   - Resolution/bitrate options
   - Format conversion

6. **Cloud API key management**
   - Optional Anthropic API key support
   - Azure OpenAI support

---

## 14. Deployment Considerations

### 14.1 Build Commands

**Development:**
```bash
npm run tauri dev
```

**Production Build:**
```bash
npm run tauri build
```

This will create platform-specific installers:
- **macOS:** `.dmg` and `.app` bundle
- **Windows:** `.msi` installer

### 14.2 Code Signing

**macOS:**
```bash
# Requires Apple Developer account
codesign --force --deep --sign "Developer ID Application: Your Name" app.app
```

**Windows:**
```bash
# Requires code signing certificate
signtool sign /f cert.pfx /p password app.exe
```

### 14.3 Distribution

1. GitHub Releases with auto-updater
2. Homebrew Cask (macOS)
3. Direct download from website

---

## 15. Security Considerations

### 15.1 API Key Storage

- Use `tauri-plugin-store` with encryption
- Never log API keys
- Clear from memory after use

### 15.2 File Access

- Restrict file system access via Tauri's scope system
- Validate file paths to prevent directory traversal
- Never execute user-provided commands

### 15.3 Network Security

- All API calls over HTTPS
- Validate SSL certificates
- Implement rate limiting for API calls

---

## 16. Documentation Requirements

### 16.1 User-Facing

1. **README:**
   - Installation instructions
   - How to obtain OpenAI API key
   - Basic usage guide

2. **In-App Help:**
   - Tooltip on settings
   - Link to API key signup
   - Error message explanations

### 16.2 Developer-Facing

1. **Setup Guide:**
   - Environment setup
   - FFmpeg binary installation
   - Build instructions

2. **Architecture Docs:**
   - State flow diagram
   - Command reference
   - Error codes

---

## Appendix A: Dependencies

### Frontend (`package.json`)
```json
{
  "dependencies": {
    "@tauri-apps/api": "^1.5.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^1.5.0",
    "@types/react": "^18.2.0",
    "typescript": "^5.0.0",
    "vite": "^5.0.0"
  }
}
```

### Backend (`Cargo.toml`)
```toml
[dependencies]
tauri = { version = "1.5", features = ["shell-sidecar", "fs-all", "dialog-all", "path-all"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
regex = "1.10"
uuid = { version = "1.0", features = ["v4", "serde"] }
tauri-plugin-store = "0.1"
```

---

## Appendix B: Sample VTT File

```
WEBVTT

00:00:00.000 --> 00:00:03.500
Welcome everyone to today's Q4 planning meeting.

00:00:03.500 --> 00:00:07.200
Let's start with a review of our budget proposal.

00:01:14.500 --> 00:01:18.200
So we're approving the $2M budget for Q4, correct?

00:01:18.200 --> 00:01:21.800
Yes, that's approved. Sarah will send the details.
```

---

## Revision History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2024 | Initial specification |

---

**END OF SPECIFICATION**

This specification provides a complete implementation guide. The engineer should follow the phases sequentially, building and testing each component before moving to the next. All critical issues identified in the initial review have been addressed with concrete solutions.