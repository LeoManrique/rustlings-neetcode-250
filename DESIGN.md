# Design

## Product

Rustlings UX driving 276 NeetCode-250 problems. User edits `exercises/<NNN-slug>/src/lib.rs`; on save, tests in `tests/solution.rs` run; on green, advance to the next exercise. State persists in `.rustlings-neetcode-state.txt`.

## Per-exercise shape

Each exercise is its own Cargo crate:

- `src/lib.rs` — scaffold with `pub struct Solution; impl Solution { pub fn <name>(...) -> ... {} }`. User-editable.
- `tests/solution.rs` — integration tests; do not edit. Uses `include!("../src/lib.rs")` so it sees the user's helpers.
- `README.md` — problem statement.
- `Cargo.toml` — `[package] name = "<slug>"`, edition 2024, `[[test]] name = "solution"`.

`solutions/<NNN-slug>/` mirrors the same shape with the reference solution (package name `<slug>-sol`). Solutions are dumped to the user's working dir on completion.

## Solution constraints

Reference solutions follow this priority order: (1) pass tests, (2) good runtime perf, (3) modern Rust 2024 idioms, (4) clean, (5) readable. **No trivializing built-ins**: when the exercise teaches an algorithm, that algorithm must be hand-written (e.g. `reverse-array` cannot call `Vec::reverse`; `sort-colors` cannot call `.sort_unstable`; `count-set-bits` cannot call `.count_ones`). Scaffolding helpers (`HashMap`, `BinaryHeap`, sorting as a preprocessing step for `4sum`, etc.) are fine.

## Solution status

201/276 reference solutions pass their tests end-to-end. 69 carry a `FIXME` for upstream test-file bugs that can't be fixed without modifying `tests/solution.rs`:

- 25 `todo!()` test stubs (no real tests defined)
- 12 i32/i128 overflow in test literals
- 10 `build_tree` BFS-helper infinite loop on certain inputs
- 7 Rust 2024 `vec![]` type-inference failures
- 3 float literals where `i32` is expected
- ~12 other (test-data inconsistencies, algorithmic dispute, single-ulp f64 precision in `Vec<f64>` `assert_eq!`)

These are content-side defects, not solver defects. Fixing them is a separate pass that requires editing test files.

## Init flow

`rustlings-neetcode init` materializes `./rustlings-neetcode/` with every exercise crate. Solution crates are dumped lazily on first completion. Workspace `Cargo.toml` uses glob members `exercises/*` and `solutions/*`.
