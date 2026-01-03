# KERN Language Reference - Control Flow

## 9.1 Definition

Control flow constructs in KERN provide explicit mechanisms for directing execution flow, including conditionals, loops, and context switching.

## 9.2 Supported Constructs

### 9.2.1 Conditional Execution
```
if expression then
  action
[ else
  alternative_action ]
```

### 9.2.2 Sequential Execution
```
action1;
action2;
action3
```

### 9.2.3 Context Switching
```
with context_identifier {
  actions
}
```

## 9.3 Syntax

```
control_action = if_action | sequence_action | context_action ;
if_action = 'if' expression 'then' action [ 'else' action ] ;
sequence_action = action { ';' action } ;
context_action = 'with' identifier '{' { action } '}' ;
```

## 9.4 Semantics

Control flow constructs define:
- Explicit execution paths
- Conditional execution based on boolean expressions
- Sequential execution order
- Context boundaries for variable scoping
- Deterministic execution behavior

## 9.5 Examples

### 9.5.1 Simple Conditional
```
rule ConditionalProcessing {
  if data.valid == true then
    process_data(data)
  else
    log_error(data.id)
}
```

### 9.5.2 Nested Conditionals
```
rule ComplexDecision {
  if user.authenticated == true then {
    if user.role == "admin" then
      grant_admin_access()
    else
      grant_user_access()
  } else
    redirect_to_login()
}
```

### 9.5.3 Sequential Actions
```
flow DataPipeline {
  load_data -> raw_data;
  clean_data(raw_data) -> clean_data;
  validate_data(clean_data) -> validated_data;
  store_data(validated_data)
}
```

### 9.5.4 Context Switching
```
rule ContextualProcessing {
  if needs_special_processing(user) then
    with special_context {
      special_process(user);
      update_context_state()
    }
}
```

## 9.6 Execution Guarantees

- Control flow executes deterministically
- Conditionals evaluate exactly once per rule firing
- Sequences execute in defined order
- Context switches are explicit and tracked
- No implicit control flow changes

## 9.7 Error Conditions

| Error Code | Condition | Recoverable |
|------------|-----------|-------------|
| CTL001 | Invalid conditional expression | No |
| CTL002 | Undefined context reference | No |
| CTL003 | Infinite loop detection | No |
| CTL004 | Context switch failure | Yes |

## 9.8 Bytecode Mapping

Control flow maps to bytecode instructions:
- `if` → COMPARE + JMP_IF instructions
- Sequential actions → Direct instruction sequence
- Context switching → CTX_SWITCH instructions
- Loops → Conditional jumps with loop detection

## 9.9 Loop Detection

KERN implementations must detect and prevent infinite loops:
- Static analysis where possible
- Runtime loop counters
- Explicit loop bounds where applicable
- Deterministic termination guarantees

## 9.10 PSI Observability

Control flow is observable as:
- Execution paths in the PSI knowledge base
- Conditional logic for reasoning
- Sequential operation patterns
- Context boundaries for analysis
- Loop structures for optimization