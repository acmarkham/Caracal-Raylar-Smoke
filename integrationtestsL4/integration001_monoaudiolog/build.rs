fn main() {
    println!("cargo:rerun-if-changed=stack-check.x");
    if std::env::var_os("CARGO_FEATURE_FIRMWARE").is_none() {
        return;
    }
    assert_eq!(
        std::env::var("TARGET").unwrap(),
        "thumbv7em-none-eabihf",
        "Evo L4 integration requires --target thumbv7em-none-eabihf"
    );
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::copy("stack-check.x", out.join("l4-stack-check.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    for arg in ["--nmagic", "-Tl4-stack-check.x", "-Tlink.x", "-Tdefmt.x"] {
        println!("cargo:rustc-link-arg-bins={arg}");
    }
    let git = std::process::Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .unwrap();
    println!(
        "cargo:rustc-env=L4_GIT={}",
        String::from_utf8(git.stdout).unwrap().trim()
    );
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=L4_BUILD_UNIX={seconds}");
}
