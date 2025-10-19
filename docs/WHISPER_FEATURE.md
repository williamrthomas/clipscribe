# Automatic Transcript Generation with Whisper

ClipScribe now supports **automatic transcript generation** using OpenAI's Whisper API. This means you can process videos without pre-existing transcripts!

## How It Works

When you upload a video without a transcript:

1. **Extract Audio** - FFmpeg extracts audio from your video as MP3
2. **Transcribe** - Whisper AI converts speech to text with timestamps
3. **Generate VTT** - Creates a properly formatted VTT transcript file
4. **Analyze** - Proceeds with normal AI clip identification

## Using the Feature

### Step 1: Upload Video Only
- Select your video file (`.mp4`, `.mov`, `.mkv`)
- Leave the transcript field empty

### Step 2: Generate Transcript
- Click the **"Generate Transcript (Whisper AI)"** button
- Wait while the audio is extracted and transcribed
- Progress updates will show current status

### Step 3: Review & Continue
- Once complete, the transcript path is automatically filled
- The VTT file is saved next to your video
- Click **"Analyze & Find Clips"** to continue

## Costs

OpenAI Whisper API pricing:
- **$0.006 per minute** of audio
- Examples:
  - 30-minute meeting: ~$0.18
  - 1-hour presentation: ~$0.36
  - 2-hour conference: ~$0.72

## Technical Details

### Audio Processing
- Extracted to temporary MP3 file
- 16kHz sample rate (Whisper requirement)
- Mono audio channel
- 64kbps bitrate for smaller file size
- Temporary file deleted after transcription

### API Limits
- Maximum file size: **25MB** (after audio extraction)
- Most videos under 2 hours fit within this limit
- Longer videos may need to be split

### Supported Languages
Currently configured for English, but Whisper supports:
- 50+ languages
- Auto-detection available
- Can be configured in settings (future feature)

## Comparison: Manual vs Auto-Generated Transcripts

| Feature | Manual VTT | Whisper AI |
|---------|-----------|------------|
| **Accuracy** | 100% (human-verified) | ~95% (AI-generated) |
| **Cost** | Free (if you have it) | ~$0.006/minute |
| **Time** | Instant upload | 2-5 minutes processing |
| **Timestamps** | Frame-accurate | Word-level accurate |
| **Editing** | Pre-edited | Raw transcription |

## Best Practices

### When to Use Manual Transcripts
- You already have a VTT file
- Meeting has sensitive information
- Need 100% accuracy for legal/compliance
- Want to review/edit before processing

### When to Use Whisper
- No transcript available
- Quick processing needed
- Good audio quality
- English or supported language
- Non-sensitive content

## Troubleshooting

### "Audio file too large (max 25MB)"
**Solution:** Video is too long or high bitrate
- Try a shorter video segment
- Use lower bitrate when recording
- Split long videos into chunks

### "Whisper API error (400)"
**Solution:** Audio format issue
- Ensure video has audio track
- Check video isn't corrupted
- Verify FFmpeg is working

### "Failed to extract audio"
**Solution:** FFmpeg issue
- Verify FFmpeg is installed correctly
- Check video file is valid format
- Ensure sufficient disk space

### Poor Transcription Quality
**Factors affecting accuracy:**
- Background noise
- Multiple speakers talking over each other
- Heavy accents
- Technical jargon
- Audio quality

**Improvements:**
- Use videos with clear audio
- One speaker at a time
- Good microphone quality
- Minimal background noise

## File Organization

Generated transcripts are saved alongside your video:

```
/path/to/your/videos/
├── meeting.mp4                    # Original video
├── meeting.vtt                    # Auto-generated transcript
└── meeting_Clips/                 # Generated clips folder
    ├── 1_Budget_Discussion.mp4
    ├── 2_Action_Items.mp4
    └── 3_Next_Steps.mp4
```

## API Requirements

Uses the same OpenAI API key as GPT-4 analysis:
- No separate Whisper API key needed
- Billed to same OpenAI account
- Usage visible in OpenAI dashboard

## Future Enhancements

Planned improvements:
- [ ] Language selection in UI
- [ ] Speaker diarization (identify speakers)
- [ ] Transcript editing before analysis
- [ ] Batch transcript generation
- [ ] Local Whisper model option (free but slower)
- [ ] Save/load transcript drafts

## Privacy & Security

**Your data:**
- Audio sent to OpenAI for transcription
- Processed according to OpenAI's data policy
- Not used for model training (API data)
- Deleted from OpenAI servers after processing

**For sensitive content:**
- Use manual transcripts instead
- Or use local Whisper model (coming soon)
- Review OpenAI's data usage policy

## Code Integration

The Whisper service is implemented in Rust:
- `src-tauri/src/services/whisper.rs` - Core transcription logic
- `src-tauri/src/commands/transcribe.rs` - Tauri command
- FFmpeg sidecar for audio extraction
- Multipart form upload to Whisper API

Frontend integration:
- Automatic detection when transcript is missing
- Progress events for real-time updates
- Error handling with user-friendly messages

## Performance

Typical processing times:
- **Audio extraction:** 5-30 seconds
- **API upload:** 5-15 seconds (depends on file size)
- **Transcription:** 20-60 seconds (depends on audio length)
- **Total:** 30 seconds - 2 minutes for most videos

Longer videos take proportionally longer.

---

**Questions?** See [README.md](README.md) or [open an issue](https://github.com/yourusername/clipscribe/issues)
