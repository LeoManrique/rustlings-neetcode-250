# Technical

## Workspace topology

Top-level `Cargo.toml` is the workspace root with `members = ["rustlings-macros", "exercises/*", "solutions/*"]` plus `default-members = ["."]` so `cargo build` only builds the CLI. `dev/Cargo.toml` is reused as the in-init template (verbatim minus its leading comment).

## Run pipeline

`Exercise::run` invokes `cargo test -p <slug> --test solution` then `cargo clippy -p <slug> --tests` (with `-D warnings` when `strict_clippy = true`). Solution checking uses package `<slug>-sol`. `--target-dir` is honored under `cfg(debug_assertions)` only.

## Embedded content

`rustlings-macros::include_files!()` walks `exercises/<folder>/` and `solutions/<folder>/` at compile time, emitting per-folder `ExerciseFiles { exercise_cargo_toml, initial_src, tests, readme, solution_cargo_toml, solution_src, solution_tests }`. `init` writes the exercise files immediately; solution files are written on first completion (`current_solution_path` is no longer gated by `debug_assertions`).

## Exercise → file mapping (watch mode)

`src/watch/notify_event.rs` maps a save event to its exercise by walking the path upward to find `<folder>/src/lib.rs` and matching `folder` against `info.toml`. Edits under `tests/` or `Cargo.toml` are ignored.

## info.toml schema

`ExerciseInfo` uses owned `String` fields (slug/folder/title/difficulty/category/hint) to handle escape sequences and Unicode in problem text. `pub struct Solution;` is intentional: making the type a public API exempts it from `dead_code` without a global lint suppression.

## Why `pub struct Solution;`

LeetCode convention. The tests reference `Solution::<method>(...)` via `include!("../src/lib.rs")`, but `cargo clippy --tests` would otherwise warn `dead_code` on the unused-in-lib-context `Solution`. Marking it `pub` makes it part of the crate's public surface and silences the lint without disabling `dead_code` repo-wide.

## Helpers exposed by solutions

Helpers like `ListNode`, `TreeNode`, weighted union-find structs, etc., are defined with `pub` at module scope so `tests/solution.rs` (which textually includes `src/lib.rs`) can name them when constructing fixtures.
