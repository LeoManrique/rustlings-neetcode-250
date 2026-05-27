use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExerciseInfo<'a> {
    folder: &'a str,
}

#[derive(Deserialize)]
struct InfoFile<'a> {
    #[serde(borrow)]
    exercises: Vec<ExerciseInfo<'a>>,
}

#[proc_macro]
pub fn include_files(_: TokenStream) -> TokenStream {
    // Remove `\r` on Windows
    let info_file = String::from_utf8(
        include_bytes!("../info.toml")
            .iter()
            .copied()
            .filter(|c| *c != b'\r')
            .collect(),
    )
    .expect("Failed to parse `info.toml` as UTF8");
    let exercises = toml::de::from_str::<InfoFile>(&info_file)
        .expect("Failed to parse `info.toml`")
        .exercises;

    let folders = exercises.iter().map(|e| e.folder.to_string());
    let exercise_cargo = exercises
        .iter()
        .map(|e| format!("../exercises/{}/Cargo.toml", e.folder));
    let exercise_lib = exercises
        .iter()
        .map(|e| format!("../exercises/{}/src/lib.rs", e.folder));
    let exercise_tests = exercises
        .iter()
        .map(|e| format!("../exercises/{}/tests/solution.rs", e.folder));
    let exercise_readme = exercises
        .iter()
        .map(|e| format!("../exercises/{}/README.md", e.folder));
    let solution_cargo = exercises
        .iter()
        .map(|e| format!("../solutions/{}/Cargo.toml", e.folder));
    let solution_lib = exercises
        .iter()
        .map(|e| format!("../solutions/{}/src/lib.rs", e.folder));
    let solution_tests = exercises
        .iter()
        .map(|e| format!("../solutions/{}/tests/solution.rs", e.folder));

    quote! {
        EmbeddedFiles {
            info_file: #info_file,
            exercise_files: &[#(
                ExerciseFiles {
                    folder: #folders,
                    exercise_cargo_toml: include_bytes!(#exercise_cargo),
                    initial_src: include_bytes!(#exercise_lib),
                    tests: include_bytes!(#exercise_tests),
                    readme: include_bytes!(#exercise_readme),
                    solution_cargo_toml: include_bytes!(#solution_cargo),
                    solution_src: include_bytes!(#solution_lib),
                    solution_tests: include_bytes!(#solution_tests),
                }
            ),*],
        }
    }
    .into()
}
