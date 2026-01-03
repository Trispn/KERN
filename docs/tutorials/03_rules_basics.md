# KERN Tutorial 03: Rules Basics

## Overview

This tutorial explores KERN's rule system, which is the primary mechanism for conditional logic and state transformation. Rules define "if this, then that" logic that executes deterministically.

## Rule Structure

A rule in KERN has the basic structure:

```kern
rule RuleName {
  if condition then
    action
}
```

## Simple Rule Example

Let's start with a simple rule that activates users:

```kern
entity User {
  id: num
  name: sym
  active: bool
}

rule ActivateValidUser {
  if user.id > 0 and user.name != "" then
    user.active = true
}
```

This rule says: "If a user has a positive ID and a non-empty name, then set their active status to true."

## Complex Conditions

Rules can have complex conditions using logical operators:

```kern
rule PremiumUser {
  if user.id > 0 and user.name != "" and user.account_balance >= 1000 then {
    user.account_type = "premium";
    user.discount = 0.1
  }
}
```

## Multiple Actions

Rules can perform multiple actions:

```kern
rule ProcessOrder {
  if order.status == "pending" and order.amount > 100 then {
    order.status = "processing";
    order.priority = "high";
    order.processed_at = current_time()  // Assuming this is provided as input
  }
}
```

## Rule Priorities

Rules can have explicit priorities to control execution order:

```kern
rule HighPriorityValidation {
  priority: 10
  if user.email != "" then
    user.email_validated = validate_email(user.email)
}

rule LowPriorityNotification {
  priority: 1
  if user.email_validated == true then
    send_welcome_email(user.email)
}
```

## Conditional Actions

Rules can have conditional actions:

```kern
rule CalculateShipping {
  if order.weight > 0 then {
    if order.weight < 5 then
      order.shipping_cost = 5
    else if order.weight < 10 then
      order.shipping_cost = 10
    else
      order.shipping_cost = 20
  }
}
```

## Rule Dependencies

Rules can depend on each other's outcomes:

```kern
entity Product {
  id: num
  name: sym
  price: num
  discounted_price: num
  is_on_sale: bool
}

rule ApplyDiscount {
  if product.is_on_sale == true then
    product.discounted_price = product.price * 0.9
}

rule UpdateDisplayPrice {
  if product.discounted_price > 0 then
    product.display_price = product.discounted_price
  else
    product.display_price = product.price
}
```

## AST Representation

The AST for a rule example would look like:
```
RuleDef: ActivateValidUser
├── Condition: user.id > 0 and user.name != ""
│   ├── BinaryOp: user.id > 0
│   │   ├── FieldAccess: user.id
│   │   └── Literal: 0
│   ├── LogicalOp: and
│   └── BinaryOp: user.name != ""
│       ├── FieldAccess: user.name
│       └── Literal: ""
└── Action: user.active = true
    ├── FieldAssignment: user.active
    └── Literal: true
```

## Execution Graph

The execution graph for rules includes:
- Condition evaluation nodes
- Action execution nodes
- Data dependency edges
- Priority ordering information

## Bytecode Snippet

The bytecode would include instructions for:
- Loading field values
- Comparing values
- Performing logical operations
- Setting field values
- Managing execution order

## Execution Trace Example

With a user having id=5 and name="Alice":
1. Rule condition evaluated: user.id > 0 and user.name != "" (true)
2. Rule action executed: user.active = true
3. Result: User is now active

## Rule Patterns

### 1. Validation Pattern
```kern
rule ValidateUser {
  if user.email != "" and contains(user.email, "@") then
    user.email_valid = true
}
```

### 2. Calculation Pattern
```kern
rule CalculateTotal {
  if order.items != [] then {
    total = 0;
    for item in order.items {
      total = total + item.price
    };
    order.total = total
  }
}
```

### 3. State Transition Pattern
```kern
rule ProcessPayment {
  if payment.status == "pending" and payment.amount > 0 then
    payment.status = "completed"
}
```

### 4. Notification Pattern
```kern
rule SendAlert {
  if system.error_count > 5 then
    alert.system_status = "critical"
}
```

## Rule Interactions

Multiple rules can interact with the same data:

```kern
rule ValidateOrder {
  if order.customer_id > 0 then
    order.customer_validated = true
}

rule CalculateTax {
  if order.customer_validated == true and order.total > 0 then
    order.tax = order.total * 0.08
}

rule CalculateTotalWithTax {
  if order.tax > 0 then
    order.total_with_tax = order.total + order.tax
}
```

## Error Handling in Rules

Rules can include error handling:

```kern
rule SafeDivision {
  if divisor != 0 then
    result = dividend / divisor
  else
    error_code = "DIVISION_BY_ZERO"
}
```

## Constraints with Rules

Rules work alongside constraints:

```kern
entity Account {
  balance: num
  status: sym
}

constraint PositiveBalance {
  account.balance >= 0
}

rule ProcessDeposit {
  if deposit.amount > 0 then
    account.balance = account.balance + deposit.amount
}
```

## PSI Observation

The PSI system observes:
- Rule conditions and their evaluation
- Rule actions and their effects
- Rule execution order and dependencies
- Priority information for execution planning
- State transitions caused by rules

## Best Practices

1. Keep rules focused on a single responsibility
2. Use clear, descriptive names for rules
3. Set appropriate priorities when needed
4. Ensure rule conditions are well-defined
5. Validate rule interactions to avoid conflicts
6. Use constraints for validation, rules for transformation

## Summary

In this tutorial, we learned:
- How to define basic rules with conditions and actions
- How to create complex conditions using logical operators
- How to perform multiple actions in a single rule
- How to set rule priorities
- How rules interact with each other
- How rules work with constraints
- Best practices for rule design

In the next tutorial, we'll explore constraints in detail, learning how to enforce data integrity and business rules.