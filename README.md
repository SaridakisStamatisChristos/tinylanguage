# TinyLanguage

TinyLanguage is a compact, formally specified language with a parser, a static type
system, and an interpreter. It includes soundness invariants and property-based tests
for typed evaluation.

## Features

- **Grammar** written in EBNF (`docs/grammar.ebnf`).
- **Parser** with operator precedence.
- **Type checker** for `Int`, `Bool`, and function types.
- **Interpreter** with lexical scoping and closures.
- **Soundness invariants** documented in `docs/soundness.md`.
- **Property-based tests** powered by `proptest`.

## Running Tests

```bash
cargo test
```

## Example

```text
let add = \x: Int -> \y: Int -> x + y in add 2 3
```
