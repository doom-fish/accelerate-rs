use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DOCS_RS");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let swift_dir = "swift-bridge";
    println!("cargo:rerun-if-changed={swift_dir}");

    let sdk_output = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("failed to resolve macOS SDK path");
    assert!(
        sdk_output.status.success(),
        "xcrun failed to resolve the macOS SDK path"
    );
    let sdk_path = String::from_utf8_lossy(&sdk_output.stdout)
        .trim()
        .to_owned();

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set by Cargo");
    let swift_build_dir = format!("{out_dir}/swift-build");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let swift_triple = match target_arch.as_str() {
        "x86_64" => "x86_64-apple-macosx",
        "aarch64" => "arm64-apple-macosx",
        other => panic!(
            "apple-accelerate: unsupported target arch '{other}'. Expected x86_64 or aarch64."
        ),
    };

    let output = Command::new("swift")
        .args([
            "build",
            "-c",
            "release",
            "--triple",
            swift_triple,
            "--package-path",
            swift_dir,
            "--scratch-path",
            &swift_build_dir,
        ])
        .output()
        .expect("failed to build Swift bridge");

    if !output.status.success() {
        eprintln!(
            "Swift build stdout:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        eprintln!(
            "Swift build stderr:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("Swift bridge build failed: {:?}", output.status.code());
    }

    println!("cargo:rustc-link-search=native={swift_build_dir}/release");
    println!("cargo:rustc-link-search=native=/usr/lib/swift");
    println!("cargo:rustc-link-search=native={sdk_path}/usr/lib/swift");
    println!("cargo:rustc-link-lib=static=AppleAccelerateBridge");
    println!("cargo:rustc-link-lib=dylib=swiftCore");
    println!("cargo:rustc-link-lib=dylib=swiftAccelerate");
    println!("cargo:rustc-link-lib=framework=Accelerate");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    if let Ok(output) = Command::new("xcode-select").arg("-p").output() {
        if output.status.success() {
            let xcode_path = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            let swift_compat_path = format!(
                "{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift-5.5/macosx"
            );
            let swift_lib_path =
                format!("{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
            println!("cargo:rustc-link-search=native={swift_compat_path}");
            println!("cargo:rustc-link-search=native={swift_lib_path}");
            println!(
                "cargo:rustc-link-arg=-Wl,-force_load,{swift_lib_path}/libswiftCompatibilityConcurrency.a"
            );
            println!(
                "cargo:rustc-link-arg=-Wl,-force_load,{swift_lib_path}/libswiftCompatibility56.a"
            );
            println!("cargo:rustc-link-arg={swift_lib_path}/libswiftCompatibilityPacks.a");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{swift_compat_path}");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{swift_lib_path}");
        }
    }
}
