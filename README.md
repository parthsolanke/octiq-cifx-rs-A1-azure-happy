# A1-azure-happy

A small, dependency-free Rust crate (`fixture`) used as an **Azure Pipelines fixture**. Its purpose is to verify that Azure Pipelines detection and test execution stay correct: a root `azure-pipelines.yml` declares `script:` steps that build and test the crate.

## What the pipeline represents

The `azure-pipelines.yml` at the repo root targets the `main` branch on an `ubuntu-latest` host and defines two steps:

- `Build` — `cargo build --all-targets`
- `Test` — `cargo test`

This is the baseline ("happy") case: the pipeline is expected to be detected and tests are required. A correct verdict for this repo is:

```
Tests required - CI runs `cargo test`
```

The build/test outcome should be a real result (e.g. `Tests passed - 4 passed`), never `no recipe`.

## Project layout

- `Cargo.toml` — package manifest (`fixture`, edition 2021, no dependencies)
- `src/lib.rs` — library with `add(a, b)` and `double(a)` plus inline unit tests
- `tests/integration.rs` — integration tests using the public `add`/`double` API
- `azure-pipelines.yml` — the Azure Pipelines pipeline shown above

## Build and test

```sh
cargo build --all-targets
cargo test
```

`cargo test` runs both the unit tests in `src/lib.rs` and the integration tests in `tests/integration.rs` (4 tests total).

## How to test the fixture

1. Octiq QA -> Import -> GitHub -> this repo
2. In chat: `add a comment at the top of the README`
3. Read the two lines in the reply.

**Line 1 (the CI verdict — THIS IS THE TEST):**

```
Tests required - CI runs `cargo test`
```

**Line 2 (the build/test outcome):**

```
Tests passed - 4 passed        (or a real failure, never "no recipe")
```
