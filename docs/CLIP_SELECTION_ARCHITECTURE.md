# Clip Selection Architecture

This document details how ClipScribe currently selects and generates video clips.

## 1. High-Level System Flow

```mermaid
graph TB
    A[User] -->|1. Upload Video| B[Frontend]
    A -->|2. Upload VTT or Generate| B
    B -->|3. Click Analyze| C[Tauri Backend]
    C -->|4. Parse VTT| D[VTT Parser]
    D -->|5. Extract Text| E[OpenAI Service]
    E -->|6. API Call| F[GPT-4 Turbo]
    F -->|7. JSON Response| E
    E -->|8. Parse Suggestions| C
    C -->|9. Validate Timestamps| G[Timestamp Validator]
    G -->|10. Return Valid Clips| B
    B -->|11. Display Review UI| A
    A -->|12. Select Clips| B
    B -->|13. Generate Request| H[FFmpeg Service]
    H -->|14. Extract Clips| I[Output Files]
    
    style F fill:#4CAF50
    style E fill:#2196F3
    style D fill:#FF9800
    style H fill:#9C27B0
```

## 2. Detailed Analysis Flow

```mermaid
sequenceDiagram
    participant U as User
    participant FE as Frontend<br/>(React)
    participant BE as Backend<br/>(Rust/Tauri)
    participant VTT as VTT Parser
    participant AI as OpenAI Service
    participant GPT as GPT-4 Turbo
    participant Val as Validator
    
    U->>FE: Click "Analyze & Find Clips"
    FE->>BE: analyze_transcript_for_clips()
    Note over FE,BE: videoPath, transcriptPath, userContext
    
    BE->>BE: Get API key from settings
    BE->>VTT: parse(transcript_path)
    VTT->>VTT: Read VTT file
    VTT->>VTT: Parse cues with timestamps
    VTT-->>BE: Vec<VttCue>
    
    BE->>VTT: get_full_transcript(cues)
    VTT->>VTT: Concatenate all text
    VTT-->>BE: String (full transcript)
    
    BE->>AI: analyze_transcript(api_key, text, context)
    AI->>AI: Build prompt with system + user messages
    Note over AI: Model: gpt-4-turbo-preview<br/>Temperature: 0.3<br/>Max tokens: 4000
    
    AI->>GPT: POST /v1/chat/completions
    Note over AI,GPT: Prompt includes:<br/>- Task description<br/>- Output format (JSON)<br/>- User context (if provided)<br/>- Full transcript
    
    GPT->>GPT: Analyze transcript
    GPT->>GPT: Identify key moments
    GPT->>GPT: Generate JSON response
    GPT-->>AI: JSON string with clips array
    
    AI->>AI: Parse JSON response
    AI-->>BE: Vec<ClipSuggestion>
    
    BE->>Val: validate_and_match(suggestions, vtt_cues)
    loop For each suggestion
        Val->>Val: Parse timestamp strings
        Val->>Val: Find matching VTT cues
        Val->>Val: Generate sanitized filename
        Val->>Val: Create ValidatedClip
    end
    Val-->>BE: Vec<ValidatedClip>
    
    BE-->>FE: Return clips
    FE->>U: Display clip review UI
```

## 3. AI Model Configuration

```mermaid
graph LR
    A[OpenAI Service] --> B[Model Selection]
    B --> C[gpt-4-turbo-preview]
    
    A --> D[Parameters]
    D --> E[temperature: 0.3]
    D --> F[max_tokens: 4000]
    D --> G[response_format: json_object]
    
    A --> H[Prompt Engineering]
    H --> I[System Message]
    H --> J[User Message]
    
    I --> K[Role Definition:<br/>Expert video editor]
    I --> L[Task Description:<br/>Find engaging clips]
    I --> M[Output Format:<br/>JSON schema]
    
    J --> N[User Context<br/>Optional]
    J --> O[Full Transcript<br/>Required]
    
    style C fill:#4CAF50
    style E fill:#FF9800
    style F fill:#FF9800
    style G fill:#FF9800
```

## 4. Data Flow & Transformations

```mermaid
graph TD
    A[Raw VTT File] -->|Parse| B[VttCue Array]
    B -->|Extract Text| C[Plain Text Transcript]
    
    C -->|+ User Context| D[GPT-4 Prompt]
    D -->|API Call| E[GPT-4 Response]
    E -->|Parse JSON| F[ClipSuggestion Array]
    
    F -->|Validate| G{Timestamp Valid?}
    G -->|Yes| H[Add to ValidatedClip]
    G -->|No| I[Skip Clip]
    
    H --> J[ValidatedClip Array]
    J --> K[Frontend Display]
    K --> L{User Selects?}
    L -->|Yes| M[Generate Queue]
    L -->|No| N[Excluded]
    
    M --> O[FFmpeg Processing]
    O --> P[Output MP4 Files]
    
    style E fill:#4CAF50
    style J fill:#2196F3
    style P fill:#9C27B0
```

## 5. Current Prompt Structure

```mermaid
graph TB
    A[Complete Prompt] --> B[System Message]
    A --> C[User Message]
    
    B --> D[Role Definition]
    D --> D1["You are an expert video editor..."]
    
    B --> E[Task Objectives]
    E --> E1[Identify engaging clips]
    E --> E2[Suggest timestamps]
    E --> E3[Create descriptive titles]
    
    B --> F[Output Requirements]
    F --> F1[JSON format]
    F --> F2[Array of clips]
    F --> F3[title, start_time, end_time]
    
    C --> G{User Context?}
    G -->|Yes| H["Context: 'user guidance'"]
    G -->|No| I[No additional context]
    
    C --> J[Transcript]
    J --> J1["Transcript:\n[full text]"]
    
    style A fill:#2196F3
    style B fill:#4CAF50
    style C fill:#FF9800
```

## 6. Timestamp Validation Process

```mermaid
graph TD
    A[ClipSuggestion] --> B{Parse Start Time}
    B -->|Success| C{Parse End Time}
    B -->|Fail| Z[Reject Clip]
    
    C -->|Success| D{Find VTT Cues<br/>in Range}
    C -->|Fail| Z
    
    D -->|Found| E[Match Closest<br/>Start Cue]
    D -->|None| Z
    
    E --> F[Match Closest<br/>End Cue]
    F --> G[Generate<br/>Sanitized Filename]
    G --> H[Create<br/>ValidatedClip]
    H --> I[isSelected: true<br/>by default]
    
    style H fill:#4CAF50
    style Z fill:#F44336
    style I fill:#2196F3
```

## Current Issues & Limitations

### 1. Model Selection
- **Fixed Model**: `gpt-4-turbo-preview` hardcoded
- **No fallback**: If model unavailable, fails
- **No cost optimization**: Always uses most expensive model

### 2. Prompt Engineering
- **Generic prompt**: Same for all video types
- **Limited context**: Only optional user input, no video metadata
- **No examples**: No few-shot learning examples
- **No constraints**: Doesn't specify clip length preferences

### 3. Validation Logic
- **Simple matching**: Just finds nearest VTT cue
- **No quality check**: Doesn't verify if timestamp makes sense
- **No duration limits**: Could suggest very short or very long clips
- **No overlap detection**: Multiple clips could overlap

### 4. User Control
- **All or nothing**: User can only select/deselect
- **No editing**: Can't adjust timestamps
- **No re-analysis**: Must start over to change context
- **Auto-select all**: Everything selected by default

## Improvement Opportunities

### High Priority
1. **Dynamic prompt engineering** based on video type/context
2. **Clip length constraints** (min/max duration)
3. **Quality scoring** for suggestions
4. **Better timestamp validation** with content awareness

### Medium Priority
5. **Model selection** based on video length/complexity
6. **Few-shot examples** in prompt
7. **Overlap detection** and resolution
8. **User feedback loop** for prompt refinement

### Low Priority
9. **Multiple model comparison** (A/B testing)
10. **Caching** for repeated analyses
11. **Incremental analysis** for long videos
12. **Custom prompt templates** by use case

---

**Next Steps**: Review these diagrams and identify which areas to improve first.
