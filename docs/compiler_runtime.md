# KERN Compiler & Runtime

## Compiler Architecture

The KERN compiler follows a multi-phase compilation pipeline that transforms human-readable KERN source code into efficient bytecode for execution. Each phase maintains the deterministic and analyzable properties that are fundamental to KERN's design.

### Compilation Pipeline Overview

The compilation process consists of five distinct phases:

1. **Lexical Analysis** - Source code → Tokens
2. **Syntax Analysis** - Tokens → Abstract Syntax Tree (AST)
3. **Semantic Analysis** - AST → Validated AST with type information
4. **Graph Building** - Validated AST → Execution Graph
5. **Bytecode Generation** - Execution Graph → Bytecode

Each phase is implemented as a separate crate in the KERN workspace, allowing for modular development and testing.

### Phase 1: Lexical Analysis (kern-lexer)

The lexer converts raw source code into a stream of tokens following the formal grammar specification.

#### Token Types
- **Keywords**: `entity`, `rule`, `flow`, `constraint`, `if`, `then`, `else`, `and`, `or`, `loop`, `break`, `halt`, `true`, `false`
- **Identifiers**: Variable and entity names
- **Literals**: Integer numbers, string literals
- **Operators**: `==`, `!=`, `>`, `<`, `>=`, `<=`, `{`, `}`, `:`, `,`, `(`, `)`
- **Whitespace**: Spaces, tabs, newlines (generally ignored except for line tracking)

#### Error Handling
The lexer provides detailed error reporting:
- Source location tracking (file, line, column)
- Specific error messages for invalid characters
- Recovery mechanisms to continue parsing after errors

#### Example Tokenization
Input: `rule Validate: if x > 5 then action()`
Output: `[RULE, IDENTIFIER("Validate"), COLON, IF, IDENTIFIER("x"), GT, NUMBER(5), THEN, IDENTIFIER("action"), LPAREN, RPAREN]`

### Phase 2: Syntax Analysis (kern-parser)

The parser generates an Abstract Syntax Tree (AST) from the token stream according to the formal grammar.

#### AST Node Types
- **Program**: Root node containing all definitions
- **EntityDef**: Entity definition with name and fields
- **RuleDef**: Rule definition with name, condition, and actions
- **FlowDef**: Flow definition with ordered actions
- **ConstraintDef**: Constraint definition with condition
- **Condition**: Boolean expression with operators
- **Action**: Function call, assignment, or control action

#### Parser Implementation
The parser uses a recursive descent approach:
- Predictive parsing based on grammar productions
- Left-recursive grammar elimination
- Operator precedence handling
- Error recovery with synchronization tokens

#### Error Recovery
- Synchronization points after semicolons or braces
- Local error recovery to continue parsing
- Detailed error messages with expected token information

### Phase 3: Semantic Analysis (kern-semantic)

The semantic analyzer performs type checking, symbol resolution, and validation on the AST.

#### Symbol Table Management
- Global symbol table for entities, rules, flows, and constraints
- Local symbol tables for rule conditions and actions
- Scope management for nested constructs
- Duplicate symbol detection

#### Type Checking
- Variable type inference and checking
- Function call argument validation
- Expression type compatibility
- Constraint condition validation

#### Validation Rules
- All referenced symbols must be defined
- No circular dependencies between rules
- Constraints must evaluate to boolean
- Flow actions must reference valid rules or functions

#### Error Reporting
- Detailed error messages with source locations
- Suggestion for common mistakes
- Multiple error detection in single pass

### Phase 4: Graph Building (kern-graph-builder)

The graph builder transforms the validated AST into an execution graph representation.

#### Execution Graph Structure
The execution graph consists of nodes and edges:
- **Nodes**: Operations, rules, conditions, control flow
- **Edges**: Data flow, control flow, conditional routing

#### Node Types
- **NODE_OP**: Bytecode operations (LOAD_SYM, COMPARE, etc.)
- **NODE_RULE**: Rule evaluation and firing
- **NODE_CONTROL**: If/then/loop control structures
- **NODE_GRAPH**: Graph manipulation operations
- **NODE_IO**: External interface operations

#### Graph Construction Process
1. **Node Allocation**: Create nodes for each operation
2. **Edge Wiring**: Connect nodes based on data and control flow
3. **Validation**: Verify graph properties and constraints
4. **Freeze**: Make graph immutable for execution

#### Optimization Opportunities
- Dead code elimination
- Constant folding
- Rule dependency analysis
- Flow optimization

### Phase 5: Bytecode Generation (kern-bytecode)

The bytecode generator converts the execution graph into fixed-width 8-byte instructions.

#### Instruction Format
```
| OPCODE (1B) | ARG1 (2B) | ARG2 (2B) | ARG3 (2B) | FLAGS (1B) |
```

#### Instruction Categories

##### Control Flow Instructions
- **NOP** (0x00): No operation
- **JMP** (0x01): Unconditional jump
- **JMP_IF** (0x02): Conditional jump
- **HALT** (0x03): Stop execution

##### Data & Symbol Instructions
- **LOAD_SYM** (0x10): Load symbol into register
- **LOAD_NUM** (0x11): Load integer constant
- **MOVE** (0x12): Copy register value
- **COMPARE** (0x13): Compare two registers

##### Graph Instructions
- **GRAPH_NODE_CREATE** (0x20): Create graph node
- **GRAPH_EDGE_CREATE** (0x21): Create graph edge
- **GRAPH_MATCH** (0x22): Pattern matching over graph
- **GRAPH_TRAVERSE** (0x23): Traverse graph via rule

##### Rule Execution Instructions
- **RULE_LOAD** (0x30): Load rule into execution scope
- **RULE_EVAL** (0x31): Evaluate current rule
- **RULE_FIRE** (0x32): Execute rule actions
- **RULE_PRIORITY_SET** (0x33): Set rule priority

##### Context & State Instructions
- **CTX_CREATE** (0x40): Create new execution context
- **CTX_SWITCH** (0x41): Switch active context
- **CTX_CLONE** (0x42): Clone a context
- **CTX_DESTROY** (0x43): Destroy a context

##### Error Handling Instructions
- **ERR_SET** (0x50): Set error register
- **ERR_CLEAR** (0x51): Clear error
- **ERR_CHECK** (0x52): Jump if error exists

##### External Interface Instructions
- **EXT_CALL** (0x60): Call external function
- **EXT_BIND** (0x61): Bind symbol to external adapter

##### Termination & Output Instructions
- **RETURN** (0x70): Return value to caller
- **OUTPUT** (0x71): Emit output to host

## Runtime Architecture

The KERN runtime executes bytecode in a register-based virtual machine designed for deterministic execution.

### Virtual Machine Design

#### Register-Based Architecture
The VM uses explicit registers instead of a stack:
- **R0-R15**: General symbolic registers (16 total)
- **CTX**: Current execution context register
- **ERR**: Error register
- **PC**: Program counter
- **FLAG**: Condition flags

#### Memory Model
- **Stack**: No traditional call stack (execution is graph-based)
- **Heap**: Managed by context allocation
- **Code Segment**: Immutable bytecode
- **Data Segment**: Execution contexts and state

### Execution Process

#### Program Loading
1. Bytecode is loaded into the VM's code segment
2. Initial execution context is created
3. Program counter is set to entry point
4. Registers are initialized

#### Instruction Execution
1. Fetch instruction at PC
2. Decode opcode and arguments
3. Execute operation
4. Update PC and registers
5. Repeat until HALT instruction

#### Context Management
- Contexts isolate execution state
- Context switching is explicit via CTX_SWITCH
- Context cloning copies register state
- No shared mutable state between contexts

### Runtime Safety Features

#### Memory Safety
- Bounds checking for array access
- Context isolation prevents interference
- Automatic memory management
- No pointer arithmetic

#### Execution Safety
- Bounded execution time
- Memory usage limits
- No infinite loops without explicit limits
- Deterministic execution paths

#### Type Safety
- Register type checking
- Instruction argument validation
- Type-safe operations only
- Compile-time type verification

## Compiler Toolchain

### Build Process

The KERN compiler is built using Cargo, Rust's package manager:

```bash
# Build debug version
cargo build --workspace

# Build release version
cargo build --workspace --release

# Build specific component
cargo build -p kern-compiler
```

### Compilation Flags

#### Debug Flags
- `--debug`: Enable debug information and tracing
- `--verbose`: Show detailed compilation steps
- `--trace`: Enable execution tracing

#### Optimization Flags
- `--release`: Enable optimizations
- `--optimize-level <n>`: Set optimization level (0-3)
- `--size-opt`: Optimize for code size

#### Output Flags
- `--format <type>`: Set output format (bytecode, graph, ast)
- `--output <file>`: Specify output file
- `--emit <type>`: Specify what to emit (asm, obj, exe)

### Integration with Build Systems

#### Cargo Integration
KERN can be integrated into Cargo build systems:

```toml
[build-dependencies]
kern-compiler = { path = "../kern-compiler" }

[dependencies]
kern-vm = { path = "../kern-vm" }
```

#### Custom Build Scripts
Create custom build scripts to compile KERN files:

```rust
// build.rs
use kern_compiler::Compiler;

fn main() {
    let mut compiler = Compiler::new();
    
    // Compile all .kern files in src/
    for kern_file in std::fs::read_dir("src").unwrap() {
        let path = kern_file.unwrap().path();
        if path.extension().unwrap_or_default() == "kern" {
            let source = std::fs::read_to_string(&path).unwrap();
            let bytecode = compiler.compile(&source).unwrap();
            
            // Write bytecode to target directory
            let output_path = format!("target/{}.kbc", path.file_stem().unwrap().to_string_lossy());
            std::fs::write(output_path, bytecode).unwrap();
        }
    }
    
    println!("cargo:rerun-if-changed=src/");
}
```

## Performance Characteristics

### Compilation Performance
- Fast compilation due to simple grammar
- Single-pass compilation where possible
- Efficient memory usage during compilation
- Parallel compilation support for large projects

### Runtime Performance
- Near-native execution speed
- Efficient register-based operations
- Minimal runtime overhead
- Predictable performance characteristics

### Memory Usage
- Bounded memory usage
- Context-based memory management
- No memory leaks in safe operations
- Configurable memory limits

## Debugging and Profiling

### Debugging Features

#### Source Mapping
- Bytecode to source location mapping
- Debug information in bytecode
- Step-through debugging support

#### Execution Tracing
- Detailed execution logs
- Rule firing information
- Context state snapshots
- Performance metrics

#### Graph Visualization
- Execution graph visualization
- Rule dependency graphs
- Flow visualization
- Memory layout visualization

### Profiling Tools

#### Built-in Profiler
```bash
# Enable profiling
cargo run --bin kern-runner -- --profile my_program.kern

# Generate profiling report
cargo run --bin kern-profiler -- --input profile_data.json
```

#### Performance Metrics
- Execution time per rule
- Memory usage patterns
- Context switching frequency
- Instruction count breakdown

## Compiler Extensions

### Adding New Instructions

To add a new bytecode instruction:

1. **Define the opcode** in the instruction set
2. **Implement the operation** in the VM
3. **Add compiler support** for the new operation
4. **Update documentation** and tests

Example:
```rust
// In bytecode generator
pub const NEW_INSTRUCTION: u8 = 0x80;

// In VM implementation
match instruction.opcode {
    0x80 => execute_new_instruction(&mut self, instruction),
    // ... other cases
}
```

### Custom Optimizations

The compiler supports custom optimization passes:

```rust
use kern_bytecode::optimizer::{Optimizer, OptimizationPass};

struct MyOptimizationPass;

impl OptimizationPass for MyOptimizationPass {
    fn optimize(&self, graph: &mut ExecutionGraph) -> bool {
        // Perform optimization
        // Return true if changes were made
        false
    }
}

// Register the optimization
let mut optimizer = Optimizer::new();
optimizer.add_pass(Box::new(MyOptimizationPass));
```

## Security Considerations

### Sandboxing
- Isolated execution contexts
- No access to host system without explicit permission
- Memory bounds checking
- No dynamic code generation

### Capability-Based Security
- Explicit permissions for external access
- Capability tokens for sensitive operations
- Principle of least privilege
- Auditable security model

### Input Validation
- Comprehensive input validation
- Sanitization of external data
- Protection against injection attacks
- Type-safe operations only

This comprehensive overview covers the KERN compiler and runtime architecture, providing the technical details needed for understanding how KERN programs are transformed from source code to executable bytecode and executed in a safe, deterministic environment.