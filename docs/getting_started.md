# Getting Started with KERN

## Prerequisites

Before installing KERN, ensure your system meets the following requirements:

### System Requirements
- **Operating System**: Windows 7+, macOS 10.14+, or Linux (any recent distribution)
- **RAM**: Minimum 2GB (4GB recommended for development)
- **Disk Space**: 500MB available space
- **Architecture**: x86_64 (64-bit) or ARM64

### Software Dependencies
- **Rust**: Stable toolchain (1.70 or later) - Required for building from source
- **Git**: Version 2.0 or later - Required for cloning the repository
- **Cargo**: Rust's package manager (installed with Rust)
- **Build Tools**: C/C++ compiler (MSVC on Windows, GCC/Clang on Unix systems)

## Installation Methods

### Method 1: Building from Source (Recommended for Developers)

1. **Install Rust Toolchain**
   ```bash
   # Install rustup (Rust installer and version manager)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Or on Windows, download rustup-init.exe from https://rustup.rs/
   
   # Restart your terminal or run:
   source ~/.cargo/env
   ```

2. **Verify Installation**
   ```bash
   rustc --version
   cargo --version
   ```

3. **Install Required Components**
   ```bash
   rustup default stable
   rustup component add rustfmt clippy
   ```

4. **Clone the KERN Repository**
   ```bash
   git clone https://github.com/your-org/kern.git
   cd kern
   ```

5. **Build KERN**
   ```bash
   # Debug build (faster compilation, slower execution)
   cargo build --workspace
   
   # Release build (slower compilation, faster execution)
   cargo build --workspace --release
   ```

6. **Run Tests to Verify Installation**
   ```bash
   cargo test --workspace
   ```

### Method 2: Using Pre-built Binaries (Coming Soon)

Currently, KERN is only available as source code. Pre-built binaries will be available in Phase 4 of development.

### Method 3: Docker Installation (Alternative)

If you prefer using Docker for isolation:

1. **Install Docker** from https://docs.docker.com/get-docker/

2. **Build the Docker Image**
   ```bash
   # From the KERN repository root
   docker build -t kern-dev .
   ```

3. **Run KERN in a Container**
   ```bash
   docker run -it --rm -v $(pwd):/workspace kern-dev
   ```

## Setting Up Your Development Environment

### IDE Configuration

KERN development works best with Rust-aware IDEs:

#### Visual Studio Code
1. Install the "rust-analyzer" extension
2. Install "CodeLLDB" for debugging support
3. Configure settings in `.vscode/settings.json`:
   ```json
   {
     "rust-analyzer.cargo.loadOutDirsFromCheck": true,
     "rust-analyzer.procMacro.enable": true
   }
   ```

#### IntelliJ IDEA / CLion
1. Install the "Rust" plugin
2. Enable "org.rust.cargo.evaluate.build.scripts" in experimental features
3. Configure toolchain path to your Rust installation

#### Vim/Neovim
1. Install rust.vim plugin
2. Configure with rust-analyzer LSP

### Environment Variables

Set these environment variables for optimal development experience:

```bash
# Enable debug logging for KERN components
export RUST_LOG=debug

# Set KERN workspace directory
export KERN_WORKSPACE=/path/to/your/kern/project

# Enable performance profiling (optional)
export KERN_PROFILE=1
```

## Your First KERN Program

Let's create and run a simple KERN program to verify everything works correctly.

### Step 1: Create a New KERN File

Create a file named `hello.kern`:

```kern
entity Message {
    text
    timestamp
}

rule HelloWorld:
    if 1 == 1
    then print("Hello, KERN World!")

flow GreetingFlow {
    HelloWorld
}
```

### Step 2: Compile and Run

If you have a KERN runner binary available:

```bash
# Run the program
cargo run --release --bin kern-runner -- hello.kern
```

If no runner binary exists, you can run a test that executes example files:

```bash
# Run tests that include example execution
cargo test --workspace -- run_examples
```

### Step 3: Expected Output

You should see output similar to:

```
[INFO] Loading KERN program: hello.kern
[INFO] Parsing program...
[INFO] Building execution graph...
[INFO] Executing rules...
Hello, KERN World!
[INFO] Execution completed successfully
```

## Understanding the KERN Project Structure

When you clone the KERN repository, you'll see this structure:

```
kern/
├── Cargo.toml              # Workspace configuration
├── README.md              # Project overview
├── docs/                  # Documentation (this guide)
├── examples/              # Example KERN programs
├── src/                   # Top-level source files
├── kern-lexer/            # Tokenization component
├── kern-parser/           # AST generation component
├── kern-ast/              # Abstract syntax tree definitions
├── kern-semantic/         # Semantic analysis component
├── kern-graph-builder/    # Execution graph construction
├── kern-rule-engine/      # Rule execution engine
├── kern-bytecode/         # Bytecode generation
├── kern-vm/               # Virtual machine implementation
└── tests/                 # Integration and unit tests
```

### Key Components Explained

- **kern-lexer**: Converts source code into tokens
- **kern-parser**: Creates AST from tokens following KERN grammar
- **kern-graph-builder**: Transforms AST into execution graphs
- **kern-rule-engine**: Executes rule-based logic
- **kern-bytecode**: Compiles graphs to efficient bytecode
- **kern-vm**: Executes bytecode in register-based environment

## Running Examples and Tests

### Running All Tests

```bash
# Run all tests in the workspace
cargo test --workspace

# Run tests with more verbose output
cargo test --workspace -- --nocapture

# Run tests for a specific component
cargo test -p kern-parser
```

### Running Examples

KERN includes several example programs in the `examples/` directory:

```bash
# List available examples
ls examples/

# Run a specific example (if runner is available)
cargo run --release --bin kern-runner -- examples/basic_rules.kern
```

### Building Documentation

KERN uses Rust documentation comments. Build them with:

```bash
# Build documentation for all crates
cargo doc --workspace --no-deps

# Open documentation in browser
cargo doc --workspace --no-deps --open
```

## Troubleshooting Common Issues

### Issue: "command not found: cargo"
**Solution**: Rust is not installed or not in your PATH. Follow the Rust installation steps above.

### Issue: "linker error" on Windows
**Solution**: Install Visual Studio Build Tools or use the GNU toolchain:
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### Issue: "memory allocation failed" during build
**Solution**: Ensure you have sufficient RAM (at least 2GB free) and disk space. Try building with fewer parallel jobs:
```bash
cargo build --workspace -j1
```

### Issue: Tests failing with timeout
**Solution**: Some tests may take longer on slower systems. Run with increased timeout:
```bash
cargo test --workspace -- --timeout=120
```

## Verifying Your Installation

Run this command to verify your KERN development environment is properly set up:

```bash
# Check that all components build successfully
cargo check --workspace

# Run a quick test to ensure everything works
cargo test --package kern-lexer --lib -- lexer_tests --exact
```

## Next Steps

Congratulations! You now have KERN installed and running. Here's what to explore next:

1. **Language Reference**: Learn about KERN's syntax and constructs
2. **Usage Guide**: Understand how to write, compile, and run KERN programs
3. **Examples & Tutorials**: Work through practical examples
4. **APIs & Integration**: Learn how to embed KERN in your applications

## Quick Reference Commands

```bash
# Build the entire workspace
cargo build --workspace

# Build in release mode
cargo build --workspace --release

# Run all tests
cargo test --workspace

# Check code formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace

# Build documentation
cargo doc --workspace --no-deps

# Clean build artifacts
cargo clean
```

Your KERN development environment is now ready for use. The next section will guide you through writing your first real KERN programs and understanding the language in depth.