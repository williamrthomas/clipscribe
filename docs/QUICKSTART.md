# QuickStart Guide

Get ClipScribe up and running in 5 minutes.

## Prerequisites

✅ Node.js 18+ installed  
✅ Rust installed  
✅ OpenAI API key ready  

## Installation Steps

### 1. Clone & Install

```bash
git clone https://github.com/yourusername/clipscribe.git
cd clipscribe
npm install
```

### 2. Download FFmpeg

**macOS:**
```bash
mkdir -p src-tauri/bin
curl -L https://evermeet.cx/ffmpeg/ffmpeg-6.0.zip -o ffmpeg.zip
unzip ffmpeg.zip
mv ffmpeg src-tauri/bin/ffmpeg-x86_64-apple-darwin
chmod +x src-tauri/bin/ffmpeg-x86_64-apple-darwin
rm ffmpeg.zip
```

**Windows:**
1. Download from: https://www.gyan.dev/ffmpeg/builds/
2. Extract `ffmpeg.exe`
3. Rename to `ffmpeg-x86_64-pc-windows-msvc.exe`
4. Move to `src-tauri\bin\`

### 3. Run

```bash
npm run tauri:dev
```

First run takes 3-5 minutes to compile. Be patient! ☕

### 4. Configure

1. Click the ⚙️ Settings icon
2. Enter your OpenAI API key
3. Click Save

### 5. Test

1. **Video**: Select any `.mp4` video file
2. **Transcript**: Create a test `.vtt` file:

```vtt
WEBVTT

00:00:00.000 --> 00:00:05.000
This is a test meeting transcript.

00:00:10.000 --> 00:00:20.000
We discussed important project updates.

00:01:00.000 --> 00:01:10.000
Action item: Complete the proposal by Friday.
```

3. Click **Analyze & Find Clips**
4. Review suggested clips
5. Click **Generate Clips**

## Common Issues

### "Command not found: cargo"
```bash
# Restart terminal after Rust install
source $HOME/.cargo/env
```

### "FFmpeg not found"
```bash
# Check it exists
ls -la src-tauri/bin/

# Make executable (macOS/Linux)
chmod +x src-tauri/bin/ffmpeg-*
```

### "Port 1420 already in use"
```bash
# Kill the process
lsof -ti:1420 | xargs kill -9  # macOS/Linux
```

### "API key invalid"
- Get a new key from https://platform.openai.com/api-keys
- Ensure you have credits in your OpenAI account

## Next Steps

📖 Read the full [README.md](README.md)  
🔧 Check [SETUP.md](SETUP.md) for detailed setup  
📋 Review [engineering-spec.md](engineering-spec.md) for architecture

## Need Help?

- 🐛 [GitHub Issues](https://github.com/yourusername/clipscribe/issues)
- 💬 [Discussions](https://github.com/yourusername/clipscribe/discussions)

---

**Happy clipping! 🎬**
