# Kern Installation Guide

## Quick Install (Recommended)

### Windows
```bash
# Download and run the installer
curl -O https://raw.githubusercontent.com/your-username/KERN/main/install-kern.bat
install-kern.bat
```

### macOS/Linux
```bash
# Download and run the installer
curl -O https://raw.githubusercontent.com/your-username/KERN/main/install-kern.sh
chmod +x install-kern.sh
./install-kern.sh
```

## Verify Installation

After installation, verify it works:
```bash
kern_compiler --help
```

## Manual Installation

If the automated installer doesn't work, follow these steps:

### Option 1: Pre-compiled Executable (Easiest)
1. Download the executable from [GitHub Releases](https://github.com/your-username/KERN/releases)
   - **Windows**: `kern_compiler.exe`
   - **macOS**: `kern_compiler-macos`
   - **Linux**: `kern_compiler-linux`

2. Move to a directory in your PATH:
   ```bash
   # Windows (PowerShell)
   Move-Item -Path ".\kern_compiler.exe" -Destination "C:\Program Files\Kern\"
   
   # macOS/Linux
   sudo mv kern_compiler-* /usr/local/bin/kern_compiler
   chmod +x /usr/local/bin/kern_compiler
   ```

3. Add to PATH if needed:
   - **Windows**: Add `C:\Program Files\Kern` to your system PATH
   - **macOS/Linux**: Already in PATH if moved to `/usr/local/bin`

### Option 2: Build from Source
Requires Rust (https://rustup.rs/)

```bash
git clone https://github.com/your-username/KERN.git
cd KERN
cargo build --release --bin kern_compiler
# Executable at: target/release/kern_compiler.exe (Windows) or target/release/kern_compiler (Mac/Linux)
```

## First Program

Create a file `hello.kern`:
```kern
rule HelloWorld {
  emit Message(text: "Hello, Kern!")
}
```

Run it:
```bash
kern_compiler hello.kern
```

## Documentation

- [KERN Language Specification](KERN_LANGUAGE.md)
- [Examples](examples/)
- [API Documentation](docs/)

## Troubleshooting

**"Command not found: kern_compiler"**
- Make sure the directory containing `kern_compiler` is in your PATH
- Restart your terminal after installation

**"Permission denied" (macOS/Linux)**
```bash
chmod +x /path/to/kern_compiler
```

**Build errors**
- Ensure Rust is installed: `rustup --version`
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build --release`

## Support

For issues or questions:
- [GitHub Issues](https://github.com/your-username/KERN/issues)
- [Documentation](docs/)
