# KERN Development Tools

This directory contains the development tools for the KERN programming language as specified in the development plan.

## Tools Implemented

### 1. Command-Line Compiler (`kernc`)
- Binary: `kernc`
- Purpose: Parse, type-check, compile, emit bytecode
- Commands:
  - `build`: Compile source to bytecode
  - `check`: Parse and validate without output
  - `graph`: Emit execution graph
  - `ir`: Show intermediate representation
  - `verify`: Verify existing bytecode file
  - `stats`: Report symbols, entities, rules

### 2. Debugger (`kerndbg`)
- Binary: `kerndbg`
- Purpose: Execute programs stepwise; inspect VM state
- Commands:
  - `run`/`r`: Start execution
  - `step`/`s`: Execute next instruction
  - `next`/`n`: Execute next instruction
  - `regs`: Show registers
  - `ctx`: Show current context
  - `trace`: Show execution trace
  - `break`/`b`: Set breakpoint
  - `mem`: Inspect memory
  - `help`/`h`: Show help
  - `quit`/`q`: Quit debugger

### 3. Profiler (`kernprof`)
- Binary: `kernprof`
- Purpose: Measure program efficiency (instruction-level)
- Modes:
  - `static`: Analyze bytecode without running
  - `dynamic`: Instrument running VM
  - `compare`: Compare two profiles
  - `trace`: Log time/step data for PSI

### 4. Bytecode Inspector (`kernbc`)
- Binary: `kernbc`
- Purpose: Disassemble and verify compiled bytecode
- Actions:
  - `disassemble`: Disassemble bytecode to human-readable format
  - `verify`: Verify bytecode integrity and validity
  - `meta`: Show bytecode metadata
  - `stats`: Show statistics about the bytecode

### 5. Graph Visualizer (`kerngraph`)
- Binary: `kerngraph`
- Purpose: Render and analyze execution graphs
- Output formats:
  - `dot`: Graphviz DOT format
  - `svg`: Scalable Vector Graphics
  - `json`: JSON representation
  - `png`: PNG image (info only)

### 6. Syntax Highlighting Support
- Files:
  - `kern-syntax.json`: Tree-sitter grammar
  - `kern.vim`: Vim syntax highlighting
  - `kern.language.json`: VS Code syntax highlighting

## Building the Tools

Each tool is a separate Rust crate that can be built with:

```bash
cd tools/<tool_name>
cargo build --release
```

## Usage Examples

### Compile a KERN file:
```bash
kernc -i myprogram.kern build -o myprogram.kbc
```

### Debug a bytecode file:
```bash
kerndbg -i myprogram.kbc
```

### Profile a bytecode file:
```bash
kernprof -i myprogram.kbc -m dynamic -f json -o profile.json
```

### Inspect bytecode:
```bash
kernbc -i myprogram.kbc disassemble
```

### Visualize execution graph:
```bash
kerngraph -i myprogram.kgraph -f svg -o myprogram.svg
```

## Design Principles

All tools follow the KERN design principles:
- Deterministic
- Sandboxed
- Reproducible
- Cross-platform (CLI-first)
- PSI-compatible