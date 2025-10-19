# Investigation Plan - FFmpeg Error 234

## Status: ✅ Logging Infrastructure Complete

## What We've Done

1. ✅ Created investigation directories (`tests/`, `logs/`, `rca/`)
2. ✅ Verified FFmpeg binary works directly
3. ✅ Confirmed no macOS quarantine blocking
4. ✅ Added comprehensive logging to:
   - FFmpeg clip extraction service
   - Whisper audio extraction service
   - All process events (stdout, stderr, errors, termination)
5. ✅ Created test script for log capture

## Next Steps for Debugging

### Step 1: Reproduce Error with Logging
1. Restart the app to get new logging code
2. Try generating clips again
3. Monitor console output for detailed logs

### Step 2: Analyze Console Output

Look for these specific log lines:

```
=== FFmpeg Clip Extraction ===
Input: /path/to/video.mov
Start: HH:MM:SS
End: HH:MM:SS
Output: /path/to/output.mp4
FFmpeg args: [...]
FFmpeg process spawned successfully
```

Then check for:
- **STDOUT/STDERR output** from FFmpeg
- **Termination code and signal**
- **Full error messages**

### Step 3: Common Patterns to Check

#### Pattern 1: Binary Not Found
```
ERROR: Failed to find FFmpeg binary: ...
```
→ Check sidecar configuration

#### Pattern 2: Spawn Failure  
```
ERROR: Failed to spawn FFmpeg: ...
```
→ Check permissions or binary corruption

#### Pattern 3: FFmpeg Error
```
FFmpeg STDERR: [error message]
FFmpeg terminated with code: Some(234)
```
→ Actual FFmpeg execution error

#### Pattern 4: Invalid Arguments
```
FFmpeg STDERR: Unrecognized option: ...
```
→ Command syntax issue

## Expected Findings

Based on the binary working directly, the most likely issues are:

1. **Invalid file paths** - Spaces, special characters, or encoding issues
2. **Invalid timestamps** - Format mismatch or out-of-bounds times
3. **Permission issues** - Can't read input or write output
4. **Codec issues** - Video format not compatible with stream copy

## Resolution Strategy

Once we see the logs, we'll:

1. Identify the exact failure point
2. Test FFmpeg command manually with same args
3. Fix the specific issue (path encoding, timestamp format, etc.)
4. Verify fix works
5. Document root cause

## How to Use

### Run with Logging
```bash
./tests/run-with-logging.sh
```

OR manually:
```bash
npm run tauri:dev 2>&1 | tee logs/debug-session.log
```

### Read Current Console
Check the terminal where `npm run tauri:dev` is running

### After Error Occurs
1. Copy all console output
2. Save to `logs/error-reproduction-[timestamp].log`
3. Analyze for patterns above
4. Update this document with findings

## Success Criteria

We'll know we've succeeded when:
- ✅ We can see the exact FFmpeg command being executed
- ✅ We can see the exact error message from FFmpeg
- ✅ We understand why code 234 is being returned
- ✅ We can reproduce the error manually
- ✅ We implement a fix that resolves it
