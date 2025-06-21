# Development Guide

## Prerequisites

Before you start, make sure you have the following installed:

1. **Rust** (latest stable version)
   - Install from [rustup.rs](https://rustup.rs/)
   - Run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. **Node.js** (version 16 or higher)
   - Install from [nodejs.org](https://nodejs.org/)

3. **System Dependencies**
   - **Windows**: Visual Studio C++ Build Tools or Visual Studio Community
   - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
   - **Linux**: Build essentials (`sudo apt-get install build-essential`)

## Setup

1. **Clone and setup**:
   ```bash
   git clone <your-repo>
   cd HostsEditor
   npm install
   ```

2. **Install Tauri CLI globally** (optional):
   ```bash
   npm install -g @tauri-apps/cli
   ```

## Development Commands

### Run in Development Mode
```bash
# Using npm script
npm run tauri:dev

# Or if you have Tauri CLI installed globally
tauri dev
```

This will:
- Start the Vite dev server on port 1420
- Launch the Tauri app with hot reload
- Open developer tools automatically

### Build for Production
```bash
# Using npm script
npm run tauri:build

# Or with global CLI
tauri build
```

### Just the Frontend
```bash
# Start only the web development server
npm run dev

# Build only the frontend
npm run build
```

## Project Structure

```
HostsEditor/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs     # Main application entry
│   │   ├── hosts.rs    # Hosts file management
│   │   ├── backup.rs   # Backup system
│   │   └── elevation.rs # Permission handling
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
├── public/             # Static assets
├── index.html          # Main HTML file
├── main.js            # Frontend JavaScript
├── package.json       # Node.js dependencies
└── vite.config.js     # Vite configuration
```

## Building Single Executable

The built executable will have no external dependencies and can be distributed as a single file.

### For Current Platform
```bash
npm run tauri:build
```

**Output locations**:
- **Windows**: `src-tauri/target/release/hosts-editor.exe`
- **macOS**: `src-tauri/target/release/bundle/macos/Hosts Editor.app`
- **Linux**: `src-tauri/target/release/hosts-editor`

### For Different Platforms (Cross-compilation)

You can build for different platforms using Rust's cross-compilation features:

```bash
# Install target
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add x86_64-unknown-linux-gnu

# Build for specific target
tauri build --target x86_64-pc-windows-msvc
```

## Testing

### Test the Backend (Rust)
```bash
cd src-tauri
cargo test
```

### Test Hosts File Operations
Run the app and test:
1. Loading hosts file
2. Adding/editing entries
3. Creating backups
4. Restoring backups
5. Permission elevation

### Test on Different Platforms
- **Windows**: Test UAC elevation
- **macOS**: Test admin authentication
- **Linux**: Test sudo/pkexec elevation

## Debugging

### Enable Debug Mode
```bash
# Set environment variable
export TAURI_DEBUG=true
npm run tauri:dev
```

### Debug the Rust Backend
Add debug prints in Rust code:
```rust
eprintln!("Debug: {}", value);
```

### Debug the Frontend
Use browser developer tools (automatically opened in dev mode).

### Common Issues

1. **Permission errors**: Make sure the app requests elevation properly
2. **File not found**: Check hosts file paths for different OS
3. **Build errors**: Ensure all system dependencies are installed

## Release Process

1. **Update version** in `package.json` and `src-tauri/Cargo.toml`
2. **Test thoroughly** on all target platforms
3. **Build release**:
   ```bash
   npm run tauri:build
   ```
4. **Test the built executable**
5. **Create release notes**
6. **Distribute the executable**

## Security Considerations

- The app only requests elevation when needed (save/restore operations)
- Backups are stored in user space, not system directories
- All file operations are validated and sanitized
- Emergency backups are created before modifications

## Performance Tips

- Use `--release` flag for production builds
- The Rust backend is already optimized for performance
- Frontend uses minimal dependencies for fast loading
