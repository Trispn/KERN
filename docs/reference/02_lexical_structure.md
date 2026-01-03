# KERN Language Reference - Lexical Structure

## 2.1 Character Set

KERN source code is encoded in UTF-8 and uses the following character classes:

- **Letters**: ASCII letters `a-z`, `A-Z`
- **Digits**: ASCII digits `0-9`
- **Symbols**: Special characters `_`, `-`, `+`, `*`, `/`, `=`, `<`, `>`, `!`, `?`, `.`, `:`, `;`, `,`, `(`, `)`, `[`, `]`, `{`, `}`, `|`, `&`, `^`, `~`, `%`
- **Whitespace**: Space, tab, newline, carriage return
- **Comments**: Lines starting with `//` until end of line

## 2.2 Identifiers

```
identifier = letter ( letter | digit | '_' | '-' )*
letter = 'a' .. 'z' | 'A' .. 'Z'
digit = '0' .. '9'
```

Identifiers must:
- Start with a letter
- Contain only letters, digits, underscore, or hyphen
- Not be reserved keywords
- Be unique within their scope

## 2.3 Keywords

The following are reserved keywords and cannot be used as identifiers:

```
entity
rule
flow
constraint
if
then
else
and
or
not
true
false
sym
num
bool
vec
ref
ctx
```

## 2.4 Literals

### 2.4.1 Numeric Literals

```
number = [ '-' ] digit+
```

Examples:
- `42`
- `-17`
- `0`

### 2.4.2 Boolean Literals

```
boolean = 'true' | 'false'
```

### 2.4.3 Symbol Literals

```
symbol = '"' character* '"'
character = ? any Unicode character except " and \ ?
```

Examples:
- `"user"`
- `"account_balance"`

## 2.5 Operators

KERN supports the following operators:

### 2.5.1 Arithmetic Operators
- `+` (addition)
- `-` (subtraction)
- `*` (multiplication)
- `/` (division)
- `%` (modulo)

### 2.5.2 Comparison Operators
- `==` (equal)
- `!=` (not equal)
- `<` (less than)
- `<=` (less than or equal)
- `>` (greater than)
- `>=` (greater than or equal)

### 2.5.3 Logical Operators
- `and` (logical AND)
- `or` (logical OR)
- `not` (logical NOT)

## 2.6 Punctuation

- `{ }` - Block delimiters
- `( )` - Grouping
- `[ ]` - Array/list delimiters
- `,` - List separator
- `.` - Field access
- `:` - Type annotation
- `;` - Statement terminator (optional)
- `=` - Assignment/equality
- `->` - Arrow for rules and flows