# RCA: FFmpeg Exit Code 234

**Date:** 2024-10-19  
**Issue:** FFmpeg clip generation failing with exit code 234  
**Impact:** Cannot generate video clips from analyzed timestamps  

## Observed Behavior

- ✅ Transcript generation works (Whisper API)
- ✅ AI analysis works (GPT-4 identifies clips)
- ✅ Clip review UI works
- ❌ FFmpeg clip extraction fails with code 234

## Error Details

```
Error: FFmpeg exited with code: Some(234)
```

## Investigation Steps

### 1. Verify FFmpeg Binary

```bash
ls -la src-tauri/bin/ffmpeg-aarch64-apple-darwin
# Expected: executable permissions, ~76MB file
```

**Result:** 
- Binary exists: ✅
- Permissions: ✅ (rwxr-xr-x)
- Size: 76MB

### 2. Test FFmpeg Directly

```bash
./src-tauri/bin/ffmpeg-aarch64-apple-darwin -version
```

**Expected output:** FFmpeg version info  
**Actual output:** (to be filled)

### 3. Check Tauri Sidecar Configuration

**File:** `src-tauri/tauri.conf.json`

```json
{
  "shell": {
    "sidecar": true,
    "scope": [
      {
        "name": "ffmpeg",
        "sidecar": true,
        "args": true
      }
    ]
  },
  "bundle": {
    "externalBin": ["bin/ffmpeg"]
  }
}
```

**Issue:** Tauri appends platform suffix automatically
- Config says: `bin/ffmpeg`
- Tauri looks for: `bin/ffmpeg-aarch64-apple-darwin`
- File is at: `bin/ffmpeg-aarch64-apple-darwin` ✅

### 4. Check FFmpeg Command Being Executed

Need to add logging to see exact command + arguments.

## Exit Code 234 Analysis

Exit code 234 is not a standard FFmpeg error code. Possible causes:

1. **Binary execution failed** - Permission, quarantine, or path issue
2. **Sidecar spawn failed** - Tauri can't execute the binary
3. **macOS Gatekeeper** - Binary not signed, blocked by security
4. **Invalid arguments** - FFmpeg rejects the command
5. **Async/event handling issue** - Process terminates unexpectedly

## Hypotheses (Priority Order)

### Hypothesis 1: macOS Gatekeeper Blocking Binary
**Probability:** HIGH  
**Test:** Check for quarantine attribute
```bash
xattr -l src-tauri/bin/ffmpeg-aarch64-apple-darwin
# Look for: com.apple.quarantine
```

**If quarantined, remove:**
```bash
xattr -d com.apple.quarantine src-tauri/bin/ffmpeg-aarch64-apple-darwin
```

### Hypothesis 2: Invalid FFmpeg Command Arguments
**Probability:** MEDIUM  
**Test:** Add comprehensive logging to see exact command  
**Action:** Add logging to `ffmpeg.rs`

### Hypothesis 3: File Path Issues
**Probability:** LOW  
**Reason:** Binary exists and has correct permissions  

### Hypothesis 4: Tauri Sidecar Not Finding Binary
**Probability:** MEDIUM  
**Test:** Check Tauri's binary resolution in dev mode  

## Next Steps

1. ✅ Create logging infrastructure
2. ⏳ Add detailed logging to FFmpeg service
3. ⏳ Check for quarantine attributes
4. ⏳ Test FFmpeg binary directly
5. ⏳ Capture exact command being sent to FFmpeg
6. ⏳ Check stderr output from FFmpeg
7. ⏳ Test with minimal FFmpeg command

## Logging Strategy

### Add to `services/ffmpeg.rs`:
- Log exact command + args before spawn
- Log all stdout/stderr from FFmpeg
- Log process exit codes
- Save to log file for analysis

### Add to frontend:
- Log video path being sent
- Log clip timestamps
- Log full error messages

## Resolution

**Status:** In Progress  
**Root Cause:** TBD  
**Fix:** TBD  

---

## Updates Log

**2024-10-19 11:51:** Investigation started, logging infrastructure created
