# KERN Programming Language Implementation

This project implements the KERN (Knowledge Execution & Reasoning Notation) programming language as specified in the development plan.

## Overview

KERN is a deterministic intelligence execution language designed to encode business logic, rules, and workflows compactly. It's a logic-centric, graph-native execution language optimized for machine reasoning rather than human comfort.

## Architecture

The KERN implementation follows a multi-phase compilation and execution pipeline:

1. **Lexer**: Tokenizes KERN source code
2. **Parser**: Generates Abstract Syntax Tree (AST) from tokens
3. **Graph Builder**: Converts AST to execution graph
4. **Rule Engine**: Executes rule-based logic
5. **Bytecode Compiler**: Translates execution graph to bytecode
6. **Virtual Machine**: Executes KERN bytecode

## Components

### kern-lexer
- Tokenizes KERN source code
- Implements comprehensive error reporting
- Handles all lexical elements as per KERN specification

### kern-parser
- Recursive descent parser
- Generates AST from tokens
- Implements all grammar productions (entities, rules, flows, constraints)
- Comprehensive error reporting

### kern-graph-builder
- Converts AST to execution graph
- Implements graph data structures as specified
- Handles all node types (Op, Rule, Control, Graph, Io)

### kern-rule-engine
- Executes rules based on execution graph
- Implements context management
- Handles rule evaluation and firing

### kern-bytecode
- Implements KERN bytecode instruction set
- Fixed-width 8-byte instructions
- Bytecode compiler that translates execution graphs to bytecode
- Serialization/deserialization of bytecode

### kern-vm
- Virtual machine for executing KERN bytecode
- Register-based execution model (R0-R15, CTX, ERR, PC, FLAG)
- Implements all KERN opcodes
- Memory management and context switching

## Features

- **Deterministic Execution**: Same input always produces same output
- **Graph-Based Execution**: All programs compile to execution graphs
- **Rule-Based Logic**: First-class support for rule definitions and execution
- **Static Validation**: Comprehensive validation at compile time
- **Error Handling**: Comprehensive error reporting at all levels
- **Bytecode Execution**: Efficient bytecode-based execution model

## Usage Example

```rust
use kern_parser::Parser;
use kern_graph_builder::GraphBuilder;
use kern_bytecode::BytecodeCompiler;
use kern_vm::VirtualMachine;

// Parse KERN code
let mut parser = Parser::new(kern_code);
let program = parser.parse_program()?;

// Build execution graph
let mut graph_builder = GraphBuilder::new();
let graph = graph_builder.build_execution_graph(&program);

// Compile to bytecode
let mut compiler = BytecodeCompiler::new();
let bytecode = compiler.compile_graph(&graph);

// Execute in VM
let mut vm = VirtualMachine::new();
vm.load_program(bytecode);
vm.execute()?;
```

## KERN Language Constructs

### Entities
```kern
entity Farmer {
    id
    location
    produce
}
```

### Rules
```kern
rule ValidateFarmer:
    if farmer.id != 0 and farmer.location == "valid"
    then approve_farmer(farmer)
```

### Flows
```kern
flow ProcessFarmers {
    load_farmers()
    validate_farmers()
}
```

### Constraints
```kern
constraint ValidId: farmer.id > 0
```

## Bytecode Instructions

The KERN VM supports the following instruction categories:

- Control Flow: NOP, JMP, JMP_IF, HALT
- Data & Symbol: LOAD_SYM, LOAD_NUM, MOVE, COMPARE
- Graph Operations: GRAPH_NODE_CREATE, GRAPH_EDGE_CREATE, GRAPH_MATCH, GRAPH_TRAVERSE
- Rule Execution: RULE_LOAD, RULE_EVAL, RULE_FIRE, RULE_PRIORITY_SET
- Context & State: CTX_CREATE, CTX_SWITCH, CTX_CLONE, CTX_DESTROY
- Error Handling: ERR_SET, ERR_CLEAR, ERR_CHECK
- External Interface: EXT_CALL, EXT_BIND
- Termination: RETURN, OUTPUT

## Development Phases Completed

1. ✅ **Phase 1**: Foundation (Grammar, AST, Static Validation)
2. ✅ **Phase 2**: Rule Engine and Graph Builder
3. ✅ **Phase 3**: Virtual Machine and Bytecode
4. **Phase 4**: Tooling and Ecosystem (Future)

## Design Principles

- **Determinism**: Same input → same output, no randomness, no hidden state
- **Minimalism**: Fewer primitives > expressive syntax
- **Explicit Logic**: No implicit behavior, all control flow visible
- **PSI-First Design**: Optimized for machine reasoning, not human comfort

## Performance Targets

- Startup time: < 10ms
- Execution: Near-native performance
- Memory: Bounded and predictable
- Compilation: Fast and deterministic

## Security Principles

- No dynamic eval
- No hidden IO
- Explicit permissions
- Sandboxed execution
- Memory limits enforced