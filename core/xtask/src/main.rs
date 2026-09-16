use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const CRATE_NAME: &str = "gb-core-ffi";
const LIB_NAME: &str = "libgb_core_ffi.a";
const XCFRAMEWORK_PATH: &str = "apps/swift/GameBoyCore/Frameworks/GameBoyCoreFFI.xcframework";
const HEADERS_DIR: &str = "core/gb-core-ffi/include/gb";

// Run `rustup target add aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim` before running this script.
const MACOS_TARGET: &str = "aarch64-apple-darwin";
const IOS_DEVICE_TARGET: &str = "aarch64-apple-ios";
const IOS_SIM_TARGET: &str = "aarch64-apple-ios-sim";
const ALL_TARGETS: &[&str] = &[MACOS_TARGET, IOS_DEVICE_TARGET, IOS_SIM_TARGET];

fn main() -> Result<(), String> {
    let root = workspace_root()?;
    let headers_dir = root.join(HEADERS_DIR);

    for target in ALL_TARGETS {
        build_target(&root, target)?;
    }

    let ios_device_lib = root.join(format!("target/{IOS_DEVICE_TARGET}/release/{LIB_NAME}"));
    let ios_sim_lib = root.join(format!("target/{IOS_SIM_TARGET}/release/{LIB_NAME}"));
    let macos_lib = root.join(format!("target/{MACOS_TARGET}/release/{LIB_NAME}"));

    let xcframework_out = root.join(XCFRAMEWORK_PATH);

    create_xcframework(
        &root,
        &[
            (&ios_device_lib, &headers_dir),
            (&ios_sim_lib, &headers_dir),
            (&macos_lib, &headers_dir),
        ],
        &xcframework_out,
    )?;

    println!("▶ xcframework created at {}", xcframework_out.display());

    Ok(())
}

fn workspace_root() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .map_err(|e| format!("failed to canonicalize manifest dir: {e}"))
}

fn build_target(root: &Path, target: &str) -> Result<(), String> {
    println!("▶ building {CRATE_NAME} for {target}");

    let success = Command::new("cargo")
        .current_dir(root)
        .arg("build")
        .arg("--release")
        .args(["--package", CRATE_NAME])
        .args(["--target", target])
        .status()
        .map_err(|e| format!("failed to run cargo build: {e}"))?
        .success();

    if !success {
        return Err(format!("cargo build failed for target {target}"));
    }

    Ok(())
}

fn create_xcframework(root: &Path, slices: &[(&Path, &Path)], output: &Path) -> Result<(), String> {
    println!("▶ creating {}", output.display());

    if output.exists() {
        fs::remove_dir_all(&output).map_err(|e| e.to_string())?;
    }

    let mut cmd = Command::new("xcodebuild");
    cmd.current_dir(root).arg("-create-xcframework");

    for (lib, headers) in slices {
        cmd.arg("-library").arg(lib).arg("-headers").arg(headers);
    }

    cmd.arg("-output").arg(output);

    let success = cmd
        .status()
        .map_err(|e| format!("failed to run xcodebuild: {e}"))?
        .success();

    if !success {
        return Err("xcodebuild -create-xcframework failed".to_string());
    }

    Ok(())
}
