# Roadmap

## Done

- Fork rustlings → rustlings-neetcode with per-crate exercise model
- 276 exercises + scaffolds embedded, flat `<NNN-slug>/` layout
- `init` materializes user workspace with lazy solution dump on completion
- 276/276 reference solutions pass their test suites
- Repaired all 69 upstream test-file FIXMEs (`todo!()` stubs, integer overflow, `build_tree` infinite loop, `vec![]` type inference, float-in-BST literals, test-data inconsistencies, missing reference implementations, `MountainArray` wrapper)

## Next

1. **Watch-mode polish**: verify reset (`x`) restores the embedded `initial_src` cleanly across all 276 crates; confirm `c` (check-all) reports green across the whole suite now that every solution passes.
2. **`cargo dev check` relaxation**: dev check currently runs strict clippy on test files and trips on style lints we can't fix (e.g. `assert_eq!(..., true)`). Either downgrade those lints in `[workspace.lints]` or skip clippy on `tests/`.
3. **Workspace path quirk**: `dev/Cargo.toml` references `../exercises/*` and `../solutions/*`, but cargo resolves member globs relative to the manifest's own directory. The current workflow requires temporary symlinks (`dev/exercises -> ../exercises`) for `cargo dev` to run. Either inline the member list (no globs) or pre-create stable symlinks at init.

## Deferred

- `cargo dev new` for adding problems beyond the 276 — low priority since the initial import is one-shot.
- Distribution: cargo install / Homebrew formula.
