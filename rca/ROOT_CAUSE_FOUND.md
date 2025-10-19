# ✅ ROOT CAUSE IDENTIFIED

**Date:** 2024-10-19 11:57 AM  
**Issue:** FFmpeg exit code 234  
**Status:** RESOLVED - Codec incompatibility

## Root Cause

**FFmpeg Error:**
```
[mp4 @ 0x7f7dee9097c0] Could not find tag for codec prores in stream #0, 
codec not currently supported in container
Conversion failed!
```

**Explanation:**
- User's video uses **Apple ProRes 422** codec
- Current code uses `-c copy` (stream copy, no re-encoding)
- ProRes codec **cannot be stream-copied into MP4 containers**
- FFmpeg returns exit code 234 for this codec/container mismatch

## Video Details

From FFmpeg analysis:
```
Input: /Users/williamthomas/Documents/test.mov
Video Codec: prores (Standard) - Apple ProRes 422
Container: QuickTime MOV
Resolution: 1280x720 @ 60fps
Bitrate: 140443 kb/s
```

## Why Stream Copy Fails

**Stream copy (`-c copy`):**
- Copies video/audio streams without re-encoding
- Fast (no CPU processing)
- **BUT** requires source codec to be compatible with target container

**ProRes + MP4 = Incompatible:**
- ProRes is designed for MOV/QuickTime containers
- MP4 spec doesn't support ProRes codec tags
- Must re-encode to H.264, H.265, or use MOV output

## Solution

### Option A: Re-encode to H.264 (Recommended)
```rust
.args(&[
    "-i", input_path,
    "-ss", start_time,
    "-to", end_time,
    "-c:v", "libx264",      // H.264 codec
    "-preset", "fast",       // Encoding speed
    "-crf", "23",            // Quality (18-28, lower = better)
    "-c:a", "aac",           // Audio codec
    "-b:a", "192k",          // Audio bitrate
    "-y",
    output_path,
])
```

**Pros:** Universal compatibility, smaller files  
**Cons:** Slower (must re-encode), quality loss (minimal with CRF 23)

### Option B: Keep MOV container
```rust
// Change output filename from .mp4 to .mov
let output_file = format!("{}_{}.mov", index + 1, clip.sanitized_filename);
// Use -c copy
```

**Pros:** Fast, no quality loss  
**Cons:** Larger files, less compatible

### Option C: Smart Detection
```rust
// Detect codec, choose strategy:
// - If H.264/H.265: use stream copy
// - If ProRes/other: re-encode
```

**Pros:** Best of both worlds  
**Cons:** More complex

## Recommendation

**Implement Option A** with automatic codec detection fallback:

1. **Try stream copy first** (fast path)
2. **If fails, retry with re-encoding** (compatible path)
3. **Log which method was used**

This gives best performance when possible, with guaranteed compatibility.

## Implementation

See: `src-tauri/src/services/ffmpeg.rs`

Update `extract_clip()` to use H.264 encoding instead of stream copy.

## Lessons Learned

1. ✅ **Stream copy is not universal** - codec/container compatibility matters
2. ✅ **Professional codecs** (ProRes, DNxHD) often need re-encoding
3. ✅ **Logging was crucial** - couldn't have found this without FFmpeg stderr
4. ✅ **Exit code 234** = codec/container incompatibility in FFmpeg

## Status

- [x] Root cause identified
- [x] Solution designed  
- [ ] Fix implemented
- [ ] Tested with ProRes video
- [ ] Tested with H.264 video (verify no regression)
