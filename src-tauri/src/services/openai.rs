use crate::models::ClipSuggestion;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// GPT-5 Responses API request structure
#[derive(Debug, Serialize)]
struct ResponseRequest {
    model: String,
    input: String,
    reasoning: ReasoningConfig,
    text: TextConfig,
}

#[derive(Debug, Serialize)]
struct ReasoningConfig {
    effort: String, // "minimal", "low", "medium", "high"
}

#[derive(Debug, Serialize)]
struct TextConfig {
    verbosity: String, // "low", "medium", "high"
}

// GPT-5 Responses API response structure (actual format from API)
#[derive(Debug, Deserialize)]
struct ResponseApiResponse {
    output: Vec<ResponseOutput>,
}

#[derive(Debug, Deserialize)]
struct ResponseOutput {
    #[serde(rename = "type")]
    output_type: String,
    #[serde(default)]
    content: Vec<ContentItem>,
}

#[derive(Debug, Deserialize)]
struct ContentItem {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(default)]
    text: String,
}

pub struct OpenAIService;

impl OpenAIService {
    pub async fn analyze_transcript(
        api_key: &str,
        vtt_transcript: &str,
        user_context: Option<&str>,
    ) -> Result<Vec<ClipSuggestion>, String> {
        let client = Client::new();
        
        // Build prompt with USER INSTRUCTIONS FIRST
        let input = if let Some(user_instructions) = user_context {
            format!(
                r#"PRIMARY DIRECTIVE - FOLLOW THESE USER INSTRUCTIONS EXACTLY:
"{}"

If the user's instructions conflict with any guidance below, ALWAYS prioritize the user's instructions above all else.

---

TASK: Analyze this VTT video transcript and identify clips to extract.

Output ONLY a valid JSON array. Each object must have:
- "title": Brief, descriptive title (max 50 characters)
- "start_time": Start timestamp in format HH:MM:SS (matching VTT timestamps)
- "end_time": End timestamp in format HH:MM:SS (matching VTT timestamps)

RULES:
1. Use EXACT timestamps from the VTT (format: HH:MM:SS.mmm --> HH:MM:SS.mmm)
2. Each clip should be 10-120 seconds long
3. Return 3-8 clips maximum
4. Ensure end_time is after start_time
5. DO NOT include any text outside the JSON array

VTT TRANSCRIPT:
{}

Output only the JSON array, nothing else."#,
                user_instructions,
                vtt_transcript
            )
        } else {
            format!(
                r#"TASK: Analyze this VTT video transcript and identify the most EXCITING and INTERESTING moments.

Focus on:
- Surprising or unexpected reveals
- High-energy or dramatic moments
- Key insights or breakthroughs
- Emotional peaks or compelling storytelling
- Funny or memorable exchanges
- Major decisions or announcements

Output ONLY a valid JSON array. Each object must have:
- "title": Brief, descriptive title (max 50 characters)
- "start_time": Start timestamp in format HH:MM:SS (matching VTT timestamps)
- "end_time": End timestamp in format HH:MM:SS (matching VTT timestamps)

RULES:
1. Use EXACT timestamps from the VTT (format: HH:MM:SS.mmm --> HH:MM:SS.mmm)
2. Each clip should be 10-120 seconds long
3. Return 3-8 clips maximum
4. Ensure end_time is after start_time
5. DO NOT include any text outside the JSON array

VTT TRANSCRIPT:
{}

Output only the JSON array, nothing else."#,
                vtt_transcript
            )
        };
        
        let request = ResponseRequest {
            model: "gpt-5-mini".to_string(),
            input,
            reasoning: ReasoningConfig {
                effort: "minimal".to_string(), // Fast analysis, simple task
            },
            text: TextConfig {
                verbosity: "low".to_string(), // Concise JSON output only
            },
        };
        
        println!("=== OpenAI API Request ===");
        println!("Model: gpt-5-mini");
        println!("Reasoning effort: minimal");
        println!("Verbosity: low");
        
        let response = client
            .post("https://api.openai.com/v1/responses")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;
        
        // Get raw response text for debugging
        let response_text = response.text().await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        println!("=== RAW API RESPONSE ===");
        println!("{}", response_text);
        println!("=== END RAW RESPONSE ===");
        
        // Check for actual errors (error field with non-null value)
        if response_text.contains("\"error\": {") || response_text.contains("\"error\":{") {
            return Err(format!("OpenAI API error: {}", response_text));
        }
        
        // Try to parse the response
        let api_response: ResponseApiResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse response: {}. Response was: {}", e, response_text))?;
        
        // Extract text from nested structure: output[?].content[?].text where type="message" and content_type="output_text"
        let content = api_response.output.iter()
            .find(|o| o.output_type == "message")
            .and_then(|msg| msg.content.iter().find(|c| c.content_type == "output_text"))
            .map(|c| &c.text)
            .ok_or("No output_text found in response")?;
        
        println!("=== GPT-5 Response ===");
        println!("{}", content);
        
        // Try to parse as JSON array directly
        let clips: Vec<ClipSuggestion> = serde_json::from_str(content)
            .map_err(|e| format!("Failed to parse clip suggestions: {}. Response was: {}", e, content))?;
        
        Ok(clips)
    }
}
