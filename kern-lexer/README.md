# KERN Language Lexer

This crate implements the lexer for the KERN language based on the formal EBNF grammar specification.

## Lexical Elements

The KERN lexer recognizes the following lexical elements:

### Keywords
- `entity` - Defines a data model
- `rule` - Defines inference logic
- `flow` - Defines execution pipeline
- `constraint` - Defines validation logic
- `if` - Conditional execution
- `then` - Action execution
- `else` - Alternative execution
- `loop` - Loop execution
- `break` - Loop termination
- `halt` - Execution termination
- `and` - Logical AND operator
- `or` - Logical OR operator

### Identifiers and Literals
- `Identifier` - Names starting with a letter or underscore, followed by letters, digits, or underscores
- `Number` - Integer literals (positive or negative)

### Symbols and Operators
- `:` - Colon (used in rule definitions)
- `,` - Comma (used for argument separation)
- `.` - Dot (used for field access)
- `{` - Left brace (start of block)
- `}` - Right brace (end of block)
- `(` - Left parenthesis (start of argument list)
- `)` - Right parenthesis (end of argument list)
- `==` - Equality comparison
- `!=` - Inequality comparison
- `>` - Greater than comparison
- `<` - Less than comparison
- `>=` - Greater than or equal comparison
- `<=` - Less than or equal comparison
- `=` - Assignment operator

### Special Tokens
- `EOF` - End of file marker
- `Illegal(char)` - Represents invalid/unrecognized characters

## Grammar Compliance

This lexer strictly follows the EBNF grammar specification from the KERN language document:

```
letter = "A"…"Z" | "a"…"z" | "_" ;
digit = "0"…"9" ;
identifier = letter , { letter | digit } ;
number = [ "-" ] , digit , { digit } ;
newline = "
" ;
whitespace = " " | "	" | newline ;
```

## Usage

```rust
use kern_lexer::{Lexer, Token, TokenType};

let mut lexer = Lexer::new("entity Farmer { id location }");
let tokens = lexer.tokenize_all();

for token in tokens {
    println!("{:?}", token);
}
```

## Design Notes

- The lexer is designed for deterministic parsing with no ambiguity
- All tokens include position information (line, column, position) for error reporting
- Error handling is explicit - invalid characters are represented as Illegal tokens
- Strings are not supported as first-class citizens in KERN, so quotes are treated as illegal characters