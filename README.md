# Hosts Editor

A cross-platform hosts file editor with GUI and backup management capabilities built with Rust and Tauri.

## Features

- 🖥️ **Cross-platform**: Works on Windows, macOS, and Linux
- 🛡️ **Permission Management**: Automatically handles elevation for system file access
- 💾 **Backup System**: Create, restore, and manage named backups of your hosts file
- 🎨 **Modern GUI**: Beautiful, responsive web-based interface
- 📦 **Single Executable**: No dependencies required - just download and run
- ⚡ **Fast & Secure**: Built with Rust for performance and security

## Quick Start

### Prerequisites

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
2. **Node.js** - Install from [nodejs.org](https://nodejs.org/)

### Building from Source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd HostsEditor
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

4. Build for production:
   ```bash
   npm run tauri build
   ```

The built executable will be in `src-tauri/target/release/bundle/`.

## Usage

1. **Launch the application** - The app will check for admin privileges
2. **Edit hosts entries** - Add, remove, enable/disable hosts entries
3. **Create backups** - Give them meaningful names for easy identification
4. **Restore backups** - Switch between different hosts configurations
5. **Save changes** - The app will request elevation if needed

## Hosts File Locations

- **Windows**: `C:\Windows\System32\drivers\etc\hosts`
- **macOS/Linux**: `/etc/hosts`

## Backup Storage

Backups are stored in your home directory under `.hosts-editor/backups/`.

## Security

- The application only requests elevation when needed (saving/restoring)
- Backups are stored in user space, not system directories
- Original hosts file structure and comments are preserved
- Emergency backup is created before any modifications

## Building Single Executable

To create a single executable with no dependencies:

```bash
# For current platform
npm run tauri build

# The executable will be in:
# Windows: src-tauri/target/release/hosts-editor.exe
# macOS: src-tauri/target/release/bundle/macos/Hosts Editor.app
# Linux: src-tauri/target/release/hosts-editor
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Troubleshooting

### Permission Issues
- On Windows: Right-click and "Run as Administrator"
- On macOS/Linux: Run with `sudo` or use the built-in elevation

### Backup Issues
- Check that `~/.hosts-editor/backups/` directory exists and is writable
- Ensure sufficient disk space for backups

### Build Issues
- Ensure Rust and Node.js are properly installed
- Run `npm install` to install dependencies
- Check that all required system libraries are installed (varies by platform)