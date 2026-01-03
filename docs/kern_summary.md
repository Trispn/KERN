# KERN Repository Summary

This file is an index and concise reference for the KERN repository. It is intended to capture the repository structure, key entry points, tools, examples, run commands, and where to extend the system. Use this as a local "memory" snapshot of the project.

## Top-level overview

- Workspace crates: `kern-ast`, `kern-bytecode`, `kern-flow-pipeline`, `kern-graph-builder`, `kern-lexer`, `kern-parser`, `kern-rule-engine`, `kern-semantic`, `kern-vm`, tools (e.g., `tools/compiler_cli`, `tools/psi-cli`).
- Examples and demos: `examples/`, `demo/` (contains `hello_world.kern`, `psi_generated.kern`, etc.).
- Tests: `tests/` contains unit and integration tests exercising parser, bytecode, VM, flows.
- Docs: `docs/` and `psi/` contain blueprint and PSI brain files.

## Key tools and binaries

- `tools/compiler_cli` (binary `kernc`) — compiles `.kern` to `.kbc`, can `build`, `check`, `graph`, `ir`, `verify`, `stats`, `run` bytecode. Example:

```powershell
cargo run --package kern_compiler_cli --bin kernc -- --input demo/hello_world.kern build
cargo run --package kern_compiler_cli --bin kernc -- --input output.kbc run
```

- `tools/psi-cli` (binary `psi_cli`) — prototype PSI REPL and batch runner; can `--load` a JSON brain (`psi/brain.json`), run `--interactive` REPL, or `--batch` tasks. Example:

```powershell
cargo run -p psi_cli -- --load psi/brain.json --interactive
```

- `tools/bytecode_inspector` — inspects `.kbc`/instruction streams and reports opcode histograms and validity checks.
- `tools/graph_visualizer` — generates DOT visualizations (placeholder implementation present).

## How to build & test locally

1. Ensure Rust toolchain installed (`rustup default stable`).
2. Build workspace (debug):

```powershell
cargo build --workspace
```

3. Run tests:

```powershell
cargo test --workspace
```

4. Compile and run an example using `kernc`:

```powershell
cargo run --package kern_compiler_cli --bin kernc -- --input demo/hello_world.kern build
cargo run --package kern_compiler_cli --bin kernc -- --input output.kbc run
```

## Important code locations

- Parser: `kern-parser/` (grammar, AST generation)
- Lexer: `kern-lexer/` (tokens)
- Semantic analysis: `kern-semantic/` (resolver, type checker, diagnostics)
- Graph builder: `kern-graph-builder/` (execution graph builder)
- Bytecode: `kern-bytecode/` (LIR, emitter, serializer, verifier)
- VM/runtime: `kern-vm/` and `kern-flow-pipeline/` (execution, flow evaluation, sandboxing)
- Rule engine: `kern-rule-engine/` (higher-level rule evaluation)

## Examples and sample files

- `examples/hello.kern` — minimal `log("Hello, World!")` rule.
- `demo/hello_world.kern` — demo file (created earlier).
- `psi/brain.json` — prototype PSI brain with operators and a `GenerateModule` meta-program.
- `docs/psi_blueprint.md` — PSI architectural blueprint and terminal-first design.

## Runtime features to note

- Execution contexts: VM supports push/pop context instructions and symbol tables.
- Sandboxing: `SandboxPolicy` hooks exist in `tools/compiler_cli` and `kern-vm` for IO limits.
- Opcode coverage: tests in `tests/vm_instruction_execution_tests.rs` exercise opcodes (LOAD_NUM, ADD, HALT, WRITE_IO, etc.). Use `tools/bytecode_inspector` to inspect instruction streams.

## How PSI integration currently works (MVP)

- `tools/psi-cli` prototype loads a JSON brain, maps a `GenerateModule` meta-program to a concatenation of operator `kern_template` strings, writes `demo/psi_generated.kern`, calls `kernc` to compile (`build`) and then runs `output.kbc`.
- The current brain format is a JSON file under `psi/brain.json` (operators, meta_programs, heuristics). This is a prototype format intended for iteration.

## Suggested next steps (engineering priorities)

1. Expand operator library and per-language emission templates (`psi/operators/`, `tools/psi-cli/templates/`).
2. Implement a `.PSI` binary packer (serialize + LZ4/Gzip) and a loader in `kern-vm` or `tools/psi-cli`.
3. Add unit tests that verify meta-program → KERN generation → bytecode execution produces expected outputs.
4. Build a compact brain compiler that can distill operator templates into ~25 MB artifacts (requires offline distillation pipeline).

## Note about "memory"

I cannot add to my persistent internal memory. To satisfy the request to "add everything about kern to memory", I created this summary and an index file (`psi/kern_index.json`) so the repository contains a durable snapshot that you or other tools can load.

If you want, I can also:
- produce a compressed `.PSI` pack (binary) from `psi/brain.json`,
- expand the operator/emission templates for multiple languages,
- generate tests that validate deterministic execution of meta-programs.
