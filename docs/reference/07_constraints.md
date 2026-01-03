# KERN Language Reference - Constraints

## 7.1 Definition

A constraint defines validation logic that must be satisfied for a state to be valid. Constraints are used to enforce business rules and data integrity.

## 7.2 Syntax

```
constraint_def = 'constraint' identifier '{' expression '}' ;
```

## 7.3 Semantics

Constraints define:
- Validation conditions that must be true
- Invariants that must hold in valid states
- Business rules that govern data relationships
- Error conditions when constraints are violated
- Validation timing (entry, exit, transition)

## 7.4 Examples

### 7.4.1 Basic Constraint
```
constraint PositiveBalance {
  account.balance >= 0
}
```

### 7.4.2 Complex Constraint
```
constraint ValidAge {
  user.age >= 0 and user.age <= 150
}
```

### 7.4.3 Relationship Constraint
```
constraint OrderTotal {
  order.total == sum(order.items[*].price)
}
```

## 7.5 Execution Guarantees

- Constraints are evaluated at specified validation points
- Constraint evaluation is deterministic
- Violations result in explicit error states
- Constraints do not modify state (pure validation)
- Constraint evaluation is fast and efficient

## 7.6 Error Conditions

| Error Code | Condition | Recoverable |
|------------|-----------|-------------|
| SYN005 | Invalid constraint expression | No |
| SEM008 | Constraint references undefined symbols | No |
| VAL001 | Constraint validation failed | No |
| VAL002 | Constraint timeout | No |

## 7.7 Bytecode Mapping

Constraints are compiled to:
- Validation expression bytecode
- Error handling code for violations
- Metadata about validation timing
- Symbol references for validation context

## 7.8 Validation Timing

Constraints can be evaluated at different times:
- **Entry**: Before state transition
- **Exit**: After state transition
- **Invariant**: Always maintained
- **Transition**: During specific transitions

## 7.9 Constraint Composition

Multiple constraints can be combined with logical operators:
```
constraint ComplexRule {
  (user.age >= 18 and user.has_license) or 
  (user.age >= 16 and user.has_permit and user.supervised)
}
```

## 7.10 PSI Observability

Constraints are observable as:
- Validation rules in the PSI knowledge base
- Invariant conditions for state analysis
- Error conditions for debugging
- Business rule definitions
- Data integrity requirements