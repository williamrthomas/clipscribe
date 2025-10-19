# Contributing to ClipScribe

Thank you for considering contributing to ClipScribe! This document provides guidelines and information for contributors.

## Code of Conduct

Be respectful, constructive, and welcoming to all contributors. We're here to build something great together.

## How to Contribute

### Reporting Bugs

**Before submitting a bug report:**
- Check existing issues to avoid duplicates
- Update to the latest version
- Verify the bug is reproducible

**When submitting:**
1. Use a clear, descriptive title
2. Describe the exact steps to reproduce
3. Provide expected vs actual behavior
4. Include system information (OS, Node version, etc.)
5. Add screenshots if applicable

**Bug Report Template:**
```markdown
**Description**
A clear description of the bug.

**Steps to Reproduce**
1. Go to '...'
2. Click on '...'
3. See error

**Expected Behavior**
What you expected to happen.

**Actual Behavior**
What actually happened.

**System Information**
- OS: [e.g., macOS 13.0]
- ClipScribe Version: [e.g., 1.0.0]
- Node Version: [e.g., 18.17.0]

**Screenshots**
If applicable, add screenshots.
```

### Suggesting Features

**Feature Request Template:**
```markdown
**Problem Statement**
What problem does this feature solve?

**Proposed Solution**
How should it work?

**Alternatives Considered**
Other approaches you've thought about.

**Additional Context**
Mockups, examples, or other details.
```

### Pull Requests

**Before submitting:**
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Test thoroughly
5. Commit with clear messages
6. Push to your fork
7. Open a Pull Request

**PR Guidelines:**
- Link related issues
- Describe what changed and why
- Include screenshots for UI changes
- Ensure all tests pass
- Follow the code style guide

**PR Template:**
```markdown
## Description
Brief description of changes.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
How did you test this?

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added where needed
- [ ] Documentation updated
- [ ] No new warnings
- [ ] Tests added/updated
```

## Development Setup

See [SETUP.md](SETUP.md) for detailed setup instructions.

Quick start:
```bash
git clone https://github.com/yourusername/clipscribe.git
cd clipscribe
npm install
# Set up FFmpeg (see SETUP.md)
npm run tauri:dev
```

## Code Style

### TypeScript/React

- Use TypeScript for all new code
- Follow functional components with hooks
- Use meaningful variable names
- Add JSDoc comments for complex functions

**Example:**
```typescript
/**
 * Validates and maps AI-generated clips to VTT timestamps
 * @param clip - Raw clip suggestion from OpenAI
 * @param vttCues - Parsed VTT cues from transcript
 * @returns Validated clip or null if invalid
 */
function validateClip(
  clip: ClipSuggestion,
  vttCues: VttCue[]
): ValidatedClip | null {
  // Implementation
}
```

### Rust

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Use meaningful variable names
- Add doc comments for public functions

**Example:**
```rust
/// Parses a VTT file and extracts all cues
/// 
/// # Arguments
/// * `file_path` - Path to the VTT file
/// 
/// # Returns
/// Vector of parsed VTT cues or error message
/// 
/// # Errors
/// Returns error if file is not valid VTT format
pub fn parse(file_path: &str) -> Result<Vec<VttCue>, String> {
    // Implementation
}
```

### Commit Messages

Follow conventional commits:

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `style:` Code style (formatting, no logic change)
- `refactor:` Code restructuring
- `test:` Adding tests
- `chore:` Maintenance tasks

**Examples:**
```
feat: add keyboard shortcuts for clip selection
fix: resolve timestamp parsing error for edge cases
docs: update FFmpeg setup instructions
refactor: extract validation logic into separate service
```

## Project Structure

```
clipscribe/
├── src/                    # React frontend
│   ├── components/         # Reusable UI components
│   ├── hooks/              # Custom React hooks
│   ├── types/              # TypeScript type definitions
│   └── App.tsx             # Main application component
│
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri command handlers
│   │   ├── models/         # Data structures
│   │   ├── services/       # Business logic
│   │   └── main.rs         # Application entry point
│   └── tauri.conf.json     # Tauri configuration
│
└── docs/                   # Documentation
```

## Testing

### Frontend Tests

```bash
# Run tests (when implemented)
npm test

# Test specific component
npm test -- FileDropZone
```

### Backend Tests

```bash
cd src-tauri
cargo test

# Test specific module
cargo test vtt_parser
```

### Manual Testing

Before submitting PR:
- [ ] Test on your platform (macOS/Windows)
- [ ] Test with various video formats
- [ ] Test with edge case VTT files
- [ ] Test error scenarios
- [ ] Verify UI is responsive

## Areas for Contribution

### High Priority
- **Icons**: Create proper app icons
- **Tests**: Add unit and integration tests
- **Error Handling**: Improve error messages
- **Documentation**: Code comments and guides

### Feature Ideas
- Support for plain text transcripts
- Clip preview before generation
- Batch processing multiple videos
- Custom export settings (resolution, format)
- Alternative AI providers (Claude, etc.)
- Keyboard shortcuts

### Known Issues
Check [GitHub Issues](https://github.com/yourusername/clipscribe/issues) for current bugs and feature requests.

## Release Process

Maintainers will handle releases:

1. Update version in `package.json` and `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag v1.x.x`
4. Build release: `npm run tauri:build`
5. Create GitHub release with binaries
6. Update documentation

## Questions?

- 💬 [GitHub Discussions](https://github.com/yourusername/clipscribe/discussions)
- 📧 Email: dev@clipscribe.app
- 📖 Read the [Engineering Spec](engineering-spec.md)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to ClipScribe! 🎉**
