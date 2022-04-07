use std::fs;
use std::path::PathBuf;

const BUILD_RS_RELATIVE_PATH_PREFIX: &str = "../..";

fn main() {
    // If crates/consts is changed (usually caused by an update to a newer minecraft version), generate new data.
    println!(
        "cargo:rerun-if-changed={}/crates/consts",
        BUILD_RS_RELATIVE_PATH_PREFIX
    );
    // If data_generators is changed, generate data using new generators.
    println!(
        "cargo:rerun-if-changed={}/crates/data_generators",
        BUILD_RS_RELATIVE_PATH_PREFIX
    );
    // If feather-data is changed, generate data using new feather-data.
    println!(
        "cargo:rerun-if-changed={}/feather_data",
        BUILD_RS_RELATIVE_PATH_PREFIX
    );

    let project_root = PathBuf::from(BUILD_RS_RELATIVE_PATH_PREFIX)
        .canonicalize()
        .unwrap();

    let build_directory = project_root.join("build");
    if !build_directory.exists() {
        fs::create_dir(&build_directory).unwrap();
    }
    std::env::set_current_dir(build_directory).unwrap();
    data_generators::extract_vanilla_data();

    std::env::set_current_dir(project_root).unwrap();
    data_generators::generate_code();
}
