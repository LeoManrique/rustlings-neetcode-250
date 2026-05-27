use anyhow::Result;

/// The generated workspace uses glob members (`exercises/*`, `solutions/*`) so it
/// stays in sync with `info.toml` automatically. Nothing to update here for now.
pub fn update() -> Result<()> {
    println!(
        "The workspace template uses glob members; no manual update needed.\n\
         If you modify `info.toml`, run `cargo dev check` to verify consistency."
    );
    Ok(())
}
