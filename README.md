# A1-azure-happy

AC 2. Root azure-pipelines.yml with a `script:` step running cargo test.

## How to test

1. Octiq QA -> Import -> GitHub -> this repo
2. In chat: `add a comment at the top of the README`
3. Read the two lines in the reply.

**Line 1 (the CI verdict - THIS IS THE TEST):**

    Tests required - CI runs `cargo test`

**Line 2 (the build/test outcome):**

    Tests passed - 4 passed        (or a real failure, never "no recipe")

## Crate API

The `fixture` crate is a small, dependency-free library with two public
functions:

- `add(a: i32, b: i32) -> i32` - returns the sum of two numbers.
- `addition(a: i32, b: i32) -> i32` - returns the sum of two numbers.
- `double(a: i32) -> i32` - returns the number multiplied by two.

```rust
use fixture::{add, addition, double};

assert_eq!(add(2, 2), 4);
assert_eq!(addition(2, 2), 4);
assert_eq!(double(21), 42);
```

## Running the tests

The test suite runs with `cargo test`:

```sh
cargo build --all-targets
cargo test
```

Unit tests live alongside the implementation in `src/lib.rs`, and integration
tests that exercise the public API from a consumer's perspective live in
`tests/integration.rs`. `azure-pipelines.yml` runs both `cargo build
--all-targets` (Build) and `cargo test` (Test) on `ubuntu-latest` when `main`
is updated.
