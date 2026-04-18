use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let web_dir = manifest_dir.parent().expect("workspace root").join("web");
    let shared_dir = manifest_dir
        .parent()
        .expect("workspace root")
        .join("shared");
    let assets_dir = manifest_dir.join("assets");

    watch_path(&web_dir.join("Cargo.toml"));
    watch_path(&web_dir.join("Trunk.toml"));
    watch_path(&web_dir.join("index.html"));
    watch_path(&web_dir.join("src"));
    watch_path(&shared_dir.join("Cargo.toml"));
    watch_path(&shared_dir.join("src"));

    std::fs::create_dir_all(&assets_dir).expect("create web asset directory");

    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let mut args = vec!["build"];

    if profile == "release" {
        args.push("--release");
    }

    let mut trunk_cmd = Command::new("trunk");
    trunk_cmd.current_dir(&web_dir).args(&args).args([
        "--dist",
        assets_dir
            .to_str()
            .expect("service/assets path should be valid utf-8"),
        "--public-url",
        "/",
    ]);

    // Set a separate target directory for the internal WASM build to avoid lock contention
    let workspace_target = manifest_dir.parent().unwrap().join("target");
    trunk_cmd.env("CARGO_TARGET_DIR", workspace_target.join("trunk-wasm"));

    // Clear jobserver environment variables to prevent deadlock in recursive cargo calls
    trunk_cmd.env_remove("CARGO_MAKEFLAGS");
    trunk_cmd.env_remove("MAKEFLAGS");
    trunk_cmd.env_remove("MFLAGS");

    let status = trunk_cmd.status().expect("failed to run trunk build");

    if !status.success() {
        panic!("trunk build failed with status {status}");
    }
}

fn watch_path(path: &Path) {
    if path.is_dir() {
        println!("cargo:rerun-if-changed={}", path.display());
        let entries = fs::read_dir(path).unwrap_or_else(|err| {
            panic!("failed to read watched directory {}: {err}", path.display());
        });

        for entry in entries {
            let entry = entry.unwrap_or_else(|err| {
                panic!("failed to read watched entry in {}: {err}", path.display());
            });
            watch_path(&entry.path());
        }
    } else {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}
