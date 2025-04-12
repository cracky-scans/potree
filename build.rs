use std::{
    fs::{self},
    path::Path,
    process::Command,
};

fn main() {
    // Tell Cargo to rerun this build script if package.json changes
    println!("cargo:rerun-if-changed=potree/package.json");

    // Run `npm install` which will also automatically build.
    run_command("npm", &["install"], "potree");

    // Copy files to asset dir
    fs::remove_dir_all("built_assets").expect("unable to remove previous assets");
    copy_dir_all("potree/build", "built_assets/build");
    copy_dir_all("potree/libs", "built_assets/libs");
}

fn run_command(command: &str, args: &[&str], dir: &str) {
    let status = Command::new(command)
        .args(args)
        .current_dir(dir)
        .status()
        .unwrap_or_else(|e| {
            panic!("Failed to run `{}`: {}", command, e);
        });

    if !status.success() {
        panic!("Command `{}` failed with status: {}", command, status);
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    fs::create_dir_all(&dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let ty = entry.file_type().unwrap();
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()));
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name())).unwrap();
        }
    }
}
