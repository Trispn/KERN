# KERN Language Token Definitions

This document provides a comprehensive overview of all token definitions for the KERN language lexer based on the EBNF grammar specification.

## Token Categories

### 1. Keyword Tokens

| Token Type | String Value | Description |
|------------|--------------|-------------|
| `Entity` | "entity" | Defines a data model structure |
| `Rule` | "rule" | Defines inference logic and rules |
| `Flow` | "flow" | Defines execution pipeline |
| `Constraint` | "constraint" | Defines validation logic |
| `If` | "if" | Conditional execution keyword |
| `Then` | "then" | Action execution keyword |
| `Else` | "else" | Alternative execution keyword |
| `Loop` | "loop" | Loop execution keyword |
| `Break` | "break" | Loop termination keyword |
| `Halt` | "halt" | Execution termination keyword |
| `And` | "and" | Logical AND operator |
| `Or` | "or" | Logical OR operator |

### 2. Identifier and Literal Tokens

| Token Type | Format | Description |
|------------|--------|-------------|
| `Identifier(String)` | letter { letter \| digit } | Variable, entity, or function names |
| `Number(i64)` | [ "-" ] digit { digit } | Integer literals (positive or negative) |

**Lexical Rules:**
- `letter` = "A"…"Z" \| "a"…"z" \| "_"
- `digit` = "0"…"9"

### 3. Symbol Tokens

| Token Type | Character(s) | Description |
|------------|--------------|-------------|
| `Colon` | ":" | Used in rule definitions |
| `Comma` | "," | Argument separator |
| `Dot` | "." | Field access operator |
| `LeftBrace` | "{" | Start of block |
| `RightBrace` | "}" | End of block |
| `LeftParen` | "(" | Start of argument list |
| `RightParen` | ")" | End of argument list |

### 4. Operator Tokens

| Token Type | Character(s) | Description |
|------------|--------------|-------------|
| `Equal` | "==" | Equality comparison |
| `NotEqual` | "!=" | Inequality comparison |
| `Greater` | ">" | Greater than comparison |
| `Less` | "<" | Less than comparison |
| `GreaterEqual` | ">=" | Greater than or equal comparison |
| `LessEqual` | "<=" | Less than or equal comparison |
| `Assignment` | "=" | Assignment operator |

### 5. Special Tokens

| Token Type | Description |
|------------|-------------|
| `Illegal(char)` | Represents invalid or unrecognized characters |
| `Eof` | End of file marker |

## Token Classification Methods

The enhanced token definitions include helper methods for classification:

- `is_keyword()` - Checks if a token is a keyword
- `is_operator()` - Checks if a token is an operator
- `is_delimiter()` - Checks if a token is a delimiter

## Position Tracking

Each token includes position information:
- `line` - Source line number
- `column` - Source column number
- `position` - Absolute position in source
- `value` - Optional string representation of the token value

## Error Handling

The lexer handles errors by:
1. Converting invalid characters to `Illegal(char)` tokens
2. Properly handling unrecognized sequences
3. Following KERN's principle of explicit behavior with no hidden state

## Compliance with KERN Principles

- **Determinism**: Same input always produces same token sequence
- **Minimalism**: Only necessary tokens are defined
- **Explicit Logic**: All lexical elements are clearly defined
- **PSI-First Design**: Tokens are structured for easy analysis by PSI.brain