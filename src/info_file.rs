use anyhow::{Context, Error, Result, bail};
use serde::Deserialize;
use std::{fs, io::ErrorKind};

use crate::embedded::EMBEDDED_FILES;

/// Deserialized from the `info.toml` file.
#[derive(Deserialize)]
pub struct ExerciseInfo {
    /// Unique slug (also the Cargo crate name).
    pub slug: String,
    /// Directory name under `exercises/` and `solutions/`.
    pub folder: String,
    /// Human-readable title shown in the UI.
    pub title: String,
    /// "Easy", "Medium", or "Hard".
    pub difficulty: String,
    /// e.g. "Arrays & Hashing".
    pub category: String,
    /// Hint to show on `h`.
    #[serde(default)]
    pub hint: String,
    /// Deny all Clippy warnings.
    #[serde(default)]
    pub strict_clippy: bool,
    /// The exercise ships already solved. Ignored when checking the all-unsolved invariant.
    #[serde(default)]
    pub skip_check_unsolved: bool,
}

impl ExerciseInfo {
    /// Path to the file the user edits (`exercises/<folder>/src/lib.rs`).
    pub fn path(&self) -> String {
        let mut path = String::with_capacity(24 + self.folder.len());
        path.push_str("exercises/");
        path.push_str(&self.folder);
        path.push_str("/src/lib.rs");
        path
    }

    /// Path to the per-exercise README.
    pub fn readme_path(&self) -> String {
        let mut path = String::with_capacity(24 + self.folder.len());
        path.push_str("exercises/");
        path.push_str(&self.folder);
        path.push_str("/README.md");
        path
    }

    /// Cargo crate name for the solution variant.
    pub fn solution_package(&self) -> String {
        let mut s = String::with_capacity(self.slug.len() + 4);
        s.push_str(&self.slug);
        s.push_str("-sol");
        s
    }
}

impl crate::exercise::RunnableExercise for ExerciseInfo {
    fn package(&self) -> &str {
        &self.slug
    }

    fn folder(&self) -> &str {
        &self.folder
    }

    fn strict_clippy(&self) -> bool {
        self.strict_clippy
    }

    fn sol_path(&self) -> String {
        let mut path = String::with_capacity(24 + self.folder.len());
        path.push_str("solutions/");
        path.push_str(&self.folder);
        path.push_str("/src/lib.rs");
        path
    }
}

/// The deserialized `info.toml` file.
#[derive(Deserialize)]
pub struct InfoFile {
    /// For possible breaking changes in the future for community exercises.
    pub format_version: u8,
    /// Shown to users when starting with the exercises.
    pub welcome_message: Option<String>,
    /// Shown to users after finishing all exercises.
    pub final_message: Option<String>,
    /// List of all exercises.
    pub exercises: Vec<ExerciseInfo>,
}

impl InfoFile {
    /// Official exercises: Parse the embedded `info.toml` file.
    /// Community exercises: Parse the `info.toml` file in the current directory.
    pub fn parse() -> Result<Self> {
        let slf = match fs::read("info.toml") {
            Ok(file_content) => {
                // Remove `\r` on Windows.
                // Leaking is fine since the info file is used until the end of the program.
                let file_content =
                    String::from_utf8(file_content.into_iter().filter(|c| *c != b'\r').collect())
                        .context("Failed to parse `info.toml` as UTF8")?
                        .leak();
                toml::de::from_str::<Self>(file_content)
                    .context("Failed to parse the `info.toml` file")?
            }
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return toml::de::from_str(EMBEDDED_FILES.info_file)
                        .context("Failed to parse the embedded `info.toml` file");
                }

                return Err(Error::from(e).context("Failed to read the `info.toml` file"));
            }
        };

        if slf.exercises.is_empty() {
            bail!("{NO_EXERCISES_ERR}");
        }

        Ok(slf)
    }
}

const NO_EXERCISES_ERR: &str = "There are no exercises yet!
Add at least one exercise before testing.";
