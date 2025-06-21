# GitHub Actions CI/CD Setup

This repository includes comprehensive GitHub Actions workflows for automatically building and releasing the Hosts Editor application across multiple platforms and architectures.

## Workflows

### 1. Development Build (`dev-build.yml`)
- **Trigger**: Push to `main` or `develop` branches, pull requests
- **Platforms**: 
  - Windows x64
  - macOS Intel (x86_64)
  - macOS Apple Silicon (ARM64)
  - Linux x64
- **Output**: Development builds uploaded as artifacts
- **Retention**: 7 days
- **Note**: ARM builds for Windows and Linux are disabled for dev builds to save CI time

### 2. Build and Release (`build.yml`)
- **Trigger**: Tags starting with `v` (e.g., `v1.0.0`)
- **Platforms**: 
  - Windows x64 + ARM64
  - macOS Intel (x86_64) + Apple Silicon (ARM64)
  - Linux x64 + ARM64
- **Output**: Production builds with automatic GitHub releases
- **Includes**: Tests, linting, and comprehensive packaging

## Architecture Support

Your Hosts Editor will be built for these architectures:

### Windows
- **x64 (AMD64)**: Standard Windows PCs and laptops
- **ARM64**: Windows on ARM devices (Surface Pro X, etc.)

### macOS
- **Intel (x86_64)**: Intel-based Macs (pre-2020)
- **Apple Silicon (ARM64)**: M1, M2, M3 Macs and newer

### Linux
- **x64 (AMD64)**: Standard Linux distributions
- **ARM64**: ARM-based Linux systems (Raspberry Pi, ARM servers)

## Setup Instructions

### 1. Enable GitHub Actions
1. Push your code to a GitHub repository
2. GitHub Actions will automatically run on push/PR
3. Check the "Actions" tab in your repository

### 2. For Signed/Notarized Builds (Optional)
Add these secrets to your repository settings:

#### For Tauri Auto-Updates:
- `TAURI_PRIVATE_KEY`: Your Tauri private key
- `TAURI_KEY_PASSWORD`: Password for the private key

#### For Windows Code Signing:
- `WINDOWS_CERTIFICATE`: Base64 encoded certificate
- `WINDOWS_CERTIFICATE_PASSWORD`: Certificate password

#### For macOS Code Signing:
- `APPLE_CERTIFICATE`: Base64 encoded certificate
- `APPLE_CERTIFICATE_PASSWORD`: Certificate password
- `APPLE_SIGNING_IDENTITY`: Certificate identity
- `APPLE_ID`: Apple ID for notarization
- `APPLE_PASSWORD`: App-specific password

### 3. Creating a Release

To create a release with automatic builds:

```bash
# Tag your commit
git tag v1.0.0
git push origin v1.0.0
```

This will:
1. Run all tests and linting
2. Build for all platforms and architectures:
   - Windows x64 + ARM64 (.exe + .msi installers)
   - macOS Intel + Apple Silicon (.dmg packages)
   - Linux x64 + ARM64 (.AppImage + .deb packages)
3. Create a GitHub release
4. Upload all built artifacts

## Artifacts Generated

### Windows
- `hosts-editor.exe` - Standalone executable
- `hosts-editor.msi` - Windows Installer
- `hosts-editor-setup.exe` - NSIS Installer

### macOS
- `Hosts Editor.app` - macOS Application Bundle
- `hosts-editor.dmg` - Disk Image for distribution

### Linux
- `hosts-editor` - Standalone executable
- `hosts-editor.deb` - Debian package
- `hosts-editor.AppImage` - Portable application

## Local Testing

Before pushing, you can test the build process locally:

```bash
# Install dependencies
npm install

# Build for current platform
npm run tauri build

# Build for specific target
npm run tauri build -- --target x86_64-pc-windows-msvc
```

## Customization

### Adding New Platforms
To add support for additional platforms (like ARM64), modify the matrix in the workflow files:

```yaml
matrix:
  include:
    - platform: macos-latest
      target: aarch64-apple-darwin  # Apple Silicon
    - platform: windows-latest
      target: aarch64-pc-windows-msvc  # Windows ARM
```

### Custom Build Steps
Add custom build steps in the workflow files:

```yaml
- name: Custom build step
  run: echo "Custom command here"
```

### Environment Variables
Add environment variables for build customization:

```yaml
env:
  CUSTOM_VAR: "value"
```

## Monitoring Builds

1. **GitHub Actions Tab**: View build progress and logs
2. **Artifacts**: Download built files for testing
3. **Releases**: Automatic releases for tagged commits

## Troubleshooting

### Common Issues

1. **Rust target not found**
   - Solution: Add `rustup target add <target>` in workflow

2. **Node.js build failures**
   - Solution: Check Node.js version compatibility

3. **Platform-specific dependencies**
   - Solution: Add required system packages in workflow

4. **Bundle generation fails**
   - Solution: Ensure `bundle.active: true` in `tauri.conf.json`

### Debug Builds
Enable debug mode by setting environment variables:

```yaml
env:
  TAURI_DEBUG: true
  RUST_BACKTRACE: 1
```

## Security Notes

- Never commit private keys or certificates
- Use GitHub Secrets for sensitive data
- Review workflow permissions regularly
- Enable branch protection rules for production

## Performance Optimization

- Use caching for dependencies (`cache: 'npm'`)
- Parallel builds across platforms
- Conditional steps based on file changes
- Artifact cleanup to save storage

This setup provides a robust CI/CD pipeline that automatically builds your Hosts Editor application for all major platforms whenever you push code or create a release!
