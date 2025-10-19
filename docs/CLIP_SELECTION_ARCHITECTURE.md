# Clip Selection Architecture

This document details how ClipScribe currently selects and generates video clips.

## 1. High-Level System Flow

```mermaid
graph TB
    A[User] -->|1. Upload Video| B[Frontend]
    A -->|2. Upload VTT or Generate| B
    B -->|3. Click Analyze| C[Tauri Backend]
    C -->|4. Parse VTT| D[VTT Parser]
    D -->|5. Format VTT| E[OpenAI Service]
    E -->|6. API Call| F[GPT-5-mini]
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
    participant GPT as GPT-5-mini
    participant Val as Validator
    
    U->>FE: Click "Analyze & Find Clips"
    FE->>BE: analyze_transcript_for_clips()
    Note over FE,BE: videoPath, transcriptPath, userContext
    
    BE->>BE: Get API key from settings
    BE->>VTT: parse(transcript_path)
    VTT->>VTT: Read VTT file
    VTT->>VTT: Parse cues with timestamps
    VTT-->>BE: Vec<VttCue>
    
    BE->>VTT: get_formatted_vtt(cues)
    VTT->>VTT: Format with full structure
    VTT-->>BE: Formatted VTT with timestamps
    
    BE->>AI: analyze_transcript(api_key, vtt, context)
    AI->>AI: Build prompt with USER INSTRUCTIONS FIRST
    Note over AI: Model: gpt-5-mini<br/>Reasoning: minimal<br/>Verbosity: low
    
    AI->>GPT: POST /v1/responses
    Note over AI,GPT: Prompt structure:<br/>- PRIMARY DIRECTIVE (user context)<br/>- Task description<br/>- VTT transcript with timestamps
    
    GPT->>GPT: Analyze VTT (minimal reasoning)
    GPT->>GPT: Identify exciting/interesting moments
    GPT->>GPT: Generate JSON response (low verbosity)
    GPT-->>AI: Nested response structure
    
    AI->>AI: Extract text from output[].content[].text
    AI->>AI: Parse JSON clips array
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
    B --> C[gpt-5-mini-2025-08-07]
    
    A --> D[Parameters]
    D --> E[reasoning.effort: minimal]
    D --> F[text.verbosity: low]
    D --> G[API: /v1/responses]
    
    A --> H[Prompt Engineering]
    H --> I[User Instructions]
    H --> J[Default Guidance]
    
    I --> K[PRIMARY DIRECTIVE:<br/>User's specific request]
    I --> L[Always prioritized<br/>over defaults]
    
    J --> M[Excitement Focus:<br/>Surprises, drama, insights]
    J --> N[Task Description:<br/>Find compelling clips]
    J --> O[Output Format:<br/>JSON with timestamps]
    
    style K fill:#FF5722
    style L fill:#FF5722
    
    style C fill:#4CAF50
    style E fill:#FF9800
    style F fill:#FF9800
    style G fill:#FF9800
```

## 4. Data Flow & Transformations

```mermaid
graph TD
    A[Raw VTT File] -->|Parse| B[VttCue Array]
    B -->|Format with Structure| C[Formatted VTT]
    
    C -->|+ User Instructions| D[GPT-5 Prompt]
    D -->|API Call| E[GPT-5 Response]
    E -->|Extract from nested| F[output[].content[].text]
    F -->|Parse JSON| G[ClipSuggestion Array]
    
    G -->|Validate| H{Timestamp Valid?}
    H -->|Yes| I[Add to ValidatedClip]
    H -->|No| J[Skip Clip]
    
    I --> K[ValidatedClip Array]
    K --> L[Frontend Display]
    L --> M{User Selects?}
    M -->|Yes| N[Generate Queue]
    M -->|No| O[Excluded]
    
    N --> P[FFmpeg Processing]
    P --> Q[Output MP4 Files]
    
    style E fill:#4CAF50
    style K fill:#2196F3
    style Q fill:#9C27B0
```

## 5. Current Prompt Structure (GPT-5)

```mermaid
graph TB
    A[Single Input Prompt] --> B{User Instructions?}
    
    B -->|Yes| C[PRIMARY DIRECTIVE]
    C --> C1["User instructions FIRST"]
    C --> C2["Always prioritized"]
    C --> C3["Conflict resolution:<br/>User wins"]
    
    B -->|No| D[Default Mode]
    D --> D1["Excitement focus"]
    D --> D2["Surprises, drama, insights"]
    
    C --> E[Task Description]
    D --> E
    
    E --> F[VTT Analysis]
    F --> F1["Full VTT with timestamps"]
    F --> F2["Cue numbers preserved"]
    F --> F3["Segment boundaries visible"]
    
    E --> G[Output Requirements]
    G --> G1["JSON array only"]
    G --> G2["10-120 second clips"]
    G --> G3["3-8 clips maximum"]
    
    style C fill:#FF5722
    style C1 fill:#FF5722
    style C2 fill:#FF5722
    style D fill:#4CAF50
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

## Current Status & Remaining Limitations

### ✅ Resolved (v1.2.0)
1. **Model Selection** - Now uses GPT-5-mini with minimal reasoning for cost optimization
2. **User Priority** - User instructions now ALWAYS prioritize over default prompts
3. **VTT Awareness** - AI analyzes full VTT structure with timestamps
4. **Excitement Focus** - New default prompt finds compelling moments
5. **Clip Constraints** - Now enforces 10-120 second length, 3-8 clips

### ⚠️ Still Limited
1. **No fallback model** - If GPT-5-mini unavailable, hard fails
2. **Simple validation** - Just finds nearest VTT cue, no quality scoring
3. **No overlap detection** - Multiple clips could overlap
4. **Auto-select all** - Everything selected by default
5. **No clip editing** - Can't adjust timestamps in UI
6. **No re-analysis** - Must start over to change context

## Next Improvements (Priority Order)

### High Priority
1. **Clip quality scoring** - Rank clips by importance/interest level
2. **Overlap detection** - Warn or auto-resolve overlapping clips
3. **Better defaults** - Don't auto-select all clips
4. **Dynamic reasoning** - Adjust effort based on video complexity

### Medium Priority
5. **Few-shot examples** - Include example outputs in prompt
6. **Video type detection** - Adjust prompt for meetings, tutorials, vlogs
7. **Metadata injection** - Pass video duration, topics to AI
8. **Fallback model** - Use gpt-4o if gpt-5-mini unavailable

### Low Priority
9. **Clip editing UI** - Adjust timestamps before generation
10. **Re-analysis** - Change context without re-uploading
11. **Caching** - Save analyses for repeated use
12. **A/B testing** - Compare different prompt strategies

---

**Next Steps**: Review these diagrams and identify which areas to improve first.
