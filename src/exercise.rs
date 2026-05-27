use anyhow::Result;
use crossterm::{
    QueueableCommand,
    style::{Attribute, Color, ResetColor, SetAttribute},
};
use std::io::{self, StdoutLock, Write};

use crate::{
    cmd::CmdRunner,
    term::{self, CountedWrite, file_path, terminal_file_link},
};

/// The initial capacity of the output buffer.
pub const OUTPUT_CAPACITY: usize = 1 << 14;

pub fn solution_link_line(
    stdout: &mut StdoutLock,
    solution_path: &str,
    emit_file_links: bool,
) -> io::Result<()> {
    stdout.queue(SetAttribute(Attribute::Bold))?;
    stdout.write_all(b"Solution")?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b" for comparison: ")?;
    file_path(stdout, Color::Cyan, |writer| {
        if emit_file_links && let Some(canonical_path) = term::canonicalize(solution_path) {
            terminal_file_link(writer, solution_path, &canonical_path)
        } else {
            writer.stdout().write_all(solution_path.as_bytes())
        }
    })?;
    stdout.write_all(b"\n")
}

/// See `info_file::ExerciseInfo`
pub struct Exercise {
    /// Crate name and unique identifier (the `slug`).
    pub name: &'static str,
    /// Directory name under `exercises/` and `solutions/`.
    pub folder: &'static str,
    /// `exercises/<folder>/src/lib.rs`.
    pub path: &'static str,
    pub canonical_path: Option<String>,
    pub title: &'static str,
    pub difficulty: &'static str,
    pub category: &'static str,
    pub strict_clippy: bool,
    pub hint: &'static str,
    pub done: bool,
}

impl Exercise {
    pub fn terminal_file_link<'a>(
        &self,
        writer: &mut impl CountedWrite<'a>,
        emit_file_links: bool,
    ) -> io::Result<()> {
        file_path(writer, Color::Blue, |writer| {
            if emit_file_links && let Some(canonical_path) = self.canonical_path.as_deref() {
                terminal_file_link(writer, self.path, canonical_path)
            } else {
                writer.write_str(self.path)
            }
        })
    }

    pub fn solution_package(&self) -> String {
        let mut s = String::with_capacity(self.name.len() + 4);
        s.push_str(self.name);
        s.push_str("-sol");
        s
    }

    pub fn readme_path(&self) -> String {
        let mut path = String::with_capacity(24 + self.folder.len());
        path.push_str("exercises/");
        path.push_str(self.folder);
        path.push_str("/README.md");
        path
    }
}

pub trait RunnableExercise {
    /// Cargo package name to invoke with `-p`.
    fn package(&self) -> &str;
    /// Directory under `exercises/` / `solutions/`.
    fn folder(&self) -> &str;
    fn strict_clippy(&self) -> bool;
    /// Path to the solution source file (`solutions/<folder>/src/lib.rs`).
    fn sol_path(&self) -> String;

    /// Compile, test, and clippy-check a single package (exercise or solution).
    fn run<const FORCE_STRICT_CLIPPY: bool>(
        &self,
        package: &str,
        mut output: Option<&mut Vec<u8>>,
        cmd_runner: &CmdRunner,
    ) -> Result<bool> {
        if let Some(output) = output.as_deref_mut() {
            output.clear();
        }

        // `cargo test` compiles, so no separate build step.
        let output_is_some = output.is_some();
        let mut test_cmd = cmd_runner.cargo("test", package, output.as_deref_mut());
        test_cmd.args(["--test", "solution"]);
        if output_is_some {
            test_cmd.args(["--", "--color", "always", "--format", "pretty"]);
        }
        let test_success = test_cmd.run("cargo test …")?;
        if !test_success {
            return Ok(false);
        }

        // Drop the test output so clippy output is the only thing visible if it errors.
        if let Some(output) = output.as_deref_mut() {
            output.clear();
        }

        let mut clippy_cmd = cmd_runner.cargo("clippy", package, output.as_deref_mut());
        clippy_cmd.args(["--tests"]);
        if FORCE_STRICT_CLIPPY || self.strict_clippy() {
            clippy_cmd.args(["--", "-D", "warnings"]);
        }

        clippy_cmd.run("cargo clippy …")
    }

    /// Compile, check and run the exercise.
    fn run_exercise(&self, output: Option<&mut Vec<u8>>, cmd_runner: &CmdRunner) -> Result<bool> {
        self.run::<false>(self.package(), output, cmd_runner)
    }

    /// Compile, check and run the exercise's solution.
    fn run_solution(&self, output: Option<&mut Vec<u8>>, cmd_runner: &CmdRunner) -> Result<bool> {
        let package = self.package();
        let mut sol_package = String::with_capacity(package.len() + 4);
        sol_package.push_str(package);
        sol_package.push_str("-sol");

        self.run::<true>(&sol_package, output, cmd_runner)
    }
}

impl RunnableExercise for Exercise {
    fn package(&self) -> &str {
        self.name
    }

    fn folder(&self) -> &str {
        self.folder
    }

    fn strict_clippy(&self) -> bool {
        self.strict_clippy
    }

    fn sol_path(&self) -> String {
        let mut path = String::with_capacity(24 + self.folder.len());
        path.push_str("solutions/");
        path.push_str(self.folder);
        path.push_str("/src/lib.rs");
        path
    }
}
