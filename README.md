## Expression Parser

This is a simple implementation of a parser that takes a string and computes its numerical value using the given rules.
Operators are applied in order of precedence from left to right. An exception to this is brackets which are used to explicitly denote precedence by grouping parts of an expression that should be evaluated first.

### Rules:
Instead of using mathematical operators directly, following alphabatical mapping can also be used:
- a => + 
- b => - 
- c => *
- d => /
- e => (
- f => )

### Usage
```rust
let input = "3c4d2aee2a4c41fc4f";
let expression = Expression::parse(input).unwrap();

let result = expression.evaluate().unwrap();
```

### Running Tests
Tests can be run using the following command:
```bash
$ cargo t
```
