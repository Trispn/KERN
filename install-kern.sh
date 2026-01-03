#!/bin/bash
# Kern Installation Script for macOS/Linux
# This script downloads and installs the Kern compiler executable

echo "Installing Kern Programming Language..."
echo ""

# Detect OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    BINARY_NAME="kern_compiler-macos"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    BINARY_NAME="kern_compiler-linux"
else
    echo "Unsupported operating system"
    exit 1
fi

# Create installation directory
INSTALL_DIR="$HOME/.local/bin/kern"
mkdir -p "$INSTALL_DIR"
echo "Created directory: $INSTALL_DIR"

# Download the latest executable from GitHub Releases
# Replace 'your-username' with actual GitHub username
GITHUB_URL="https://github.com/your-username/KERN/releases/download/latest/$BINARY_NAME"

echo "Downloading Kern compiler from GitHub..."
curl -L "$GITHUB_URL" -o "$INSTALL_DIR/kern_compiler" 2>/dev/null

if [ -f "$INSTALL_DIR/kern_compiler" ]; then
    chmod +x "$INSTALL_DIR/kern_compiler"
    echo "✓ Downloaded successfully!"
else
    echo "✗ Download failed. Check your internet connection and GitHub URL."
    exit 1
fi

# Add to PATH if not already there
if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
    echo "Already in PATH"
else
    echo "Adding Kern to PATH..."
    if [[ "$SHELL" == *"zsh"* ]]; then
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> ~/.zshrc
        source ~/.zshrc
    else
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> ~/.bashrc
        source ~/.bashrc
    fi
fi

echo ""
echo "============================================"
echo "✓ Kern installed successfully!"
echo "============================================"
echo ""
echo "Installation directory: $INSTALL_DIR"
echo ""
echo "To use Kern:"
echo "1. Close and reopen your terminal"
echo "2. Run: kern_compiler --help"
echo ""
