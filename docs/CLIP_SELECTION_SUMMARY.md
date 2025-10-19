# Clip Selection - Current Implementation Summary (v1.2.0)

## Key Details

### Model Configuration ✅ Updated
- **Model**: `gpt-5-mini-2025-08-07`
- **API**: Responses API (`/v1/responses`)
- **Reasoning Effort**: `minimal` (fast, cost-optimized)
- **Verbosity**: `low` (concise JSON output)
- **Response Format**: Nested structure → `output[].content[].text`

### Current Prompt Strategy ✅ Updated

**PRIMARY DIRECTIVE (when user provides instructions):**
```
FOLLOW THESE USER INSTRUCTIONS EXACTLY: "[user instructions]"

If the user's instructions conflict with any guidance below, 
ALWAYS prioritize the user's instructions above all else.
```

**Default Focus (no user instructions):**
- Exciting and interesting moments
- Surprises, drama, insights
- High-energy content
- Memorable exchanges

**Key Constraints:**
1. Clip length: 10-120 seconds (expanded from 15-90)
2. Return: 3-8 clips maximum (expanded from 3-7)
3. Output: JSON array only
4. Input: Full VTT with timestamps (not plain text)

**Input Format:**
```
WEBVTT - Video Transcript

1
00:00:15.000 --> 00:00:18.500
[transcript text]

[continues with full VTT structure]
```

### Status Update

#### ✅ Resolved in v1.2.0
1. **Model Selection** - Now uses GPT-5-mini with minimal reasoning
2. **Cost Optimization** - Minimal reasoning effort reduces API costs
3. **User Priority** - PRIMARY DIRECTIVE system ensures user control
4. **VTT Awareness** - AI analyzes full transcript structure with timestamps
5. **Excitement Focus** - Default prompt finds compelling moments
6. **Better Constraints** - 10-120s clips, 3-8 maximum

#### ⚠️ Remaining Limitations
1. **No fallback model** - Hard fails if GPT-5-mini unavailable
2. **Generic for video types** - Same prompt for meetings, tutorials, vlogs
3. **No quality scoring** - All clips treated equally
4. **No overlap detection** - Multiple clips could overlap
5. **Auto-select all** - Everything selected by default
6. **Simple validation** - Just matches nearest VTT timestamp
7. **No clip editing** - Can't adjust timestamps in UI

## Improvement Priorities

### Phase 1: Foundation (Low Complexity, High Impact)
1. **Add clip length validation** - Reject clips < 10s or > 120s
2. **Overlap detection** - Warn if clips overlap
3. **Quality scoring** - Rank clips by importance
4. **Better defaults** - Don't auto-select all clips

**Scope**: Small changes to validation logic  
**Risk**: Low - minimal existing code changes  
**Dependencies**: None

### Phase 2: Prompt Enhancement (Medium Complexity, High Impact)
1. **Video type detection** - Adjust prompt based on context
2. **Few-shot examples** - Include 2-3 example outputs in prompt
3. **Dynamic constraints** - Adjust rules based on video length
4. **Metadata injection** - Pass video duration, topics to prompt

**Scope**: Moderate refactor of prompt engineering  
**Risk**: Medium - prompt changes could affect quality  
**Dependencies**: Phase 1 validation improvements

### Phase 3: Model Intelligence (Medium Complexity, Medium Impact)
1. **Dynamic reasoning** - Adjust effort based on video complexity  
2. **Fallback strategy** - Use gpt-4o if gpt-5-mini unavailable
3. **Token optimization** - Truncate long transcripts intelligently
4. **Caching** - Save analyses to avoid re-processing

**Scope**: Medium refactor of API service layer  
**Risk**: Medium - API changes, cost implications  
**Dependencies**: Configuration system for model settings

### Phase 4: Advanced Features (High Complexity, Variable Impact)
1. **Multi-pass analysis** - First pass: identify segments, second: refine
2. **Speaker detection** - Use speaker changes for clip boundaries
3. **Topic modeling** - Cluster similar content
4. **User feedback loop** - Learn from selections/rejections

**Scope**: Large feature additions  
**Risk**: High - significant architectural changes  
**Dependencies**: All previous phases, possibly ML libraries

## Code Locations

| Component | File | Lines |
|-----------|------|-------|
| Main analysis | `src-tauri/src/commands/analyze.rs` | 1-81 |
| OpenAI service | `src-tauri/src/services/openai.rs` | 1-117 |
| System prompt | `src-tauri/src/services/openai.rs` | 43-65 |
| Validation | `src-tauri/src/commands/analyze.rs` | 39-68 |
| VTT parsing | `src-tauri/src/services/vtt_parser.rs` | Full file |

## Example Current Prompt

For a 30-minute meeting with user context "Focus on budget discussions":

```
SYSTEM:
You are an expert video editor analyzing meeting transcripts...
[full system prompt from openai.rs:43-65]

USER:
User guidance: Focus on budget discussions

---TRANSCRIPT---

00:00:00.000 --> 00:00:05.000
Welcome everyone to today's budget review meeting...

[continues for full transcript]
```

**GPT-4 Response:**
```json
[
  {
    "title": "Q4 Budget Allocation",
    "start_time": "00:03:15",
    "end_time": "00:04:30"
  },
  {
    "title": "Marketing Budget Concerns",
    "start_time": "00:12:45",
    "end_time": "00:14:20"
  }
]
```

## Metrics to Track

When improving, we should measure:

1. **Accuracy** - % of clips user keeps vs discards
2. **Coverage** - % of important moments captured
3. **Efficiency** - API cost per video
4. **Quality** - User satisfaction rating
5. **Speed** - Time to generate suggestions

---

**View full diagrams:** [CLIP_SELECTION_ARCHITECTURE.md](./CLIP_SELECTION_ARCHITECTURE.md)

## Next: Which improvement should we tackle first?

**Recommendations by Priority:**
- **Lowest complexity, immediate value**: Clip length validation + overlap detection
- **Highest impact on quality**: Enhanced prompt with video type detection  
- **Best foundation for future work**: Quality scoring + better defaults
