# KERN Language Reference - Flows

## 6.1 Definition

A flow defines an ordered sequence of operations that execute as an execution pipeline. Flows provide explicit control over execution order and enable complex multi-step processes.

## 6.2 Syntax

```
flow_def = 'flow' identifier '{' { flow_step } '}' ;
flow_step = expression [ '->' expression ] ;
```

## 6.3 Semantics

Flows define:
- An ordered sequence of operations
- Explicit control flow between steps
- Data transformation pipelines
- Synchronous execution of steps
- Error handling and recovery paths

## 6.4 Examples

### 6.4.1 Basic Flow
```
flow UserRegistration {
  validate_input -> create_user -> send_welcome_email
}
```

### 6.4.2 Flow with Data Transformation
```
flow ProcessPayment {
  validate_payment_data;
  calculate_fees -> adjusted_amount;
  process_transaction -> transaction_result;
  log_transaction
}
```

### 6.4.3 Flow with Conditional Steps
```
flow DataProcessing {
  load_data -> raw_data;
  clean_data(raw_data) -> clean_data;
  if validate_data(clean_data) then {
    transform_data(clean_data) -> processed_data;
    store_data(processed_data)
  }
}
```

## 6.5 Execution Guarantees

- Flow steps execute in defined order
- Each step completes before the next begins
- Data dependencies are explicitly defined
- Flow execution is deterministic
- Error propagation follows explicit paths
- Flows can be paused and resumed deterministically

## 6.6 Error Conditions

| Error Code | Condition | Recoverable |
|------------|-----------|-------------|
| SYN004 | Invalid flow step syntax | No |
| SEM006 | Circular flow dependencies | No |
| SEM007 | Undefined step references | No |
| VM002 | Step execution timeout | Yes |
| VM003 | Data dependency failure | Yes |

## 6.7 Bytecode Mapping

Flows are compiled to:
- Sequential instruction sequences
- Explicit control flow instructions (JMP, JMP_IF)
- Data dependency tracking
- Context management for state
- Error handling code for each step

## 6.8 Flow Control Operations

Flows support:
- Sequential execution
- Conditional execution (if/then)
- Data transformation
- Context switching
- Error handling

## 6.9 PSI Observability

Flows are observable as:
- Execution pipelines in the PSI knowledge base
- Sequential operation patterns
- Data transformation chains
- Control flow structures
- Error handling paths
- Performance characteristics