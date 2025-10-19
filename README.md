# ClipScribe

**AI-powered video clip extraction tool** - Automatically identify and extract the most important moments from meeting recordings using AI.

![ClipScribe](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

✨ **AI-Powered Analysis** - Uses OpenAI GPT-4 to intelligently identify key moments in meetings  
🎬 **Automatic Clip Generation** - Extracts video clips with precise timestamps  
📝 **VTT Support** - Works with WebVTT transcript files  
🎤 **Auto-Transcription** - NEW! Generate transcripts automatically using Whisper AI  
🖥️ **Cross-Platform** - Runs on macOS (Intel & Apple Silicon) and Windows  
⚡ **Fast Processing** - Uses FFmpeg stream copy for quick clip extraction  
🔒 **Secure** - API keys stored encrypted locally

## Prerequisites

Before you begin, ensure you have:

1. **Node.js 18+** - [Download here](https://nodejs.org/)
2. **Rust 1.70+** - [Install via rustup](https://rustup.rs/)
3. **OpenAI API Key** - [Get one here](https://platform.openai.com/api-keys)
4. **FFmpeg binary** - See [FFmpeg Setup](#ffmpeg-setup) below

## Installation

### For Developers

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/clipscribe.git
   cd clipscribe
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Set up FFmpeg** (see [FFmpeg Setup](#ffmpeg-setup))

4. **Run in development mode**
   ```bash
   npm run tauri:dev
   ```

### Building for Production

```bash
npm run tauri:build
```

This will create installers in `src-tauri/target/release/bundle/`:
- **macOS**: `.dmg` and `.app` in `dmg/` and `macos/`
- **Windows**: `.msi` installer in `msi/`

## FFmpeg Setup

ClipScribe requires FFmpeg as a bundled binary for video processing.

### macOS

Download the appropriate static build:

**Intel Macs:**
```bash
curl -L https://evermeet.cx/ffmpeg/ffmpeg-6.0.zip -o ffmpeg.zip
unzip ffmpeg.zip
mv ffmpeg src-tauri/bin/ffmpeg-x86_64-apple-darwin
chmod +x src-tauri/bin/ffmpeg-x86_64-apple-darwin
```

**Apple Silicon Macs:**
```bash
curl -L https://evermeet.cx/ffmpeg/ffmpeg-6.0.zip -o ffmpeg.zip
unzip ffmpeg.zip
mv ffmpeg src-tauri/bin/ffmpeg-aarch64-apple-darwin
chmod +x src-tauri/bin/ffmpeg-aarch64-apple-darwin
```

### Windows

1. Download FFmpeg static build from [gyan.dev](https://www.gyan.dev/ffmpeg/builds/)
2. Extract `ffmpeg.exe` from the archive
3. Rename it to `ffmpeg-x86_64-pc-windows-msvc.exe`
4. Place it in `src-tauri/bin/`

## Usage

### 1. Configure API Key

On first launch, click the **Settings** icon (⚙️) and enter your OpenAI API key.

### 2. Select Files

- **Video File**: Drop or browse for your `.mp4`, `.mov`, or `.mkv` file
- **Transcript File**: Drop or browse for your `.vtt` transcript file
  - **OR** Generate transcript automatically using Whisper AI (if you don't have one)

### 3. Add Context (Optional)

Provide guidance for the AI, such as:
- "Focus on technical decisions"
- "Find action items and next steps"
- "Extract product announcements"

### 4. Analyze & Review

Click **Analyze & Find Clips** to let AI identify key moments. Review the suggested clips and toggle which ones to generate.

### 5. Generate Clips

Click **Generate Clips** to extract selected clips. They'll be saved in a new folder next to your original video: `[VideoName]_Clips/`

## Project Structure

```
clipscribe/
├── src/                      # React frontend
│   ├── components/           # UI components
│   ├── hooks/                # React hooks
│   ├── types/                # TypeScript types
│   └── App.tsx               # Main app component
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── commands/         # Tauri commands
│   │   ├── models/           # Data models
│   │   ├── services/         # Business logic
│   │   └── main.rs           # Entry point
│   ├── bin/                  # FFmpeg binaries
│   └── tauri.conf.json       # Tauri configuration
├── package.json              # Node dependencies
└── README.md                 # This file
```

## How It Works

1. **Upload** - You provide a video file and VTT transcript
2. **Parse** - ClipScribe parses the transcript to extract timestamps and text
3. **Analyze** - OpenAI GPT-4 analyzes the transcript to identify key moments
4. **Validate** - Timestamps are validated and mapped to actual VTT cues
5. **Extract** - FFmpeg extracts video clips using precise timestamps
6. **Save** - Clips are saved with descriptive filenames in a new folder

## API Costs

ClipScribe uses OpenAI's APIs. Approximate costs:

**GPT-4 Turbo (Clip Analysis):**
- **Per analysis**: $0.01 - $0.10 (depending on transcript length)
- Most meetings under 2 hours: < $0.05

**Whisper (Auto-Transcription):** *(Optional)*
- **Per minute**: $0.006
- 30-minute meeting: ~$0.18
- 1-hour meeting: ~$0.36

**Total typical cost:** $0.20 - $0.50 per video with auto-transcription

## Troubleshooting

### "No API key configured"
- Go to Settings and add your OpenAI API key
- Verify the key is valid by clicking "Save"

### "Failed to find FFmpeg binary"
- Ensure FFmpeg is placed in `src-tauri/bin/` with correct naming
- On macOS/Linux, verify executable permissions: `chmod +x src-tauri/bin/ffmpeg-*`

### "FFmpeg exited with code 234"  
- **Fixed in v1.1.0** - Now uses H.264 encoding for universal compatibility
- Previously failed with ProRes and other professional codecs
- See `rca/ROOT_CAUSE_FOUND.md` for technical details

### "Invalid VTT file"
- Ensure your file starts with `WEBVTT`
- Check that timestamps are in format `HH:MM:SS.mmm --> HH:MM:SS.mmm`

### Clips are cut off or timestamps are wrong
- VTT timestamps may not perfectly align with video
- Try a different transcript source or re-generate your VTT file

### Slow clip generation
- ClipScribe now re-encodes to H.264 for compatibility
- ProRes/professional formats take longer than H.264 source
- Processing time depends on video length and system specs

## Development

### Tech Stack

- **Frontend**: React 18, TypeScript, TailwindCSS
- **Backend**: Rust, Tauri 1.5
- **AI**: OpenAI GPT-4 Turbo
- **Video Processing**: FFmpeg

### Running Tests

```bash
# Rust tests
cd src-tauri
cargo test

# Frontend tests (add your own)
npm test
```

### Code Style

- **Rust**: Use `cargo fmt` and `cargo clippy`
- **TypeScript**: Follows standard Prettier/ESLint config

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- OpenAI for GPT-4 API
- FFmpeg for video processing
- Tauri for the desktop framework
- React and the open-source community

## Support

For issues, questions, or suggestions:
- 📧 Email: support@clipscribe.app
- 🐛 Issues: [GitHub Issues](https://github.com/yourusername/clipscribe/issues)
- 📖 Docs: [Full Documentation](docs/)

---

**Built with ❤️ using Tauri, React, and Rust**
