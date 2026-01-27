# Soundness Invariants

TinyLanguage enforces a standard *progress* + *preservation* story for its typed core.
These are the invariants the implementation maintains:

## Core Invariants

1. **Well-typed programs do not get stuck (progress).**
   * If `type_of(e) = T`, then either `e` is a value, or `e` can take a step under the
     evaluator. In the interpreter this means `eval(e)` can only fail if the expression
     was ill-typed or used an unbound variable.

2. **Evaluation preserves types (preservation).**
   * If `type_of(e) = T` and `eval(e) = v`, then `v` has type `T`.
   * Closures capture environments that match the static typing environment from
     type checking.

3. **Boolean guards are enforced.**
   * For `if c then t else f`, the checker requires `c : Bool` and both branches share
     the same type.

4. **Binary operators are type-directed.**
   * Arithmetic (`+`, `-`, `*`) only applies to `Int` values.
   * Comparison (`<`) only applies to `Int` values and returns `Bool`.
   * Equality (`==`) requires operands of the same type and returns `Bool`.

5. **Application respects function types.**
   * For `f x`, the checker enforces `f : A -> B` and `x : A`, producing `B`.

These invariants are validated by unit tests and property-based tests that generate
well-typed programs and verify that evaluation yields values of the expected type.
