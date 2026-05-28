# rustlings-neetcode

A rustlings-style interactive CLI for solving the [NeetCode 250](https://neetcode.io/) problems in Rust.

Each problem is its own Cargo crate with an integration test suite. `rustlings-neetcode`
watches the exercise file you're currently working on, re-runs `cargo test` on save, and
advances when the tests pass.

## Layout

```
exercises/<NNN-slug>/
├── Cargo.toml
├── README.md            problem statement
├── src/lib.rs           you edit this
└── tests/solution.rs    integration tests (don't edit)
```

## Build & install

```sh
cargo install --path .       # installs `rustlings-neetcode` to ~/.cargo/bin
```

The binary is standalone — all 276 exercises and reference solutions are
embedded at compile time, so deleting this repo doesn't affect installed copies.

To uninstall: `cargo uninstall rustlings-neetcode`.

## Getting started

```sh
rustlings-neetcode init                # scaffolds a `rustlings-neetcode/` working dir
rustlings-neetcode init my-folder      # or use a custom directory name
cd rustlings-neetcode                  # (or my-folder)
rustlings-neetcode                     # enters watch mode
```

Watch-mode keys: `n` next, `h` hint, `l` list, `c` check-all, `x` reset, `q` quit.

## Provenance

276 exercises, sourced from the
[learn-offline](https://github.com/) neetcode app and seeded with reference
solutions. The migration tool that produced the initial scaffolding has been
removed; future updates to exercise content should be a manual import.

## Development

```sh
cargo build -p rustlings-neetcode    # build the CLI
cargo run -p rustlings-neetcode -- run array-sum
cargo dev check                       # verify every exercise/solution
```
