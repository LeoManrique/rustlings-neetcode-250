use anyhow::{Context, Result, bail};
use crossterm::{
    QueueableCommand,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
};
use serde::Deserialize;
use std::{
    env::{current_dir, set_current_dir},
    fs::{self, create_dir},
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

use crate::{
    embedded::EMBEDDED_FILES, info_file::InfoFile, term::press_enter_prompt,
};

#[derive(Deserialize)]
struct CargoLocateProject<'a> {
    #[serde(borrow)]
    root: &'a Path,
}

pub fn init() -> Result<()> {
    let project_dir = Path::new("rustlings-neetcode");
    if project_dir.exists() {
        bail!(PROJECT_DIR_ALREADY_EXISTS_ERR);
    }

    let locate_project_output = Command::new("cargo")
        .arg("locate-project")
        .arg("-q")
        .arg("--workspace")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .context(
            "Failed to run the command `cargo locate-project …`\n\
             Did you already install Rust?\n\
             Try running `cargo --version` to diagnose the problem.",
        )?;

    if !Command::new("cargo")
        .arg("clippy")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Failed to run the command `cargo clippy --version`")?
        .success()
    {
        bail!(
            "Clippy, the official Rust linter, is missing.\n\
             Please install it first before initializing rustlings-neetcode."
        )
    }

    let mut stdout = io::stdout().lock();
    let mut init_git = true;

    if locate_project_output.status.success() {
        if Path::new("exercises").exists() && Path::new("solutions").exists() {
            bail!(IN_INITIALIZED_DIR_ERR);
        }

        let workspace_manifest =
            serde_json::de::from_slice::<CargoLocateProject>(&locate_project_output.stdout)
                .context(
                    "Failed to read the field `root` from the output of `cargo locate-project …`",
                )?
                .root;

        let workspace_manifest_content = fs::read_to_string(workspace_manifest)
            .with_context(|| format!("Failed to read the file {}", workspace_manifest.display()))?;
        if !workspace_manifest_content.contains("[workspace]")
            && !workspace_manifest_content.contains("workspace.")
        {
            bail!(
                "The current directory is already part of a Cargo project.\n\
                 Please initialize rustlings-neetcode in a different directory"
            );
        }

        stdout.write_all(b"This command will create the directory `rustlings-neetcode/` as a member of this Cargo workspace.\n\
                           Press ENTER to continue ")?;
        press_enter_prompt(&mut stdout)?;

        // Use `cargo new` to register the new directory in the parent workspace.
        let status = Command::new("cargo")
            .arg("new")
            .arg("-q")
            .arg("--vcs")
            .arg("none")
            .arg("rustlings-neetcode")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()?;
        if !status.success() {
            bail!(
                "Failed to initialize a new Cargo workspace member.\n\
                 Please initialize rustlings-neetcode in a different directory"
            );
        }

        stdout.write_all(b"The directory `rustlings-neetcode` has been added to `workspace.members` in the parent `Cargo.toml`.\n")?;
        fs::remove_dir_all(project_dir)
            .context("Failed to remove the temporary directory `rustlings-neetcode/`")?;
        init_git = false;
    } else {
        stdout.write_all(b"This command will create the directory `rustlings-neetcode/` which will contain the exercises.\n\
                           Press ENTER to continue ")?;
        press_enter_prompt(&mut stdout)?;
    }

    create_dir(project_dir).context("Failed to create the `rustlings-neetcode/` directory")?;
    set_current_dir(project_dir)
        .context("Failed to change the current directory to `rustlings-neetcode/`")?;

    let info_file = InfoFile::parse()?;
    EMBEDDED_FILES
        .init_exercises_dir(&info_file.exercises)
        .context("Failed to initialize the `exercises/` and `solutions/` directories")?;

    // The workspace `Cargo.toml` is the dev template verbatim, minus its first comment line.
    let template = include_str!("../dev-Cargo.toml");
    let newline_ind = template
        .as_bytes()
        .iter()
        .position(|c| *c == b'\n')
        .context("The embedded `Cargo.toml` is empty or contains only one line")?;
    let body = template
        .get(newline_ind + 1..)
        .context("The embedded `Cargo.toml` contains only one line")?;
    fs::write("Cargo.toml", body)
        .context("Failed to create the file `rustlings-neetcode/Cargo.toml`")?;

    fs::write("rust-analyzer.toml", RUST_ANALYZER_TOML)
        .context("Failed to create the file `rustlings-neetcode/rust-analyzer.toml`")?;

    fs::write(".gitignore", GITIGNORE)
        .context("Failed to create the file `rustlings-neetcode/.gitignore`")?;

    create_dir(".vscode").context("Failed to create the directory `rustlings-neetcode/.vscode`")?;
    fs::write(".vscode/extensions.json", VS_CODE_EXTENSIONS_JSON)
        .context("Failed to create the file `rustlings-neetcode/.vscode/extensions.json`")?;
    fs::write(".vscode/settings.json", VS_CODE_SETTINGS_JSON)
        .context("Failed to create the file `rustlings-neetcode/.vscode/settings.json`")?;

    if init_git && let Ok(dir) = current_dir() {
        let mut dir = dir.as_path();

        loop {
            if dir.join(".git").exists() || dir.join(".jj").exists() {
                break;
            }

            if let Some(parent) = dir.parent() {
                dir = parent;
            } else {
                // Ignore any Git error because Git initialization is not required.
                let _ = Command::new("git")
                    .arg("init")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
                break;
            }
        }
    }

    stdout.queue(SetForegroundColor(Color::Green))?;
    stdout.write_all("Initialization done ✓".as_bytes())?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b"\n\n")?;

    stdout.queue(SetAttribute(Attribute::Bold))?;
    stdout.write_all(POST_INIT_MSG)?;
    stdout.queue(ResetColor)?;

    Ok(())
}

pub const RUST_ANALYZER_TOML: &[u8] = br#"check.command = "clippy"
check.extraArgs = ["--tests"]
cargo.targetDir = true
"#;

const GITIGNORE: &[u8] = b"Cargo.lock
target/
.vscode/
";

pub const VS_CODE_EXTENSIONS_JSON: &[u8] = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;

/// Tell rust-analyzer to load the workspace at the project root. Glob members in
/// `Cargo.toml` mean every exercise and solution crate is discovered automatically.
pub const VS_CODE_SETTINGS_JSON: &[u8] = br#"{
  "rust-analyzer.linkedProjects": ["Cargo.toml"],
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.extraArgs": ["--tests"]
}
"#;

const IN_INITIALIZED_DIR_ERR: &str = "It looks like rustlings-neetcode is already initialized in this directory.

If you already initialized rustlings-neetcode, run the command `rustlings-neetcode` for instructions on getting started with the exercises.
Otherwise, please run `rustlings-neetcode init` again in a different directory.";

const PROJECT_DIR_ALREADY_EXISTS_ERR: &str =
    "A directory with the name `rustlings-neetcode` already exists in the current directory.
You probably already initialized rustlings-neetcode.
Run `cd rustlings-neetcode`
Then run `rustlings-neetcode` again";

const POST_INIT_MSG: &[u8] = b"Run `cd rustlings-neetcode` to go into the generated directory.
Then run `rustlings-neetcode` to get started.
";
