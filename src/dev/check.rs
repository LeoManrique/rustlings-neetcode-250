use anyhow::{Context, Error, Result, anyhow, bail};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::{self, read_dir},
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
    thread,
};

use crate::{
    CURRENT_FORMAT_VERSION,
    cmd::CmdRunner,
    exercise::{OUTPUT_CAPACITY, RunnableExercise},
    info_file::InfoFile,
    term::ProgressCounter,
};

const MAX_N_EXERCISES: usize = 999;
const MAX_SLUG_LEN: usize = 64;

// A char that isn't allowed in a Cargo crate name (slug) or folder name.
fn forbidden_slug_char(input: &str) -> Option<char> {
    input
        .chars()
        .find(|c| !c.is_ascii_alphanumeric() && *c != '-' && *c != '_')
}

fn forbidden_folder_char(input: &str) -> Option<char> {
    input
        .chars()
        .find(|c| !c.is_ascii_alphanumeric() && *c != '-' && *c != '_' && *c != '.')
}

// Validate each entry in `info.toml`, ensure files exist, return the set of folder names.
fn check_info_file_exercises(info_file: &InfoFile) -> Result<HashSet<String>> {
    let mut slugs = HashSet::with_capacity(info_file.exercises.len());
    let mut folders = HashSet::with_capacity(info_file.exercises.len());

    for exercise_info in &info_file.exercises {
        let slug = exercise_info.slug.as_str();
        if slug.is_empty() {
            bail!("Found an empty slug in `info.toml`");
        }
        if slug.len() > MAX_SLUG_LEN {
            bail!(
                "The length of slug `{slug}` is bigger than the maximum {MAX_SLUG_LEN}"
            );
        }
        if let Some(c) = forbidden_slug_char(slug) {
            bail!("Char `{c}` in slug `{slug}` is not allowed (use [a-z0-9_-])");
        }
        if slug.starts_with(|c: char| c.is_ascii_digit()) {
            bail!("Slug `{slug}` cannot start with a digit (Cargo crate name rule)");
        }

        let folder = exercise_info.folder.as_str();
        if folder.is_empty() {
            bail!("Empty `folder` for slug `{slug}`");
        }
        if let Some(c) = forbidden_folder_char(folder) {
            bail!("Char `{c}` in folder `{folder}` is not allowed");
        }

        if !slugs.insert(slug.to_string()) {
            bail!("Duplicate slug `{slug}` in `info.toml`");
        }
        if !folders.insert(folder.to_string()) {
            bail!("Duplicate folder `{folder}` in `info.toml`");
        }

        // Required files.
        for rel in [
            "Cargo.toml",
            "src/lib.rs",
            "tests/solution.rs",
            "README.md",
        ] {
            let p = format!("exercises/{folder}/{rel}");
            if !Path::new(&p).is_file() {
                bail!("Missing required file `{p}` for slug `{slug}`");
            }
        }

        // Solution required files.
        for rel in ["Cargo.toml", "src/lib.rs", "tests/solution.rs"] {
            let p = format!("solutions/{folder}/{rel}");
            if !Path::new(&p).is_file() {
                bail!("Missing required solution file `{p}` for slug `{slug}`");
            }
        }
    }

    Ok(folders)
}

// Make sure every directory under `exercises/` and `solutions/` corresponds to an info.toml entry.
fn check_unexpected_dirs(parent: &str, expected_folders: &HashSet<String>) -> Result<()> {
    for entry in read_dir(parent)
        .with_context(|| format!("Failed to open the `{parent}` directory"))?
    {
        let entry = entry.with_context(|| format!("Failed to read the `{parent}` directory"))?;
        let path = entry.path();
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !expected_folders.contains(name) {
            return Err(anyhow!(
                "Unexpected directory `{}`. Add an entry to info.toml or delete it.",
                path.display()
            ));
        }
    }
    Ok(())
}

fn check_exercises_unsolved(
    info_file: &'static InfoFile,
    cmd_runner: &'static CmdRunner,
) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(b"Running all exercises to check that they aren't already solved...\n")?;

    let handles = info_file
        .exercises
        .iter()
        .filter_map(|exercise_info| {
            if exercise_info.skip_check_unsolved {
                return None;
            }

            Some(
                thread::Builder::new()
                    .spawn(|| exercise_info.run_exercise(None, cmd_runner))
                    .map(|handle| (exercise_info.slug.as_str(), handle)),
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to spawn a thread to check if an exercise is already solved")?;

    let mut progress_counter = ProgressCounter::new(&mut stdout, handles.len())?;

    for (slug, handle) in handles {
        let Ok(result) = handle.join() else {
            bail!("Panic while trying to run the exercise {slug}");
        };

        match result {
            Ok(true) => {
                bail!(
                    "The exercise {slug} is already solved.\n\
                     {SKIP_CHECK_UNSOLVED_HINT}",
                )
            }
            Ok(false) => (),
            Err(e) => return Err(e),
        }

        progress_counter.increment()?;
    }

    Ok(())
}

fn check_exercises(info_file: &'static InfoFile, cmd_runner: &'static CmdRunner) -> Result<()> {
    match info_file.format_version.cmp(&CURRENT_FORMAT_VERSION) {
        Ordering::Less => bail!(
            "`format_version` < {CURRENT_FORMAT_VERSION} (supported version)\n\
             Please migrate to the latest format version"
        ),
        Ordering::Greater => bail!(
            "`format_version` > {CURRENT_FORMAT_VERSION} (supported version)\n\
             Try updating the rustlings-neetcode program"
        ),
        Ordering::Equal => (),
    }

    let handle = thread::Builder::new()
        .spawn(move || check_exercises_unsolved(info_file, cmd_runner))
        .context("Failed to spawn a thread to check if any exercise is already solved")?;

    let folders = check_info_file_exercises(info_file)?;
    check_unexpected_dirs("exercises", &folders)?;
    check_unexpected_dirs("solutions", &folders)?;

    handle.join().unwrap()
}

enum SolutionCheck {
    Success,
    RunFailure { output: Vec<u8> },
    Err(Error),
}

fn check_solutions(
    info_file: &'static InfoFile,
    cmd_runner: &'static CmdRunner,
) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(b"Running all solutions...\n")?;

    let handles = info_file
        .exercises
        .iter()
        .map(|exercise_info| {
            thread::Builder::new().spawn(move || {
                let mut output = Vec::with_capacity(OUTPUT_CAPACITY);
                match exercise_info.run_solution(Some(&mut output), cmd_runner) {
                    Ok(true) => SolutionCheck::Success,
                    Ok(false) => SolutionCheck::RunFailure { output },
                    Err(e) => SolutionCheck::Err(e),
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to spawn a thread to check a solution")?;

    let mut fmt_cmd = Command::new("rustfmt");
    fmt_cmd
        .arg("--check")
        .arg("--edition")
        .arg("2024")
        .arg("--color")
        .arg("always")
        .stdin(Stdio::null());

    let mut progress_counter = ProgressCounter::new(&mut stdout, handles.len())?;

    for (exercise_info, handle) in info_file.exercises.iter().zip(handles) {
        let Ok(check_result) = handle.join() else {
            bail!(
                "Panic while trying to run the solution of the exercise {}",
                exercise_info.slug,
            );
        };

        match check_result {
            SolutionCheck::Success => {
                fmt_cmd.arg(exercise_info.sol_path());
            }
            SolutionCheck::RunFailure { output } => {
                drop(progress_counter);
                stdout.write_all(&output)?;
                bail!(
                    "Running the solution of the exercise {} failed with the error above",
                    exercise_info.slug,
                );
            }
            SolutionCheck::Err(e) => return Err(e),
        }

        progress_counter.increment()?;
    }

    if !fmt_cmd
        .status()
        .context("Failed to run `rustfmt` on all solution files")?
        .success()
    {
        bail!("Some solutions aren't formatted. Run `rustfmt` on them");
    }

    Ok(())
}

pub fn check(_require_solutions: bool) -> Result<()> {
    let info_file = InfoFile::parse()?;

    if info_file.exercises.len() > MAX_N_EXERCISES {
        bail!("The maximum number of exercises is {MAX_N_EXERCISES}");
    }

    // Leaking is fine since they are used until the end of the program.
    let cmd_runner = Box::leak(Box::new(CmdRunner::build()?));
    let info_file = Box::leak(Box::new(info_file));

    check_exercises(info_file, cmd_runner)?;
    check_solutions(info_file, cmd_runner)?;

    println!("Everything looks fine!");

    Ok(())
}

const SKIP_CHECK_UNSOLVED_HINT: &str = "If this is an introduction exercise that is intended to be already solved, add `skip_check_unsolved = true` to the exercise's metadata in the `info.toml` file";
