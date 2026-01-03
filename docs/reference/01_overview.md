# KERN Language Reference - Overview

## 1.1 Purpose

The KERN Language Reference is the **single source of truth** for the Knowledge Execution & Reasoning Notation language. This document defines:

- Syntax and formal grammar
- Semantic meaning of all constructs
- Execution behavior guarantees
- Error conditions and handling
- Bytecode translation rules
- PSI observability characteristics

## 1.2 Core Principles

KERN is designed around four core principles:

1. **Determinism**: Same input → same output, no randomness, no hidden state
2. **Minimalism**: Fewer primitives > expressive syntax
3. **Explicit Logic**: No implicit behavior
4. **PSI-First Design**: Optimized for machine reasoning, not human comfort

## 1.3 Language Structure

KERN is a logic-centric, graph-native execution language with the following structural constructs:

- **Entity**: Data model definition
- **Rule**: Inference logic
- **Flow**: Execution pipeline
- **Constraint**: Validation logic

## 1.4 Execution Model

KERN uses a graph-based execution model with:
- Fixed-width 8-byte bytecode instructions
- Register-based execution (R0-R15, CTX, ERR, PC, FLAG)
- Deterministic execution without call stack
- Explicit nodes and edges for data/control flow

## 1.5 Language Primitives

KERN supports these primitive types:
- `sym`: Symbolic identifier
- `num`: Integer value
- `bool`: Boolean (true/false)
- `vec`: Small fixed-size vector
- `ref`: External reference
- `ctx`: Execution context

## 1.6 Document Conventions

In this reference:
- `EBNF` notation defines formal grammar
- `UPPERCASE` indicates terminals
- `lowercase` indicates non-terminals
- `?` indicates optional elements
- `*` indicates zero or more repetitions
- `+` indicates one or more repetitions