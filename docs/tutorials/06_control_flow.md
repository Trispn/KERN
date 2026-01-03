# KERN Tutorial 06: Control Flow

## Overview

This tutorial explores KERN's control flow mechanisms, which provide explicit ways to direct execution flow, including conditionals, sequential execution, and context management.

## Basic Conditional Execution

The fundamental control flow construct in KERN is the conditional:

```kern
if condition then
  action
```

### Simple Conditional

```kern
entity User {
  id: num
  status: sym
}

rule UpdateStatus {
  if user.id > 0 then
    user.status = "active"
}
```

### Conditional with Else

```kern
rule ConditionalUpdate {
  if user.id > 0 then
    user.status = "active"
  else
    user.status = "inactive"
}
```

## Complex Conditionals

Conditionals can have complex expressions:

```kern
rule ComplexDecision {
  if user.id > 0 and user.name != "" and user.email_valid == true then {
    user.status = "verified";
    user.access_level = "full"
  }
  else {
    user.status = "pending";
    user.access_level = "limited"
  }
}
```

## Nested Conditionals

Conditionals can be nested:

```kern
rule NestedLogic {
  if user.authenticated == true then {
    if user.role == "admin" then
      grant_admin_access()
    else if user.role == "moderator" then
      grant_moderator_access()
    else
      grant_user_access()
  }
  else
    redirect_to_login()
}
```

## Sequential Execution

Multiple operations can execute in sequence:

```kern
flow SequentialProcess {
  validate_input;
  process_data;
  store_result;
  send_notification
}
```

## Sequential with Data Flow

Sequential operations can pass data between steps:

```kern
flow DataPipeline {
  load_data -> raw_data;
  clean_data(raw_data) -> clean_data;
  validate_data(clean_data) -> validated_data;
  transform_data(validated_data) -> transformed_data;
  store_data(transformed_data) -> result
}
```

## Context Switching

KERN provides explicit context switching:

```kern
entity ProcessingContext {
  temp_data: ref
  state: sym
}

rule ContextualProcessing {
  if needs_special_processing(user) then
    with special_context {
      temp_data = prepare_special_data(user);
      state = "processing_special";
      result = special_process(temp_data)
    }
}
```

## Advanced Context Management

Contexts can be managed more complexly:

```kern
flow ComplexContextFlow {
  with validation_context {
    validate_input -> validation_result
  };
  if validation_result == "valid" then
    with processing_context {
      process_data(validation_result) -> processed_data
    };
  with storage_context {
    store_data(processed_data)
  }
}
```

## Loop Patterns (Bounded)

While KERN doesn't have traditional loops, bounded iteration can be achieved:

```kern
entity ProcessList {
  items: vec
  index: num
  processed_count: num
}

rule ProcessNextItem {
  if process_list.index < length(process_list.items) and 
     process_list.processed_count < 100 then {  // Bounded processing
    item = process_list.items[process_list.index];
    process_item(item);
    process_list.index = process_list.index + 1;
    process_list.processed_count = process_list.processed_count + 1
  }
}
```

## Pattern Matching Alternative

KERN uses explicit conditionals instead of pattern matching:

```kern
rule TypeBasedProcessing {
  if data.type == "text" then
    process_text(data)
  else if data.type == "image" then
    process_image(data)
  else if data.type == "video" then
    process_video(data)
  else
    log_error("Unknown type: " + data.type)
}
```

## State Machine Pattern

Control flow can implement state machines:

```kern
entity StateMachine {
  current_state: sym
  input: ref
  output: ref
}

rule StateTransition {
  if state_machine.current_state == "idle" and has_input(state_machine.input) then
    state_machine.current_state = "processing"
  else if state_machine.current_state == "processing" and processing_complete() then
    state_machine.current_state = "completed"
  else if state_machine.current_state == "completed" then
    state_machine.current_state = "idle"
}
```

## Error Handling Patterns

Control flow includes explicit error handling:

```kern
rule SafeOperation {
  if validate_inputs() then {
    result = perform_operation();
    if result.success == true then
      handle_success(result)
    else
      handle_operation_error(result.error)
  }
  else
    handle_validation_error()
}
```

## Conditional Flows

Flows can include conditional execution:

```kern
flow ConditionalFlow {
  load_data -> input_data;
  if validate_data(input_data) then {
    process_data(input_data) -> processed_data;
    store_data(processed_data)
  }
  else {
    log_invalid_data(input_data);
    send_error_notification()
  }
}
```

## Multi-Branch Decision Trees

Complex decision trees are expressed as nested conditionals:

```kern
rule PricingLogic {
  if customer.type == "premium" then {
    if product.category == "electronics" then
      apply_discount(0.15)
    else if product.category == "books" then
      apply_discount(0.10)
    else
      apply_discount(0.05)
  }
  else if customer.type == "standard" then {
    if order.total > 100 then
      apply_discount(0.05)
    else
      apply_discount(0.02)
  }
  else
    apply_discount(0.0)  // No discount for others
}
```

## AST Representation

The AST for control flow constructs would look like:
```
IfAction
├── Condition: user.id > 0 and user.name != ""
├── ThenAction: user.status = "active"
└── ElseAction: user.status = "inactive"
SequenceAction
├── Action1: validate_input
├── Action2: process_data
└── Action3: store_result
ContextAction: special_context
└── Actions: [prepare_special_data(user), ...]
```

## Execution Graph

The execution graph for control flow includes:
- Conditional branch nodes
- Sequential execution paths
- Context switching points
- Data dependency edges
- Error handling branches

## Bytecode Snippet

The bytecode would include instructions for:
- Conditional evaluation (COMPARE, JMP_IF)
- Sequential execution
- Context switching (CTX_SWITCH)
- Data flow management
- Error handling

## Execution Trace Example

With a complex conditional:
1. Condition evaluated: user.id > 0 and user.name != "" (true)
2. Then branch executed: user.status = "active"
3. Result: User status updated to active

## Best Practices for Control Flow

### 1. Keep Conditionals Readable
```kern
// Good: Clear and readable
rule ClearConditional {
  if user.is_valid and user.has_permission then
    allow_access()
}

// Avoid: Complex nested conditionals
rule ComplexConditional {
  if user.authenticated then {
    if user.verified then {
      if user.active then {
        if user.role != "banned" then
          allow_access()
      }
    }
  }
}
```

### 2. Use Descriptive Variable Names
```kern
rule DescriptiveNames {
  is_user_valid = validate_user(user);
  has_required_permissions = check_permissions(user);
  
  if is_user_valid and has_required_permissions then
    process_user(user)
}
```

### 3. Limit Nesting Depth
```kern
// Better: Flattened conditions
rule FlattenedConditions {
  if user.authenticated == true and
     user.verified == true and
     user.active == true and
     user.role != "banned" then
    allow_access()
}
```

### 4. Handle All Cases
```kern
rule CompleteHandling {
  if user.type == "admin" then
    grant_admin_access()
  else if user.type == "moderator" then
    grant_moderator_access()
  else if user.type == "user" then
    grant_user_access()
  else
    deny_access()  // Handle unknown types
}
```

## PSI Observation

The PSI system observes:
- Control flow structures and branches
- Conditional logic and decision points
- Context switching and its effects
- Sequential execution patterns
- Error handling paths
- Data flow through control structures

## Performance Considerations

Control flow should be efficient:

```kern
// Efficient: Simple condition
rule SimpleCheck {
  if user.active == true then
    process_user(user)
}

// Less efficient: Complex calculation in condition
rule ComplexCheck {
  if expensive_calculation(user.data) == expected_value then
    process_user(user)
}
```

## Summary

In this tutorial, we learned:
- How to use basic conditional execution
- How to create complex conditional logic
- How to implement sequential execution
- How to manage contexts with control flow
- How to handle errors explicitly
- Best practices for control flow design
- How to avoid common pitfalls

In the next tutorial, we'll explore contexts in more depth, learning how to manage state isolation and execution environments effectively.