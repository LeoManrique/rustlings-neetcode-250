# Roadmap

## Done

- Fork rustlings → rustlings-neetcode with per-crate exercise model
- 276 exercises + scaffolds embedded, flat `<NNN-slug>/` layout
- `init` materializes user workspace with lazy solution dump on completion
- 201/276 reference solutions verified passing their full test suites

## Next

1. **Fix upstream test bugs** (69 FIXMEs). One pass to edit `exercises/<folder>/tests/solution.rs`:
   - Replace `todo!()` stubs with real tests for designed-class problems (LRU cache, design-twitter, etc.).
   - Bound integer literals to `i32` range or change parameter types where the test intent is `i64`/`i128`.
   - Replace bare `vec![]` with type-annotated empty literals (`Vec::<Vec<i32>>::new()`) under Rust 2024.
   - Fix the `build_tree` BFS helper's infinite loop when its level-order queue drains before vals is exhausted.
   - Switch `assert_eq!` on `Vec<f64>` to approximate comparison (e.g. `177-evaluate-division`).
3. **Watch-mode polish**: verify reset (`x`) restores the embedded `initial_src` cleanly across all 276 crates; confirm `c` (check-all) doesn't deadlock on the tree-helper bug.
4. **`cargo dev check` relaxation**: the dev check currently runs strict clippy on test files and trips on style lints we can't fix (e.g. `assert_eq!(..., true)`). Either downgrade those lints in `[workspace.lints]` or skip clippy on `tests/`.

## Deferred

- `cargo dev new` for adding problems beyond the 276 — low priority since the initial import is one-shot.
- Distribution: cargo install / Homebrew formula. Not in scope until the test-bug pass lands.
