# KERN Language Reference - Rules

## 5.1 Definition

A rule defines conditional logic that executes when its condition is satisfied. Rules are the primary mechanism for inference and state transformation in KERN.

## 5.2 Syntax

```
rule_def = 'rule' identifier '{' 'if' condition 'then' action '}' ;
condition = expression ;
action = expression | assignment | sequence ;
assignment = identifier '=' expression ;
sequence = '{' { expression ';' } '}' ;
```

## 5.3 Semantics

Rules define:
- A condition that determines when the rule fires
- An action that executes when the condition is true
- Priority for execution order (optional)
- Side effects that modify the execution state

## 5.4 Examples

### 5.4.1 Basic Rule
```
rule ActivateUser {
  if user.active == false then
    user.active = true
}
```

### 5.4.2 Rule with Complex Condition
```
rule CalculateBonus {
  if employee.salary > 50000 and employee.years > 5 then {
    employee.bonus = employee.salary * 0.1;
    employee.status = "senior"
  }
}
```

### 5.4.3 Rule with Multiple Actions
```
rule ProcessOrder {
  if order.status == "pending" and order.amount > 1000 then {
    order.status = "review";
    order.priority = "high";
    notify.manager(order.id)
  }
}
```

## 5.5 Execution Guarantees

- Rules execute deterministically based on their conditions
- Rule execution order follows priority and dependency rules
- Each rule execution is atomic
- No side effects outside the explicitly defined actions
- Rules execute until fixpoint (no more rules can fire)

## 5.6 Error Conditions

| Error Code | Condition | Recoverable |
|------------|-----------|-------------|
| SYN003 | Missing 'if' or 'then' | No |
| SEM003 | Circular rule dependencies | Yes |
| SEM004 | Invalid condition expression | No |
| SEM005 | Invalid action expression | No |
| VM001 | Infinite rule execution loop | No |

## 5.7 Bytecode Mapping

Rules are compiled to:
- Condition evaluation bytecode
- Action execution bytecode
- Priority metadata
- Dependency tracking information
- Graph nodes representing the rule structure

## 5.8 Rule Priority

Rules may have explicit priorities:
```
rule HighPriorityRule {
  priority: 10
  if condition then action
}
```

Default priority is 0. Higher numbers execute first.

## 5.9 PSI Observability

Rules are observable as:
- Conditional logic in the PSI knowledge base
- Inference patterns for reasoning
- Execution traces for debugging
- Dependency graphs for analysis
- Priority information for execution planning