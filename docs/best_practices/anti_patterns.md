# KERN Best Practices: Anti-Patterns

## Overview

This document outlines common anti-patterns to avoid in KERN programming. Understanding these anti-patterns helps ensure that your KERN programs are efficient, maintainable, and deterministic.

## Determinism Anti-Patterns

### 1. Hidden State Mutation

**Problem:** Modifying state that isn't explicitly part of the current scope.

**Example:**
```kern
// BAD: Hidden state mutation
entity GlobalState {
  counter: num
}

rule HiddenMutation {
  // Modifying global state without explicit reference
  global_state.counter = global_state.counter + 1  // Hidden dependency
}

// GOOD: Explicit state management
entity ProcessingContext {
  counter: num
}

rule ExplicitMutation {
  with processing_context {
    processing_context.counter = processing_context.counter + 1
  }
}
```

### 2. Implicit Ordering Assumptions

**Problem:** Assuming that operations will execute in a particular order without explicit control.

**Example:**
```kern
// BAD: Implicit ordering assumption
rule ProcessA {
  if condition_a then
    shared_resource.value = "A"
}

rule ProcessB {
  if condition_b then
    // Assumes ProcessA has already run
    result = shared_resource.value + "_processed"
}

// GOOD: Explicit ordering control
rule ProcessA {
  if condition_a then
    shared_resource.value = "A";
    shared_resource.processed_a = true
}

rule ProcessB {
  if shared_resource.processed_a == true and condition_b then
    result = shared_resource.value + "_processed"
}
```

### 3. Dynamic Rule Creation

**Problem:** Creating rules dynamically during execution.

**Example:**
```kern
// BAD: Dynamic rule creation (conceptual - not supported in KERN)
rule DynamicRuleCreator {
  if condition then {
    // Conceptual: creating new rules at runtime
    create_new_rule_at_runtime()
  }
}

// GOOD: Static rule definition
rule PredefinedRule {
  if condition then
    action
}
```

## Performance Anti-Patterns

### 4. Unbounded Recursion

**Problem:** Creating recursive patterns that could run indefinitely.

**Example:**
```kern
// BAD: Potential infinite recursion
entity Counter {
  value: num
}

rule IncrementCounter {
  if counter.value < 100 then
    counter.value = counter.value + 1  // Could trigger itself
}

// GOOD: Bounded recursion
rule BoundedIncrement {
  if counter.value < 100 and counter.value < counter.max_value then {
    counter.value = counter.value + 1;
    counter.iteration_count = counter.iteration_count + 1
  }
}
```

### 5. Context Leakage

**Problem:** Allowing contexts to access or modify state outside their intended scope.

**Example:**
```kern
// BAD: Context leakage (conceptual)
flow ContextLeakageFlow {
  with context_a {
    // Accessing or modifying context_b's state directly
    context_b.shared_value = calculate_value()  // Not allowed in KERN
  }
}

// GOOD: Proper context isolation
flow ProperContextFlow {
  with validation_context {
    validation_context.input = get_input();
    validation_context.result = validate(validation_context.input)
  };
  
  with processing_context {
    // Explicit data passing
    processing_context.input = validation_context.input;
    processing_context.validated = validation_context.result
  }
}
```

## Rule Anti-Patterns

### 6. Overlapping Write Domains

**Problem:** Multiple rules modifying the same data without coordination.

**Example:**
```kern
// BAD: Overlapping write domains
rule RuleA {
  if condition_a then
    shared_entity.value = "A"
}

rule RuleB {
  if condition_b then
    shared_entity.value = "B"  // Could conflict with RuleA
}

// GOOD: Coordinated write domains
rule RuleA {
  if condition_a and shared_entity.value != "B" then
    shared_entity.value = "A"
}

rule RuleB {
  if condition_b and shared_entity.value != "A" then
    shared_entity.value = "B"
}

// Or use a coordinator rule
rule Coordinator {
  if condition_a and not condition_b then
    shared_entity.value = "A"
  else if condition_b and not condition_a then
    shared_entity.value = "B"
}
```

### 7. Rule Explosion

**Problem:** Creating too many small, related rules instead of a few well-structured ones.

**Example:**
```kern
// BAD: Rule explosion
rule ValidateEmail {
  if user.email != "" then
    user.email_not_empty = true
}

rule ValidateEmailFormat {
  if user.email_not_empty and contains(user.email, "@") then
    user.email_format_valid = true
}

rule ValidateEmailDomain {
  if user.email_format_valid and validate_domain(user.email) then
    user.email_domain_valid = true
}

// GOOD: Consolidated validation
rule ValidateEmailCompletely {
  if user.email != "" and 
     contains(user.email, "@") and 
     validate_domain(user.email) then
    user.email_valid = true
}
```

## Flow Anti-Patterns

### 8. Hidden Side Effects

**Problem:** Flows that modify state without clear indication of what they're changing.

**Example:**
```kern
// BAD: Hidden side effects
flow HiddenSideEffects {
  load_data;
  process_data;  // What does this modify?
  store_result  // What is stored?
}

// GOOD: Clear side effects
flow ClearSideEffects {
  input_data = load_data();
  validation_result = validate_data(input_data);
  if validation_result.success == true then {
    processed_data = transform_data(input_data);
    store_processed_data(processed_data)
  }
  else
    handle_validation_error(validation_result.errors)
}
```

### 9. Complex Nested Control Flow

**Problem:** Creating flows with deeply nested conditional logic.

**Example:**
```kern
// BAD: Complex nested flow
flow ComplexNestedFlow {
  if condition1 then {
    if condition2 then {
      if condition3 then {
        if condition4 then
          action_a()
        else
          action_b()
      }
      else
        action_c()
    }
    else
      action_d()
  }
  else
    action_e()
}

// GOOD: Flattened conditions
flow FlattenedFlow {
  if condition1 and condition2 and condition3 and condition4 then
    action_a()
  else if condition1 and condition2 and condition3 then
    action_b()
  else if condition1 and condition2 then
    action_c()
  else if condition1 then
    action_d()
  else
    action_e()
}
```

## Context Anti-Patterns

### 10. Context Inheritance Cycles

**Problem:** Creating circular dependencies between contexts.

**Example:**
```kern
// BAD: Context dependency cycle (conceptual)
context ContextA {
  value: ref
  depends_on_b: ref  // References ContextB
}

context ContextB {
  value: ref
  depends_on_a: ref  // References ContextA - creates cycle
}

// GOOD: Linear context dependencies
context BaseContext {
  base_value: ref
}

context DerivedContext {
  base_ref: ref  // References BaseContext
  derived_value: ref
}
```

## Error Handling Anti-Patterns

### 11. Silent Failures

**Problem:** Operations that fail without any indication of failure.

**Example:**
```kern
// BAD: Silent failure
rule SilentFailure {
  if condition then {
    result = potentially_failing_operation();
    // No check if operation succeeded
    use_result(result)  // Could use invalid result
  }
}

// GOOD: Explicit error handling
rule ExplicitErrorHandling {
  if condition then {
    result = potentially_failing_operation();
    if result.success == true then
      use_result(result.data)
    else
      handle_error(result.error)
  }
}
```

### 12. Exception Swallowing

**Problem:** Catching errors but not handling them appropriately.

**Example:**
```kern
// BAD: Error swallowing
rule ErrorSwallowing {
  if condition then {
    try {
      result = operation()
    }
    catch error {
      // Error caught but not handled
      // Silent continuation could lead to invalid state
    };
    // Continue as if operation succeeded
    use_result(result)
  }
}

// GOOD: Proper error handling
rule ProperErrorHandling {
  if condition then {
    try {
      result = operation();
      use_result(result)
    }
    catch error {
      log_error(error);
      set_error_state();
      // Either recover or fail gracefully
      if can_recover(error) then
        recovery_action()
      else
        fail_gracefully(error)
    }
  }
}
```

## Resource Management Anti-Patterns

### 13. Resource Leaks

**Problem:** Acquiring resources without properly releasing them.

**Example:**
```kern
// BAD: Resource leak
context ResourceContext {
  resource_handle: ref
}

flow ResourceLeakFlow {
  with resource_context {
    resource_context.resource_handle = acquire_resource();
    use_resource(resource_context.resource_handle);
    // Resource never released
  }
}

// GOOD: Proper resource management
flow ProperResourceManagement {
  with resource_context {
    resource_context.resource_handle = acquire_resource();
    if resource_context.resource_handle != null then {
      use_resource(resource_context.resource_handle);
      release_resource(resource_context.resource_handle);
      resource_context.resource_handle = null
    }
  }
}
```

## Performance Anti-Patterns

### 14. Premature Optimization

**Problem:** Optimizing code before identifying actual bottlenecks.

**Example:**
```kern
// BAD: Premature optimization
rule PrematureOptimization {
  // Complex, hard-to-understand optimized version
  // that may not actually be faster
  if complex_optimized_condition_check() then
    complex_optimized_action()
}

// GOOD: Clear, readable code first
rule ClearReadableCode {
  if simple_clear_condition() then
    simple_clear_action()
}
// Optimize only if performance analysis shows it's needed
```

### 15. Inefficient Data Access

**Problem:** Accessing data in an inefficient manner.

**Example:**
```kern
// BAD: Inefficient data access
rule InefficientAccess {
  // Repeatedly accessing the same data
  if get_large_dataset().property_a > threshold then
    result_a = process(get_large_dataset().property_a);
  
  if get_large_dataset().property_b > threshold then
    result_b = process(get_large_dataset().property_b)
}

// GOOD: Efficient data access
rule EfficientAccess {
  dataset = get_large_dataset();
  if dataset.property_a > threshold then
    result_a = process(dataset.property_a);
  
  if dataset.property_b > threshold then
    result_b = process(dataset.property_b)
}
```

## Security Anti-Patterns

### 16. Inadequate Input Validation

**Problem:** Not properly validating inputs before processing.

**Example:**
```kern
// BAD: Insufficient validation
rule InsufficientValidation {
  if user_input then
    process_unvalidated_data(user_input)  // Security risk
}

// GOOD: Proper validation
constraint ValidInput {
  length(user_input.data) < MAX_INPUT_SIZE and
  validate_format(user_input.data)
}

rule ProperValidation {
  if constraint.ValidInput then
    process_validated_data(user_input)
  else
    reject_invalid_input(user_input)
}
```

## General Anti-Patterns

### 17. Magic Numbers and Strings

**Problem:** Using hardcoded values without explanation.

**Example:**
```kern
// BAD: Magic numbers
rule MagicNumbers {
  if user.account_balance > 10000 then  // What is 10000?
    user.status = "premium"  // What does "premium" mean?
}

// GOOD: Named constants
entity Constants {
  premium_threshold: num = 10000
  premium_status: sym = "premium"
}

rule NamedConstants {
  if user.account_balance > constants.premium_threshold then
    user.status = constants.premium_status
}
```

### 18. Complex Expressions

**Problem:** Using overly complex expressions that are hard to understand.

**Example:**
```kern
// BAD: Complex expression
rule ComplexExpression {
  if (user.age >= 18 and user.has_license) or 
     (user.age >= 16 and user.has_permit and user.supervised) or
     (user.age >= 14 and user.has_special_permit and user.guardian_present and user.school_approval) then
    allow_action()
}

// GOOD: Simplified with intermediate values
rule SimplifiedLogic {
  is_adult = user.age >= 18 and user.has_license;
  is_young_with_permit = user.age >= 16 and user.has_permit and user.supervised;
  is_young_with_special = user.age >= 14 and user.has_special_permit and 
                          user.guardian_present and user.school_approval;
  
  if is_adult or is_young_with_permit or is_young_with_special then
    allow_action()
}
```

## Summary

Avoiding these anti-patterns will result in KERN programs that are:

1. **Deterministic**: No hidden state or implicit ordering
2. **Efficient**: Proper resource management and performance considerations
3. **Maintainable**: Clear, readable code without complex expressions
4. **Secure**: Proper validation and error handling
5. **Reliable**: No silent failures or resource leaks
6. **Scalable**: Well-structured without rule explosion

Remember: The goal of avoiding anti-patterns is to write KERN code that is clear, efficient, and maintains the deterministic nature of the language while being easy to understand and maintain.