# KERN Best Practices: Rule Design

## Overview

This document outlines best practices for designing effective KERN rules. Rules are the primary mechanism for conditional logic and state transformation in KERN.

## Rule Structure and Organization

### 1. Single Responsibility Principle

**DO:**
- Design each rule to handle one specific concern
- Keep rules focused and simple
- Use multiple rules for complex logic

**DON'T:**
- Create rules that handle multiple unrelated concerns
- Pack too much logic into a single rule
- Create overly complex rules

**Example:**
```kern
// GOOD: Single responsibility rules
rule ValidateUserEmail {
  if user.email != "" and contains(user.email, "@") then
    user.email_valid = true
}

rule ValidateUserAge {
  if user.age >= 0 and user.age <= 150 then
    user.age_valid = true
}

rule ActivateUser {
  if user.email_valid == true and user.age_valid == true then
    user.active = true
}

// BAD: Multiple responsibilities in one rule
rule ComplexValidationAndActivation {
  if user.email != "" and contains(user.email, "@") and 
     user.age >= 0 and user.age <= 150 then {
    user.email_valid = true;
    user.age_valid = true;
    user.active = true;
    user.created_at = current_timestamp();
    user.status = "active"
  }
}
```

### 2. Clear Naming Conventions

**DO:**
- Use descriptive names that clearly indicate the rule's purpose
- Follow a consistent naming pattern
- Include the entity name when relevant

**DON'T:**
- Use generic names like "Process" or "Handle"
- Use abbreviations that aren't clear
- Name rules after implementation details

**Example:**
```kern
// GOOD: Descriptive names
rule ValidateCustomerEmailFormat {
  if customer.email != "" and validate_email_format(customer.email) then
    customer.email_format_valid = true
}

rule CalculateOrderTotal {
  if order.items != [] then {
    total = 0;
    for item in order.items {
      total = total + item.price * item.quantity
    };
    order.total = total
  }
}

// BAD: Generic or unclear names
rule Process {
  if condition then
    action
}

rule HandleValidation {
  if customer.email != "" then
    customer.valid = validate(customer.email)
}
```

## Condition Design

### 3. Simple and Readable Conditions

**DO:**
- Keep conditions as simple as possible
- Extract complex logic into helper functions or intermediate values
- Use intermediate variables for complex expressions

**DON'T:**
- Create overly complex nested conditions
- Repeat complex expressions in multiple places
- Make conditions hard to understand

**Example:**
```kern
// GOOD: Simple, readable conditions
rule ProcessHighValueOrder {
  is_high_value = order.total > 1000;
  is_premium_customer = customer.type == "premium";
  
  if is_high_value and is_premium_customer then
    apply_premium_processing(order)
}

// BAD: Complex, hard-to-read conditions
rule ProcessComplexOrder {
  if order.total > 1000 and customer.type == "premium" and 
     customer.status == "active" and customer.last_order_date > threshold and
     not customer.flagged_for_review then
    complex_processing(order)
}
```

### 4. Avoid Circular Dependencies

**DO:**
- Design rules so they don't trigger each other infinitely
- Use clear dependency chains
- Consider rule execution order

**DON'T:**
- Create rules that can trigger each other repeatedly
- Design circular state transitions
- Ignore rule interaction patterns

**Example:**
```kern
// GOOD: Clear dependency chain
rule ValidateOrder {
  if order.items != [] and order.customer_id > 0 then
    order.validation_status = "valid"
}

rule ProcessValidOrder {
  if order.validation_status == "valid" then
    order.processing_status = "in_progress"
}

rule CompleteOrder {
  if order.processing_status == "in_progress" then
    order.status = "completed"
}

// BAD: Circular dependency
rule IncrementA {
  if counter.a < 10 then
    counter.b = counter.b + 1  // Could trigger IncrementB
}

rule IncrementB {
  if counter.b < 10 then
    counter.a = counter.a + 1  // Could trigger IncrementA
}
```

## Action Design

### 5. Minimal Side Effects

**DO:**
- Keep rule actions focused and minimal
- Change only the necessary state
- Use rules primarily for state transformation

**DON'T:**
- Perform complex operations in rule actions
- Have rules with multiple unrelated side effects
- Use rules for complex business logic that should be in flows

**Example:**
```kern
// GOOD: Minimal, focused actions
rule UpdateUserStatus {
  if user.last_login > user.last_activity + 86400 then  // 24 hours
    user.status = "inactive"
}

rule CalculateDiscount {
  if customer.years_as_customer > 5 then
    customer.discount_rate = 0.10
}

// BAD: Complex actions with multiple side effects
rule ComplexActionRule {
  if condition then {
    // Multiple unrelated changes
    user.status = "updated";
    user.last_modified = current_timestamp();
    user.version = user.version + 1;
    
    // Complex processing
    for item in user.items {
      process_item(item)
    };
    
    // External call
    log_activity("user_updated", user.id)
  }
}
```

## Performance Considerations

### 6. Efficient Condition Evaluation

**DO:**
- Place the most selective conditions first
- Avoid expensive calculations in conditions
- Use indexes or precomputed values when possible

**DON'T:**
- Perform expensive operations in conditions
- Use complex calculations in frequently evaluated rules
- Ignore the cost of condition evaluation

**Example:**
```kern
// GOOD: Efficient condition evaluation
rule ProcessActivePremiumUser {
  // Fast, selective check first
  if user.active == true and 
     // Less expensive check second
     user.account_type == "premium" and
     // More expensive check last
     user.total_orders > 10 then
    apply_premium_processing(user)
}

// BAD: Inefficient condition evaluation
rule ProcessUserWithExpensiveCheck {
  if expensive_validation_function(user.data) and  // Expensive first
     user.active == true and                      // Fast check later
     user.account_type == "premium" then
    process_user(user)
}
```

### 7. Rule Priority Management

**DO:**
- Use explicit priorities when execution order matters
- Keep priority values reasonable and spaced
- Document why priorities are needed

**DON'T:**
- Use priorities unnecessarily
- Create complex priority schemes
- Ignore the impact of priority on performance

**Example:**
```kern
// GOOD: Clear priority usage
rule CriticalValidation {
  priority: 100
  if critical_condition then
    handle_critical_case()
}

rule StandardProcessing {
  priority: 50
  if standard_condition then
    handle_standard_case()
}

rule Cleanup {
  priority: 10
  if cleanup_condition then
    perform_cleanup()
}

// BAD: Complex priority scheme
rule ComplexPriorities {
  priority: 999999  // Arbitrary high number
  if condition then
    action
}
```

## Error Handling in Rules

### 8. Graceful Error Handling

**DO:**
- Handle potential errors explicitly
- Use error states instead of failing silently
- Log errors when appropriate

**DON'T:**
- Ignore potential error conditions
- Allow rules to fail without handling
- Create rules that can crash the system

**Example:**
```kern
// GOOD: Explicit error handling
rule SafeDivision {
  if divisor != 0 then {
    result.value = dividend / divisor;
    result.success = true
  }
  else {
    result.success = false;
    result.error_code = "DIVISION_BY_ZERO";
    result.error_message = "Cannot divide by zero"
  }
}

// BAD: No error handling
rule UnsafeDivision {
  if condition then
    result = dividend / divisor  // Could fail if divisor is 0
}
```

## Rule Testing and Validation

### 9. Testable Rule Design

**DO:**
- Design rules that can be tested in isolation
- Use clear, predictable logic
- Make rules' behavior easy to verify

**DON'T:**
- Create rules that are hard to test
- Use non-deterministic elements
- Make rules dependent on complex external state

**Example:**
```kern
// GOOD: Testable rule
rule CalculateTax {
  if order.amount > 0 then {
    tax_rate = 0.08;  // Fixed value for predictability
    order.tax = order.amount * tax_rate;
    order.total_with_tax = order.amount + order.tax
  }
}

// Test cases:
// Input: order.amount = 100
// Expected: order.tax = 8, order.total_with_tax = 108

// BAD: Hard to test rule
rule TimeDependentRule {
  if current_time() > threshold_time then  // Non-deterministic
    apply_time_based_logic()
}
```

## Rule Composition and Interaction

### 10. Clear Rule Interactions

**DO:**
- Design rules that work well together
- Consider how rules affect each other
- Document rule interaction patterns

**DON'T:**
- Create rules that interfere with each other
- Ignore the cumulative effect of multiple rules
- Design rules without considering the rule set

**Example:**
```kern
// GOOD: Well-coordinated rules
rule ValidateInput {
  if input.data != "" then
    input.valid = true
}

rule ProcessValidInput {
  if input.valid == true then
    output.result = process(input.data)
}

rule LogProcessingResult {
  if output.result != null then
    log_result(output.result)
}

// BAD: Interfering rules
rule SetFlagA {
  if condition then
    shared_flag = "A"
}

rule SetFlagB {
  if different_condition then
    shared_flag = "B"  // Could interfere with SetFlagA
}
```

## Performance Optimization

### 11. Rule Activation Optimization

**DO:**
- Design conditions that are selective
- Avoid rules that activate too frequently
- Consider the cost of rule evaluation

**DON'T:**
- Create rules that activate on every change
- Ignore the frequency of rule activation
- Design rules that are expensive to evaluate

**Example:**
```kern
// GOOD: Selective rule activation
rule ProcessHighValueOrder {
  // Only activates for high-value orders
  if order.total > 10000 then
    apply_special_processing(order)
}

// BAD: High-frequency activation
rule ProcessEveryChange {
  // Activates on every minor change
  if order.any_field_changed then
    expensive_validation(order)  // Expensive for frequent activation
}
```

## Summary

Following these rule design best practices will result in KERN programs that are:

1. **Maintainable**: Rules are clear, focused, and well-organized
2. **Efficient**: Rules are optimized for performance
3. **Reliable**: Rules handle errors gracefully and predictably
4. **Testable**: Rules can be verified and validated
5. **Scalable**: Rule sets can grow without becoming unmanageable

Remember: Rules are the heart of KERN's logic processing. Well-designed rules make KERN programs robust, efficient, and easy to understand.