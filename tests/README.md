# KERN Language - Unit Testing & Validation Infrastructure

This directory contains the comprehensive testing infrastructure for the KERN language implementation.

## Overview

The KERN testing infrastructure ensures:
- All grammar productions are correctly parsed
- Error handling works deterministically
- Edge cases in validation and semantic analysis are caught
- Integration with PSI is deterministic and reproducible
- Regression prevention as the system evolves

## Test Organization

The test suite is organized into the following categories:

### 1. Lexer Tests (`lexer_tests.rs`)
- Tests for tokenizing KERN source code
- Verification of all lexical elements
- Edge cases for string, number, and identifier parsing

### 2. Parser Tests (`parser_tests.rs`)
- Tests for parsing tokens into AST nodes
- Verification of all grammar productions
- Error recovery tests

### 3. AST Tests (`ast_tests.rs`)
- Tests for Abstract Syntax Tree node creation
- Verification of AST structure integrity
- Serialization/deserialization tests

### 4. Semantic Tests (`semantic_tests.rs`)
- Tests for symbol table management
- Type checking validation
- Dependency graph construction
- Rule conflict detection

### 5. Rule Engine Tests (`rule_engine_tests.rs`)
- Tests for rule creation and execution
- Priority system validation
- Conflict resolution mechanisms

### 6. Flow Pipeline Tests (`flow_pipeline_tests.rs`)
- Tests for flow execution
- Demand-driven evaluation
- Context propagation
- Control flow operations

### 7. Error Recovery Tests (`error_recovery_tests.rs`)
- Tests for handling invalid syntax
- Recovery after parsing errors
- Graceful degradation

### 8. Edge Case Tests (`edge_case_tests.rs`)
- Tests for boundary conditions
- Maximum nesting scenarios
- Long identifiers and large numbers
- Special character handling

## Running Tests

### All Tests
```bash
cargo test
```

### Specific Test Suite
```bash
cargo test lexer_tests
cargo test parser_tests
cargo test ast_tests
# ... etc
```

### All Tests with Output
```bash
cargo test -- --nocapture
```

## Test Runner

The `main_test_runner.rs` file provides a unified interface for running all tests and generating reports.

## Assertion API

The `assertions.rs` file provides a comprehensive assertion API with functions like:
- `assert_equal(actual, expected, message)`
- `assert_true(condition, message)`
- `assert_false(condition, message)`
- `assert_raises(callable, expected_error, message)`

## CI Integration

The testing infrastructure is integrated with GitHub Actions for continuous integration. See `.github/workflows/test.yml` for the CI configuration.

## Determinism Guarantees

All tests follow these determinism principles:
- Same input → same test outcome
- Tests run in alphabetical order for consistency
- No external dependencies that could cause variation
- Error messages include stable line/column information

## Coverage

The test suite aims for 100% coverage of:
- Grammar productions
- AST node types
- Semantic validation checks
- Flow operations
- Error handling paths