use std::process::Command;

fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-env-changed=RAYLAR_GIT_HASH");
    println!(
        "cargo:rustc-env=RAYLAR_FIRMWARE_VERSION={}",
        env!("CARGO_PKG_VERSION")
    );
    if std::env::var_os("RAYLAR_GIT_HASH").is_none() {
        if let Ok(output) = Command::new("git")
            .args(["rev-parse", "--short=12", "HEAD"])
            .output()
        {
            if output.status.success() {
                if let Ok(hash) = std::str::from_utf8(&output.stdout) {
                    println!("cargo:rustc-env=RAYLAR_GIT_HASH={}", hash.trim());
                }
            }
        }
    }
}
