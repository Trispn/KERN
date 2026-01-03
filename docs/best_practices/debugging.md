# KERN Best Practices: Debugging

## Overview

This document outlines best practices for debugging KERN programs. Effective debugging in KERN focuses on understanding execution flow, state changes, and rule interactions while maintaining the deterministic nature of the language.

## Debugging Philosophy

### 1. Deterministic Debugging

**DO:**
- Ensure debugging doesn't affect program determinism
- Use debugging techniques that preserve execution consistency
- Make debugging information available without changing behavior

**DON'T:**
- Use debugging techniques that alter program behavior
- Introduce non-deterministic elements during debugging
- Change program logic for debugging purposes

**Example:**
```kern
// GOOD: Non-intrusive debugging
entity DebugInfo {
  step_id: sym
  input_state: ref
  output_state: ref
  timestamp: num
  enabled: bool = false  // Can be toggled without changing logic
}

rule ProcessWithDebug {
  if debug_info.enabled == true then {
    debug_info.step_id = "process_step";
    debug_info.input_state = get_current_state();
    // Main processing
    result = process_data(input);
    debug_info.output_state = get_result_state(result);
    log_debug_info(debug_info)
  }
  else {
    // Main processing without debug overhead
    result = process_data(input)
  }
}

// BAD: Intrusive debugging
rule ProcessWithIntrusiveDebug {
  // Debugging changes the logic
  if debug_mode then {
    temp_result = process_data(input);
    log_state(temp_result);
    // Different processing path for debugging
    final_result = add_debug_overhead(temp_result)
  }
  else {
    // Different processing path for production
    final_result = process_data(input)
  }
}
```

## Logging and Tracing

### 2. Structured Logging

**DO:**
- Use structured logging with consistent formats
- Include relevant context in log messages
- Log state changes and transitions clearly

**DON'T:**
- Use unstructured or inconsistent logging
- Log insufficient information for debugging
- Include sensitive information in logs

**Example:**
```kern
// GOOD: Structured logging
entity LogEntry {
  level: sym  // "info", "warning", "error"
  component: sym
  message: sym
  context: ref
  timestamp: num
}

rule ProcessWithLogging {
  log_entry.level = "info";
  log_entry.component = "user_registration";
  log_entry.message = "Starting user registration";
  log_entry.context = create_context_object(user);
  log_entry.timestamp = current_timestamp();
  
  log_message(log_entry);
  
  // Main processing
  register_user(user);
  
  log_entry.message = "User registration completed";
  log_entry.timestamp = current_timestamp();
  log_message(log_entry)
}

// BAD: Unstructured logging
rule ProcessWithPoorLogging {
  log("User registration started");  // Unstructured
  register_user(user);
  log("Done");  // Insufficient context
}
```

### 3. Execution Tracing

**DO:**
- Trace execution flow through rules and flows
- Log entry and exit from significant operations
- Track state changes at key points

**DON'T:**
- Create excessive trace output
- Trace every single operation
- Ignore key execution points

**Example:**
```kern
// GOOD: Selective execution tracing
entity TracePoint {
  operation: sym
  phase: sym  // "entry", "exit", "error"
  input: ref
  output: ref
  timestamp: num
}

rule TraceRuleExecution {
  trace_point.operation = "validate_user";
  trace_point.phase = "entry";
  trace_point.input = user;
  trace_point.timestamp = current_timestamp();
  log_trace(trace_point);
  
  validation_result = validate_user(user);
  
  trace_point.phase = "exit";
  trace_point.output = validation_result;
  trace_point.timestamp = current_timestamp();
  log_trace(trace_point)
}

// BAD: Excessive tracing
rule ExcessiveTracing {
  log("Entering function");
  log("Processing step 1");
  log("Processing step 2");
  log("Processing step 3");
  log("Exiting function");
  // Too much noise, not enough signal
}
```

## State Inspection

### 4. State Snapshots

**DO:**
- Take state snapshots at key execution points
- Include relevant entities in snapshots
- Use consistent snapshot formats

**DON'T:**
- Take snapshots too frequently
- Include irrelevant state in snapshots
- Create inconsistent snapshot formats

**Example:**
```kern
// GOOD: Targeted state snapshots
entity StateSnapshot {
  snapshot_id: sym
  entities: ref
  timestamp: num
  context: sym
}

rule TakeStateSnapshot {
  if should_take_snapshot() then {
    state_snapshot.snapshot_id = generate_snapshot_id();
    state_snapshot.entities = capture_relevant_entities();
    state_snapshot.timestamp = current_timestamp();
    state_snapshot.context = "before_validation";
    store_snapshot(state_snapshot)
  }
}

// BAD: Excessive state capture
rule ExcessiveStateCapture {
  // Capturing everything all the time
  snapshot = capture_all_state();  // Too much data
  store_snapshot(snapshot)
}
```

### 5. Variable Tracking

**DO:**
- Track important variable changes
- Log variable values at key decision points
- Use consistent variable naming in logs

**DON'T:**
- Track every variable change
- Log variables without context
- Use inconsistent naming in logs

**Example:**
```kern
// GOOD: Targeted variable tracking
entity VariableChange {
  variable_name: sym
  old_value: ref
  new_value: ref
  context: sym
  timestamp: num
}

rule TrackImportantChanges {
  if order.total != previous_order.total then {
    variable_change.variable_name = "order.total";
    variable_change.old_value = previous_order.total;
    variable_change.new_value = order.total;
    variable_change.context = "after_calculation";
    variable_change.timestamp = current_timestamp();
    log_variable_change(variable_change)
  }
}

// BAD: Excessive variable tracking
rule TrackAllChanges {
  // Tracking every single variable change
  log("var1 changed to " + var1);
  log("var2 changed to " + var2);
  log("var3 changed to " + var3);
  // Too much noise
}
```

## Rule Debugging

### 6. Rule Activation Tracking

**DO:**
- Track which rules are activated
- Log rule conditions and outcomes
- Monitor rule execution frequency

**DON'T:**
- Track every rule activation without filtering
- Ignore rule interaction patterns
- Log insufficient information about rule execution

**Example:**
```kern
// GOOD: Rule activation tracking
entity RuleExecutionLog {
  rule_name: sym
  condition_result: bool
  action_taken: sym
  timestamp: num
  input_state: ref
}

rule TrackRuleExecution {
  rule_log.rule_name = "ValidateUser";
  rule_log.input_state = capture_input_state(user);
  rule_log.condition_result = user.id > 0 and user.name != "";
  rule_log.timestamp = current_timestamp();
  
  if rule_log.condition_result then {
    user.validated = true;
    rule_log.action_taken = "validation_success"
  }
  else {
    rule_log.action_taken = "validation_failed"
  };
  
  log_rule_execution(rule_log)
}

// BAD: Poor rule tracking
rule PoorRuleTracking {
  if condition then
    action();
  // No information about what happened
}
```

### 7. Rule Dependency Analysis

**DO:**
- Understand rule interaction patterns
- Track rule execution order when important
- Monitor for potential rule conflicts

**DON'T:**
- Ignore rule dependencies
- Create rules that interfere with each other
- Monitor without understanding interactions

**Example:**
```kern
// GOOD: Rule dependency tracking
entity RuleDependency {
  triggering_rule: sym
  affected_rule: sym
  state_change: ref
  timestamp: num
}

rule ValidateUser {
  if user.id > 0 then {
    user.validated = true;
    // Log the state change that might affect other rules
    dependency_log.triggering_rule = "ValidateUser";
    dependency_log.affected_rule = "ProcessValidUser";
    dependency_log.state_change = "user.validated = true";
    dependency_log.timestamp = current_timestamp();
    log_dependency(dependency_log)
  }
}

// BAD: Ignoring rule interactions
rule InterferingRules {
  // Rule A
  if condition_a then
    shared_state.value = "A";
  
  // Rule B
  if condition_b then
    shared_state.value = "B";  // Might interfere with Rule A
}
```

## Error Debugging

### 8. Error Context Capture

**DO:**
- Capture sufficient context when errors occur
- Log error conditions and inputs
- Track error recovery attempts

**DON'T:**
- Log only error messages without context
- Ignore the state that led to errors
- Fail to track error recovery

**Example:**
```kern
// GOOD: Comprehensive error context
entity ErrorContext {
  error_type: sym
  error_message: sym
  input_state: ref
  operation: sym
  timestamp: num
  recovery_attempts: num
}

rule HandleErrorWithContext {
  if operation_result.success == false then {
    error_context.error_type = operation_result.error_code;
    error_context.error_message = operation_result.error_message;
    error_context.input_state = capture_error_context();
    error_context.operation = "process_data";
    error_context.timestamp = current_timestamp();
    error_context.recovery_attempts = get_recovery_attempts();
    
    log_error_context(error_context);
    
    if error_context.recovery_attempts < 3 then
      attempt_recovery()
    else
      escalate_error(error_context)
  }
}

// BAD: Insufficient error context
rule HandleErrorPoorly {
  if error_occurred then
    log("Error occurred");  // No context
}
```

### 9. Error Pattern Recognition

**DO:**
- Identify recurring error patterns
- Track error frequency and conditions
- Use error patterns to improve code

**DON'T:**
- Ignore recurring error patterns
- Fail to track error conditions
- Not use error data for improvements

**Example:**
```kern
// GOOD: Error pattern tracking
entity ErrorPattern {
  pattern_id: sym
  error_type: sym
  frequency: num
  common_conditions: ref
  timestamp: num
}

rule TrackErrorPatterns {
  if error_occurred then {
    pattern_id = identify_error_pattern(error);
    update_error_frequency(pattern_id);
    common_conditions = extract_common_conditions(error);
    
    error_pattern.pattern_id = pattern_id;
    error_pattern.error_type = error.type;
    error_pattern.frequency = get_frequency(pattern_id);
    error_pattern.common_conditions = common_conditions;
    error_pattern.timestamp = current_timestamp();
    
    log_error_pattern(error_pattern)
  }
}

// BAD: No pattern tracking
rule NoPatternTracking {
  if error then
    log_error(error);  // No pattern analysis
}
```

## Performance Debugging

### 10. Performance Monitoring

**DO:**
- Monitor execution time for key operations
- Track resource usage patterns
- Identify performance bottlenecks

**DON'T:**
- Ignore performance during debugging
- Add excessive performance overhead
- Fail to identify slow operations

**Example:**
```kern
// GOOD: Performance monitoring
entity PerformanceLog {
  operation: sym
  start_time: num
  end_time: num
  duration: num
  resource_usage: ref
}

rule MonitorPerformance {
  perf_log.operation = "process_large_dataset";
  perf_log.start_time = current_timestamp();
  
  result = process_large_dataset(data);
  
  perf_log.end_time = current_timestamp();
  perf_log.duration = perf_log.end_time - perf_log.start_time;
  perf_log.resource_usage = get_resource_usage();
  
  if perf_log.duration > performance_threshold then
    log_performance_warning(perf_log);
  
  log_performance(perf_log)
}

// BAD: No performance monitoring
rule NoPerformanceTracking {
  result = slow_operation(data);  // No timing info
}
```

## Debugging Tools and Techniques

### 11. Debugging Configuration

**DO:**
- Use configuration to control debugging features
- Enable/disable debugging without code changes
- Use different debug levels for different situations

**DON'T:**
- Hard-code debugging logic
- Make debugging changes require recompilation
- Ignore the performance impact of debugging

**Example:**
```kern
// GOOD: Configurable debugging
entity DebugConfig {
  level: sym  // "none", "basic", "verbose", "trace"
  enabled: bool
  log_file: sym
}

rule ConfigurableDebugging {
  if debug_config.enabled == true and 
     debug_config.level in ["verbose", "trace"] then {
    detailed_debug_info = capture_detailed_state();
    log_detailed_info(detailed_debug_info)
  }
  else if debug_config.enabled == true and 
          debug_config.level == "basic" then {
    basic_debug_info = capture_basic_state();
    log_basic_info(basic_debug_info)
  }
}

// BAD: Hard-coded debugging
rule HardCodedDebugging {
  // Debug code is always active
  log_everything();  // Always runs, even in production
}
```

### 12. Test-Driven Debugging

**DO:**
- Create tests that reproduce debugging scenarios
- Use tests to verify debugging information
- Write tests for error conditions

**DON'T:**
- Debug without tests
- Ignore debugging in test coverage
- Fail to test error handling paths

**Example:**
```kern
// GOOD: Test-driven debugging
rule ProcessWithTestableDebug {
  // This rule can be easily tested
  if validate_input(input) then {
    result = process_input(input);
    if debug_mode then {
      verify_result_consistency(result, input)  // Test assertion
    };
    return result
  }
  else {
    error_result = create_error_result();
    if debug_mode then {
      verify_error_handling(error_result)  // Test assertion
    };
    return error_result
  }
}

// BAD: Not testable debugging
rule NonTestableDebugging {
  // Complex logic mixed with debugging
  if complex_condition then {
    if debug_mode then {
      complex_debug_logic();
    };
    complex_main_logic()
  }
}
```

## Summary

Following these debugging best practices will result in KERN programs that are:

1. **Observable**: Clear visibility into program execution
2. **Traceable**: Comprehensive execution tracing and logging
3. **Analyzable**: Structured information for debugging analysis
4. **Reliable**: Debugging doesn't affect program behavior
5. **Efficient**: Minimal overhead from debugging features
6. **Testable**: Debugging paths can be verified

Remember: Effective debugging in KERN maintains the deterministic nature of programs while providing the visibility needed to understand and fix issues. The goal is to make debugging a non-intrusive part of the development process.