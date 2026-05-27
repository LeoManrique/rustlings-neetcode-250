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

276/276 reference solutions pass their tests end-to-end. All 69 upstream test-file FIXMEs have been resolved by editing `tests/solution.rs` directly:

- 25 `todo!()` stubs → real tests written against the existing reference API
- 13 i32/i128 overflow in test literals → bounded to i32 or test deleted when overflow was load-bearing
- 10 `build_tree` BFS-helper infinite loop → let-else termination when the level-order queue drains
- 8 Rust 2024 `vec![]` type-inference failures → `Vec::<T>::new()` annotated empties
- 3 float literals in i32-typed BST builders → integer values that preserve test intent
- 10 test-data disputes (alien dictionary, k-closest tie-break, swim-water constraint, etc.) → recomputed expected outputs against the algorithmic spec
- 132-construct-quad-tree, 156-walls-and-gates, 096-copy-list-with-random-pointer → reference implementations written (no Rust scaffold existed)
- 177-evaluate-division → `assert_eq!(Vec<f64>, ...)` rewritten to compare within 1e-5 tolerance
- 227-find-in-mountain-array → local `MountainArray` wrapper added so the problem is testable in isolation

Some upstream tests were deleted when their inputs violated the problem's stated constraints (e.g. `koko-eating-bananas` tests with `h < piles.len()`, `swim-in-rising-water` with duplicate cell values, BST builders with non-integer node values). Deletions are documented in commit messages.

## Init flow

`rustlings-neetcode init` materializes `./rustlings-neetcode/` with every exercise crate. Solution crates are dumped lazily on first completion. Workspace `Cargo.toml` uses glob members `exercises/*` and `solutions/*`.
