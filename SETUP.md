# ClipScribe Setup Guide

Complete setup instructions for developers working on ClipScribe.

## System Requirements

### Operating System
- macOS 11+ (Big Sur or later)
- Windows 10/11
- Linux (Ubuntu 20.04+ or equivalent)

### Development Tools
- **Node.js**: 18.x or higher ([Download](https://nodejs.org/))
- **Rust**: 1.70 or higher ([Install via rustup](https://rustup.rs/))
- **Git**: Latest version
- **Code Editor**: VS Code recommended

## Step-by-Step Setup

### 1. Install Node.js and Rust

**Install Node.js:**
```bash
# macOS (using Homebrew)
brew install node@18

# Windows (using Chocolatey)
choco install nodejs-lts

# Or download from https://nodejs.org/
```

**Install Rust:**
```bash
# All platforms
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts and restart your terminal
rustc --version  # Verify installation
```

### 2. Clone and Install Dependencies

```bash
# Clone the repository
git clone https://github.com/yourusername/clipscribe.git
cd clipscribe

# Install Node dependencies
npm install

# This will install:
# - React, React DOM
# - Tauri API bindings
# - Vite build tool
# - TailwindCSS
# - TypeScript
# - Lucide icons
```

### 3. Set Up Tauri Dependencies

**macOS:**
```bash
# Install system dependencies
xcode-select --install
```

**Windows:**
```bash
# Install Visual Studio C++ Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/
# Select "Desktop development with C++"
```

**Linux:**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### 4. Download FFmpeg Binaries

ClipScribe requires platform-specific FFmpeg binaries.

#### macOS

**For Intel Macs (x86_64):**
```bash
# Create bin directory
mkdir -p src-tauri/bin

# Download FFmpeg
curl -L https://evermeet.cx/ffmpeg/ffmpeg-6.0.zip -o ffmpeg.zip
unzip ffmpeg.zip

# Rename and move to correct location
mv ffmpeg src-tauri/bin/ffmpeg-x86_64-apple-darwin
chmod +x src-tauri/bin/ffmpeg-x86_64-apple-darwin

# Clean up
rm ffmpeg.zip
```

**For Apple Silicon Macs (ARM64):**
```bash
mkdir -p src-tauri/bin

# Download ARM64 build
curl -L https://evermeet.cx/ffmpeg/ffmpeg-6.0.zip -o ffmpeg.zip
unzip ffmpeg.zip

# Rename and move
mv ffmpeg src-tauri/bin/ffmpeg-aarch64-apple-darwin
chmod +x src-tauri/bin/ffmpeg-aarch64-apple-darwin

rm ffmpeg.zip
```

**Build for both architectures (Universal Binary):**
```bash
# You'll need both binaries in src-tauri/bin/
# Tauri will automatically select the correct one
```

#### Windows

```powershell
# Create bin directory
New-Item -ItemType Directory -Path src-tauri\bin -Force

# Download FFmpeg from https://www.gyan.dev/ffmpeg/builds/
# Get the "release essentials" build

# Extract ffmpeg.exe from the archive
# Rename to: ffmpeg-x86_64-pc-windows-msvc.exe
# Move to: src-tauri\bin\
```

Or use this PowerShell script:
```powershell
$url = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
$output = "ffmpeg.zip"
Invoke-WebRequest -Uri $url -OutFile $output
Expand-Archive -Path $output -DestinationPath .
Move-Item "ffmpeg-*\bin\ffmpeg.exe" "src-tauri\bin\ffmpeg-x86_64-pc-windows-msvc.exe"
Remove-Item $output
Remove-Item "ffmpeg-*" -Recurse
```

### 5. Verify Setup

```bash
# Check Node.js
node --version  # Should be 18.x or higher

# Check Rust
rustc --version  # Should be 1.70 or higher
cargo --version

# Check FFmpeg binary exists
ls -la src-tauri/bin/  # Should show ffmpeg-* file(s)

# Build Rust backend (test compilation)
cd src-tauri
cargo build
cd ..
```

### 6. Configure Development Environment

**VS Code (Recommended Extensions):**
```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tauri-apps.tauri-vscode",
    "dbaeumer.vscode-eslint",
    "esbenp.prettier-vscode",
    "bradlc.vscode-tailwindcss"
  ]
}
```

Save this as `.vscode/extensions.json` in your project root.

### 7. Run Development Server

```bash
# Start the development server
npm run tauri:dev

# This will:
# 1. Start Vite dev server (React frontend) on port 1420
# 2. Compile Rust backend
# 3. Launch the Tauri app window
```

First run will take 3-5 minutes to compile Rust dependencies. Subsequent runs are much faster.

## Configuration

### Environment Variables (Optional)

Create a `.env` file in the root directory:

```env
# Not used by default, but available for future features
VITE_API_URL=https://api.openai.com/v1
```

### Tauri Configuration

The main config is in `src-tauri/tauri.conf.json`. Key sections:

- **bundle.identifier**: App identifier (e.g., `com.clipscribe.app`)
- **bundle.externalBin**: FFmpeg binary paths
- **allowlist**: File system and API permissions
- **windows**: App window settings

## Getting an OpenAI API Key

1. Go to [OpenAI Platform](https://platform.openai.com/)
2. Sign up or log in
3. Navigate to **API Keys** section
4. Click **Create new secret key**
5. Copy the key (starts with `sk-`)
6. Add it in ClipScribe Settings once the app is running

**Costs:** GPT-4 Turbo costs approximately $0.01-0.10 per transcript analysis.

## Testing the Application

### Manual Testing Checklist

1. **Settings Modal**
   - [ ] Opens when clicking settings icon
   - [ ] Accepts API key input
   - [ ] Validates API key correctly
   - [ ] Saves API key securely

2. **File Upload**
   - [ ] Browse button opens file dialog
   - [ ] Accepts .mp4, .mov, .mkv videos
   - [ ] Accepts .vtt transcript files
   - [ ] Shows selected filenames

3. **Analysis**
   - [ ] "Analyze" button disabled without files
   - [ ] Shows loading spinner during analysis
   - [ ] Displays clips after analysis
   - [ ] Handles errors gracefully

4. **Clip Review**
   - [ ] Can toggle clip selection
   - [ ] Shows timestamps correctly
   - [ ] Generate button shows count

5. **Processing**
   - [ ] Progress bar updates
   - [ ] Creates output folder correctly
   - [ ] Generates clips with correct names
   - [ ] "Show in Folder" opens file explorer

### Sample Files for Testing

Create a test VTT file (`test.vtt`):

```
WEBVTT

00:00:00.000 --> 00:00:05.000
Welcome everyone to our Q4 planning meeting.

00:00:05.000 --> 00:00:10.000
Today we'll discuss the budget and upcoming projects.

00:01:00.000 --> 00:01:15.000
I'd like to propose we increase the marketing budget by 20%.

00:01:15.000 --> 00:01:25.000
That sounds good. Let's approve that and move forward.
```

Use any short video file for testing.

## Troubleshooting

### Port Already in Use

If port 1420 is in use:
```bash
# Find and kill the process
lsof -ti:1420 | xargs kill -9  # macOS/Linux
netstat -ano | findstr :1420   # Windows
```

Or change the port in `vite.config.ts`:
```typescript
server: {
  port: 3000,  // Change to any available port
  strictPort: true,
}
```

### Rust Compilation Errors

```bash
# Update Rust
rustup update stable

# Clean and rebuild
cd src-tauri
cargo clean
cargo build
```

### FFmpeg Not Found

```bash
# Verify binary exists and has correct name
ls -la src-tauri/bin/

# Check permissions (macOS/Linux)
chmod +x src-tauri/bin/ffmpeg-*

# Verify it runs
./src-tauri/bin/ffmpeg-x86_64-apple-darwin -version
```

### API Key Issues

- Ensure key starts with `sk-`
- Check you have sufficient OpenAI credits
- Verify network connection
- Check for firewall blocking HTTPS requests

## Building for Production

```bash
# Build optimized release
npm run tauri:build

# Output locations:
# macOS: src-tauri/target/release/bundle/dmg/
# Windows: src-tauri/target/release/bundle/msi/
```

### Code Signing (Optional)

**macOS:**
```bash
# Requires Apple Developer account
export APPLE_CERTIFICATE="Developer ID Application: Your Name"
export APPLE_CERTIFICATE_PASSWORD="your-password"
export APPLE_ID="your-apple-id@email.com"
export APPLE_TEAM_ID="XXXXXXXXXX"

npm run tauri:build
```

**Windows:**
```bash
# Requires code signing certificate
$env:TAURI_PRIVATE_KEY = "path/to/cert.pfx"
$env:TAURI_KEY_PASSWORD = "cert-password"

npm run tauri:build
```

## Next Steps

- Read the [Engineering Spec](engineering-spec.md) for architecture details
- Check [Contributing Guidelines](CONTRIBUTING.md)
- Review [API Documentation](docs/API.md)

## Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/yourusername/clipscribe/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/yourusername/clipscribe/discussions)
- 📧 **Email**: dev@clipscribe.app
