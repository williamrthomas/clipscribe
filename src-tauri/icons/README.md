# App Icons

This folder should contain the application icons for ClipScribe.

## Required Icons

Generate icons in the following sizes:

- `32x32.png` - Small icon
- `128x128.png` - Medium icon  
- `128x128@2x.png` - High DPI medium icon
- `icon.icns` - macOS icon bundle
- `icon.ico` - Windows icon file

## Generating Icons

You can use the Tauri icon generator:

```bash
npm install -g @tauri-apps/cli
tauri icon path/to/your-icon.png
```

This command will automatically generate all required icon sizes and formats.

## Design Guidelines

**Recommended:**
- Start with a 1024x1024px PNG file
- Use simple, recognizable imagery
- Ensure good contrast for light/dark modes
- Test at small sizes (32x32) for clarity

**ClipScribe Theme:**
- Main colors: Blue (#2563eb) and white
- Icon concept: Sparkles or video clip symbol
- Keep it professional and modern

## Temporary Placeholder

Until custom icons are created, Tauri will use default icons. The app will still function normally.

## Tools

- **Figma** - Icon design
- **Photoshop/Sketch** - Raster editing
- **IconJar** - Icon management
- **@tauri-apps/cli** - Icon generation
