use anyhow::{Context, Result, bail};
use std::{
    env::set_current_dir,
    fs::{self, create_dir},
    path::Path,
    process::Command,
};

use crate::{CURRENT_FORMAT_VERSION, init::RUST_ANALYZER_TOML};

fn create_rel_dir(dir_name: &str, current_dir: &str) -> Result<()> {
    create_dir(dir_name)
        .with_context(|| format!("Failed to create the directory {current_dir}/{dir_name}"))?;
    println!("Created the directory {current_dir}/{dir_name}");
    Ok(())
}

fn write_rel_file<C>(file_name: &str, current_dir: &str, content: C) -> Result<()>
where
    C: AsRef<[u8]>,
{
    fs::write(file_name, content)
        .with_context(|| format!("Failed to create the file {current_dir}/{file_name}"))?;
    println!("Created the file      {current_dir}/{file_name}");
    Ok(())
}

pub fn new(path: &Path, no_git: bool) -> Result<()> {
    let dir_path_str = path.to_string_lossy();

    create_dir(path).with_context(|| format!("Failed to create the directory {dir_path_str}"))?;
    println!("Created the directory {dir_path_str}");

    set_current_dir(path)
        .with_context(|| format!("Failed to set {dir_path_str} as the current directory"))?;

    if !no_git
        && !Command::new("git")
            .arg("init")
            .status()
            .context("Failed to run `git init`")?
            .success()
    {
        bail!("`git init` didn't run successfully. See the possible error message above");
    }

    write_rel_file(".gitignore", &dir_path_str, GITIGNORE)?;

    create_rel_dir("exercises", &dir_path_str)?;
    create_rel_dir("solutions", &dir_path_str)?;

    write_rel_file(
        "info.toml",
        &dir_path_str,
        format!(
            "{INFO_FILE_BEFORE_FORMAT_VERSION}{CURRENT_FORMAT_VERSION}{INFO_FILE_AFTER_FORMAT_VERSION}"
        ),
    )?;

    write_rel_file("Cargo.toml", &dir_path_str, CARGO_TOML)?;

    write_rel_file("README.md", &dir_path_str, README)?;

    write_rel_file("rust-analyzer.toml", &dir_path_str, RUST_ANALYZER_TOML)?;

    create_rel_dir(".vscode", &dir_path_str)?;
    write_rel_file(
        ".vscode/extensions.json",
        &dir_path_str,
        crate::init::VS_CODE_EXTENSIONS_JSON,
    )?;

    println!("\nInitialization done ✓");

    Ok(())
}

pub const GITIGNORE: &[u8] = b"Cargo.lock
target/
.vscode/
!.vscode/extensions.json
";

const INFO_FILE_BEFORE_FORMAT_VERSION: &str =
    "# Format version of this `info.toml`.\n\
     format_version = ";

const INFO_FILE_AFTER_FORMAT_VERSION: &str = r#"

# Optional multi-line message to be shown to users when just starting with the exercises.
welcome_message = """Welcome to these community rustlings-neetcode exercises."""

# Optional multi-line message to be shown to users after finishing all exercises.
final_message = """We hope you enjoyed!"""

# Repeat this section for every exercise.
[[exercises]]
# Cargo crate name (also used as the exercise's unique slug).
slug = "???"

# Directory name under `exercises/` and `solutions/`. May contain `.`/`-`/`_`.
folder = "???"

# Human-readable title shown in the UI.
title = "???"

# Easy / Medium / Hard.
difficulty = "Easy"

# Category label (e.g. "Arrays & Hashing").
category = "???"

# Optional hint shown on `h`.
hint = ""

# Set to `true` to deny all Clippy warnings.
# strict_clippy = false
"#;

const CARGO_TOML: &[u8] = br#"# Don't edit the `members` list manually! Use `rustlings-neetcode dev update`.
[workspace]
resolver = "2"
members = []

[workspace.package]
edition = "2024"
publish = false

[profile.release]
panic = "abort"

[profile.dev]
panic = "abort"
"#;

const README: &str = "# rustlings-neetcode community pack

Welcome to a community-built rustlings-neetcode exercise pack.

First, install rustlings-neetcode following the official instructions.

Then, clone this repository, open a terminal in this directory and run `rustlings-neetcode`
to get started with the exercises.
";
