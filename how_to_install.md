# How to Install and Run KERN on a New Computer

## Prerequisite: Install Rust
Since KERN is a compiled language built in Rust, you need the Rust toolchain installed on your new computer.
1.  Go to [rustup.rs](https://rustup.rs/)
2.  Download and run the installer.
3.  Ensure `cargo` is in your system PATH.

## Step 1: Transfer the Code
Copy this entire `KERN` folder to your new computer.

## Step 2: Build the Compiler
Open a terminal in the `KERN` folder and run:

```bash
cargo build --release --bin kernc
```

This will compile the KERN CLI tool (`kernc`).
The executable will be located at: `target/release/kernc.exe` (Windows) or `target/release/kernc` (Mac/Linux).

## Step 3: (Optional) Install VS Code Extension
For syntax highlighting:
1.  Open VS Code.
2.  Copy the `tools/syntax` folder to your `.vscode/extensions` folder (or just use the files manually if you know how).
3.  Alternatively, just open the files in VS Code and it might detect the configuration if you open the root folder.

## Step 4: Add to Path
For convenience, add the `target/release` directory to your system PATH so you can run `kernc` from anywhere.

## Usage
To verify it works:
```bash
kernc --help
```
