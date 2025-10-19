# Changelog

All notable changes to ClipScribe will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-10-18

### Added
- Initial release of ClipScribe
- AI-powered video clip extraction using OpenAI GPT-4
- VTT transcript parsing and validation
- FFmpeg integration for video processing
- Cross-platform support (macOS, Windows)
- Secure API key storage using Tauri plugin store
- Modern React + TypeScript frontend with TailwindCSS
- Rust backend with Tauri framework
- File drag-and-drop support (with browse fallback)
- Real-time progress tracking during clip generation
- Clip review interface with toggle selection
- Automatic output folder creation
- "Show in Folder" functionality
- Settings modal for API key management
- Comprehensive documentation (README, SETUP, QUICKSTART)
- Sample VTT transcript file for testing
- Error handling and user feedback

### Features
- **AI Analysis**: GPT-4 identifies key moments in meetings
- **Timestamp Matching**: Maps AI suggestions to actual VTT timestamps
- **Fast Processing**: Uses FFmpeg stream copy for quick extraction
- **Smart Naming**: Sanitizes filenames and adds sequence numbers
- **Context Support**: Optional user guidance for AI analysis
- **Multiple Formats**: Supports MP4, MOV, MKV video files

### Technical
- Tauri 1.5+ for desktop framework
- React 18 with TypeScript
- Rust 1.70+ backend
- TailwindCSS for styling
- Lucide icons
- OpenAI GPT-4 Turbo API integration
- FFmpeg 6.0+ for video processing
- VTT parser with regex-based timestamp extraction

### Documentation
- Complete README with features and usage
- Detailed SETUP guide for developers
- QUICKSTART for 5-minute setup
- CONTRIBUTING guidelines
- Engineering specification
- Sample transcript file
- License (MIT)

## [1.1.0] - 2024-10-19  ✅ FIRST WORKING VERSION

### Added
- **Automatic Transcript Generation** using OpenAI Whisper API
- Extract audio from video using FFmpeg
- Transcribe audio to VTT format with timestamps
- "Generate Transcript" button in UI when no transcript provided
- Real-time progress updates during transcription
- Cost estimation display ($0.006/minute)
- Comprehensive Whisper feature documentation (WHISPER_FEATURE.md)
- Support for videos without pre-existing transcripts
- Comprehensive logging system for debugging
- RCA (Root Cause Analysis) documentation structure
- Test and logging infrastructure

### Changed
- Updated README with auto-transcription feature
- Updated API costs section to include Whisper pricing
- Enhanced user workflow: video-only → auto-transcribe → analyze → generate clips
- **CRITICAL FIX:** Changed from stream copy to H.264 encoding for universal codec support
- Now supports ProRes, DNxHD, and all professional video codecs

### Fixed
- **FFmpeg exit code 234** - ProRes and professional codecs now supported
- Codec/container compatibility issues resolved
- File-based settings storage (replaced tauri-plugin-store dependency issue)

### Technical
- New `whisper.rs` service for audio transcription
- New `transcribe.rs` command module
- Audio extraction with FFmpeg (MP3, 16kHz, mono)
- Multipart form upload to Whisper API
- Event-based progress tracking
- Automatic VTT file creation alongside video
- H.264 encoding with configurable quality (CRF 23)
- AAC audio encoding for universal compatibility
- Comprehensive FFmpeg logging to stdout/stderr

## [1.2.0] - 2024-10-19

### Added
- **GPT-5-mini Integration** - Upgraded from GPT-4 Turbo to GPT-5-mini with Responses API
- **User instruction priority system** - User context now always overrides default prompts
- **VTT-aware analysis** - AI analyzes full VTT structure for better timestamp accuracy
- **Excitement-focused defaults** - New prompt finds interesting/exciting moments by default
- **Enhanced logging** - Comprehensive debugging output for API interactions
- **Proper error detection** - Distinguishes between null errors and actual failures

### Changed
- **Clip length range** - Expanded from 15-90s to 10-120s
- **Clip count** - Increased from 3-7 to 3-8 clips maximum
- **Prompt structure** - User instructions always listed first in prompt
- **API endpoint** - Switched from Chat Completions to Responses API

### Fixed
- **Response parsing** - Correctly handles nested GPT-5 response structure
- **Error detection** - No longer treats successful responses as errors

### Technical
- Responses API integration with nested output structure
- Minimal reasoning effort for fast, cost-effective analysis
- Low verbosity for concise JSON responses
- Full VTT formatting preserves cue structure

## [Unreleased]

### Planned Features
- Language selection for Whisper transcription
- Speaker diarization (identify who said what)
- Transcript editing before analysis
- Local Whisper model option (free but slower)
- Clip preview before generation
- Batch processing multiple videos
- Clip quality scoring and ranking
- Overlap detection
- Dynamic reasoning effort based on video complexity
- Custom clip padding (add seconds before/after)
- Export settings (resolution, bitrate, format)
- Alternative AI providers (Anthropic Claude, Azure OpenAI)
- Keyboard shortcuts
- Dark/light theme toggle
- Clip history and favorites
- Advanced VTT features (chapters)

### Known Issues
- Drag-and-drop doesn't expose file paths in Tauri (security limitation)
- First compilation takes 3-5 minutes (Rust dependencies)
- FFmpeg binaries must be downloaded separately (not included in repo)
- No app icons yet (using Tauri defaults)
- Limited test coverage

### Future Enhancements
- Automated testing suite
- CI/CD pipeline
- Auto-updater integration
- Cloud sync for settings
- Templates for different meeting types
- AI model selection (GPT-3.5 vs GPT-4)
- Cost estimation before analysis
- Undo/redo functionality
- Clip export to cloud storage

---

## Version History

**1.0.0** - Initial release with core features

[1.0.0]: https://github.com/yourusername/clipscribe/releases/tag/v1.0.0
[Unreleased]: https://github.com/yourusername/clipscribe/compare/v1.0.0...HEAD
