# KERN Rule Engine

The KERN Rule Engine is a deterministic rule evaluation system that implements the KERN (Knowledge Execution & Reasoning Notation) specification. It provides pattern matching, conflict resolution, priority-based execution, and recursion prevention.

## Architecture

The Rule Engine follows the specification and consists of the following components:

### 1. Rule Engine Core (`rule_engine.rs`)
- Main execution logic
- Rule evaluation and execution
- Program state management
- Integration with all other components

### 2. Pattern Matcher (`pattern_matcher.rs`)
- Pattern matching algorithms
- Structural and value-based matching
- Optimization for repeated evaluations

### 3. Scheduler (`scheduler.rs`)
- Rule execution scheduling
- Priority-based ordering
- Dependency resolution
- Deterministic execution queue management

### 4. Conflict Resolver (`conflict_resolver.rs`)
- Detection of conflicting rules
- Multiple resolution strategies (Ignore, Override, Merge, Error)
- Resolution history tracking

### 5. Priority Manager (`priority_manager.rs`)
- Rule priority assignment
- Priority level management (Lowest, Normal, High, Critical)
- Priority adjustment based on dependencies and complexity

### 6. Recursion Guard (`recursion_guard.rs`)
- Recursion detection and prevention
- Execution count tracking
- Call stack management
- Configurable limits

## Key Features

### Deterministic Execution
- Same input state always produces the same execution order
- Stable sorting with tie-breaking by rule ID
- Explicit recursion limits prevent unbounded execution

### Pattern Matching
- Structural pattern matching over execution graphs
- Variable binding and substitution
- Composite pattern support

### Conflict Resolution
- Multiple conflict resolution strategies
- Automatic detection of attribute write conflicts
- Flow and action conflict handling

### Priority System
- Configurable priority levels
- Dynamic priority adjustment
- Stable priority-based sorting

### Recursion Prevention
- Explicit recursion limits per rule
- Call stack depth monitoring
- Direct and indirect recursion detection

## Usage

```rust
use kern_rule_engine::{RuleEngine, RuleExecutionInfo};
use kern_graph_builder::GraphBuilder;
use kern_parser::Parser;

// Parse KERN code
let mut parser = Parser::new(kern_code);
let program = parser.parse_program()?;

// Build execution graph
let mut builder = GraphBuilder::new();
let graph = builder.build_execution_graph(&program);

// Create rule engine
let mut engine = RuleEngine::new(graph);

// Execute rule evaluation cycle
engine.execute_cycle()?;
```

## Execution Flow

The Rule Engine follows this deterministic execution flow:

1. **Pattern Matching Engine** → Identify applicable rules
2. **Priority Sorting** → Order rules by priority
3. **Conflict Detection & Resolution** → Handle conflicting rules
4. **Rule Execution Scheduler** → Schedule rules for execution
5. **Action Subgraph Execution** → Execute rule actions
6. **Program State Update** → Update execution state

## Determinism Guarantees

- Rule order: priority + stable ID tie-break
- Pattern matches: deterministic graph traversal
- Scheduler: deterministic queue insertion
- Recursion: explicit limits prevent unbounded calls
- Conflicts: deterministic resolution

## File Structure

```
/rule_engine
  lib.rs                 # Module declarations and exports
  rule_engine.rs         # Core rule engine implementation
  pattern_matcher.rs     # Pattern matching algorithms
  scheduler.rs           # Rule execution scheduling
  conflict_resolver.rs   # Conflict detection and resolution
  priority_manager.rs    # Priority management system
  recursion_guard.rs     # Recursion prevention
  implementation.rs      # Main implementation combining all components
  tests.rs               # Comprehensive test suite
```

## Status

All requirements from the specification are implemented:
- ✅ Rule matching algorithms
- ✅ Pattern matching engine
- ✅ Rule priority system
- ✅ Conflict resolution
- ✅ Execution scheduling
- ✅ Recursion prevention with explicit limits