use anyhow::{Context, Error, Result};
use std::{
    fs::{self, create_dir, create_dir_all},
    io,
};

use crate::info_file::ExerciseInfo;

/// Contains all embedded files.
pub static EMBEDDED_FILES: EmbeddedFiles = rustlings_macros::include_files!();

/// All files belonging to one exercise (and its solution).
pub struct ExerciseFiles {
    /// Directory name under `exercises/` and `solutions/`.
    pub folder: &'static str,
    pub exercise_cargo_toml: &'static [u8],
    /// Starter content for `exercises/<folder>/src/lib.rs`.
    pub initial_src: &'static [u8],
    /// Content for `exercises/<folder>/tests/solution.rs`.
    pub tests: &'static [u8],
    /// Content for `exercises/<folder>/README.md`.
    pub readme: &'static [u8],
    pub solution_cargo_toml: &'static [u8],
    /// Solved content for `solutions/<folder>/src/lib.rs`.
    pub solution_src: &'static [u8],
    pub solution_tests: &'static [u8],
}

fn create_dir_if_not_exists(path: &str) -> Result<()> {
    if let Err(e) = create_dir(path)
        && e.kind() != io::ErrorKind::AlreadyExists
    {
        return Err(Error::from(e).context(format!("Failed to create the directory {path}")));
    }

    Ok(())
}

/// All embedded files.
pub struct EmbeddedFiles {
    /// The content of the `info.toml` file.
    pub info_file: &'static str,
    pub exercise_files: &'static [ExerciseFiles],
}

impl EmbeddedFiles {
    /// Write every exercise crate (Cargo.toml, src/lib.rs starter, tests/solution.rs, README.md)
    /// under `exercises/`, plus a placeholder solution crate at `solutions/<folder>/`
    /// that contains the puzzle skeleton so the workspace loads even before any
    /// exercise is solved.
    pub fn init_exercises_dir(&self, _exercise_infos: &[ExerciseInfo]) -> Result<()> {
        create_dir_if_not_exists("exercises")?;
        create_dir_if_not_exists("solutions")?;

        for files in self.exercise_files {
            write_exercise_crate(files)?;
            write_solution_placeholder(files)?;
        }

        Ok(())
    }

    /// Write a fresh copy of an exercise crate to disk (used by reset, init, and when re-creating
    /// missing scaffolding around an exercise).
    pub fn write_exercise_to_disk(&self, exercise_ind: usize, _path: &str) -> Result<()> {
        let files = &self.exercise_files[exercise_ind];
        write_exercise_crate(files)
    }

    /// Write the solution crate to disk and return the path of its `src/lib.rs`.
    pub fn write_solution_to_disk(
        &self,
        exercise_ind: usize,
        _exercise_name: &str,
    ) -> Result<String> {
        create_dir_if_not_exists("solutions")?;

        let files = &self.exercise_files[exercise_ind];
        let base = format!("solutions/{}", files.folder);
        create_dir_all(format!("{base}/src"))
            .with_context(|| format!("Failed to create {base}/src"))?;
        create_dir_all(format!("{base}/tests"))
            .with_context(|| format!("Failed to create {base}/tests"))?;

        let cargo_path = format!("{base}/Cargo.toml");
        fs::write(&cargo_path, files.solution_cargo_toml)
            .with_context(|| format!("Failed to write {cargo_path}"))?;

        let tests_path = format!("{base}/tests/solution.rs");
        fs::write(&tests_path, files.solution_tests)
            .with_context(|| format!("Failed to write {tests_path}"))?;

        let lib_path = format!("{base}/src/lib.rs");
        fs::write(&lib_path, files.solution_src)
            .with_context(|| format!("Failed to write {lib_path}"))?;

        Ok(lib_path)
    }
}

/// Write the solution crate scaffolding (Cargo.toml + tests/solution.rs) plus a
/// placeholder `src/lib.rs` that mirrors the exercise's starter. The placeholder
/// makes the workspace valid; the real solution is dumped over it on completion.
fn write_solution_placeholder(files: &ExerciseFiles) -> Result<()> {
    let base = format!("solutions/{}", files.folder);
    create_dir_all(format!("{base}/src"))
        .with_context(|| format!("Failed to create {base}/src"))?;
    create_dir_all(format!("{base}/tests"))
        .with_context(|| format!("Failed to create {base}/tests"))?;

    fs::write(format!("{base}/Cargo.toml"), files.solution_cargo_toml)
        .with_context(|| format!("Failed to write {base}/Cargo.toml"))?;
    fs::write(format!("{base}/tests/solution.rs"), files.solution_tests)
        .with_context(|| format!("Failed to write {base}/tests/solution.rs"))?;
    fs::write(format!("{base}/src/lib.rs"), files.initial_src)
        .with_context(|| format!("Failed to write {base}/src/lib.rs"))?;

    Ok(())
}

fn write_exercise_crate(files: &ExerciseFiles) -> Result<()> {
    let base = format!("exercises/{}", files.folder);
    create_dir_all(format!("{base}/src"))
        .with_context(|| format!("Failed to create {base}/src"))?;
    create_dir_all(format!("{base}/tests"))
        .with_context(|| format!("Failed to create {base}/tests"))?;

    let cargo_path = format!("{base}/Cargo.toml");
    fs::write(&cargo_path, files.exercise_cargo_toml)
        .with_context(|| format!("Failed to write {cargo_path}"))?;

    let tests_path = format!("{base}/tests/solution.rs");
    fs::write(&tests_path, files.tests)
        .with_context(|| format!("Failed to write {tests_path}"))?;

    let readme_path = format!("{base}/README.md");
    fs::write(&readme_path, files.readme)
        .with_context(|| format!("Failed to write {readme_path}"))?;

    let lib_path = format!("{base}/src/lib.rs");
    fs::write(&lib_path, files.initial_src)
        .with_context(|| format!("Failed to write {lib_path}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    struct ExerciseInfo<'a> {
        folder: &'a str,
    }

    #[derive(Deserialize)]
    struct InfoFile<'a> {
        #[serde(borrow)]
        exercises: Vec<ExerciseInfo<'a>>,
    }

    #[test]
    fn folders_match() {
        let exercises = toml::de::from_str::<InfoFile>(EMBEDDED_FILES.info_file)
            .expect("Failed to parse `info.toml`")
            .exercises;

        assert_eq!(exercises.len(), EMBEDDED_FILES.exercise_files.len());

        for (info, files) in exercises.iter().zip(EMBEDDED_FILES.exercise_files) {
            assert_eq!(info.folder, files.folder);
        }
    }
}
