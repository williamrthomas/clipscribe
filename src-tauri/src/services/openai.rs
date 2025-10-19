use crate::models::ClipSuggestion;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
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
6. Ensure end_time is always after start_time

Example output format:
[
  {
    "title": "Q4 Budget Approval",
    "start_time": "00:14:32",
    "end_time": "00:15:45"
  }
]"#;

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
        
        // Try to parse as JSON array directly
        let clips: Vec<ClipSuggestion> = serde_json::from_str(content)
            .map_err(|e| format!("Failed to parse clip suggestions: {}. Response was: {}", e, content))?;
        
        Ok(clips)
    }
}
