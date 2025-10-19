# Clip Selection - Current Implementation Summary

## Key Details

### Model Configuration
- **Model**: `gpt-4-turbo-preview` (hardcoded)
- **Temperature**: `0.3` (low randomness for consistency)
- **Max Tokens**: Not explicitly set (defaults to model max)
- **Response Format**: JSON object array

### Current Prompt Strategy

**System Message:**
```
You are an expert video editor analyzing meeting transcripts.
```

**Key Instructions:**
1. Focus on: decisions, action items, announcements, discussions
2. Clip length: 15-90 seconds
3. Return: 3-7 clips maximum
4. Output: JSON array only

**User Message:**
```
User guidance: [optional context]

---TRANSCRIPT---

[full transcript text]
```

### Limitations Identified

#### 1. **Fixed Model** 🔴
- No model selection logic
- No fallback if model unavailable
- No cost optimization (always uses most expensive)

#### 2. **Generic Prompt** 🟡
- Same prompt for all video types (meeting, presentation, tutorial, etc.)
- No video metadata context (duration, resolution, source)
- No few-shot examples
- No domain-specific guidance

#### 3. **Limited Constraints** 🟡
- Only specifies 15-90 second clips
- No minimum gap between clips
- No maximum total duration
- No guidance on clip diversity

#### 4. **Simple Validation** 🟡
- Just matches nearest VTT timestamp
- No quality scoring
- No overlap detection
- No context awareness

#### 5. **User Experience** 🔴
- All clips auto-selected
- No timestamp editing
- Can't provide feedback for re-analysis
- Binary select/deselect only

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
1. **Model selection** - Choose based on video complexity
2. **Fallback strategy** - Use cheaper models if turbo unavailable
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
