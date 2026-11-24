fn main() {
    println!("cargo:rerun-if-changed=stack-check.x");
    if std::env::var_os("CARGO_FEATURE_FIRMWARE").is_none() {
        return;
    }
    assert_eq!(
        std::env::var("TARGET").unwrap(),
        "thumbv7em-none-eabihf",
        "Evo L4 firmware requires --target thumbv7em-none-eabihf"
    );
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::copy("stack-check.x", out.join("stack-check.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    for arg in ["--nmagic", "-Tlink.x", "-Tdefmt.x", "-Tstack-check.x"] {
        println!("cargo:rustc-link-arg-bins={arg}");
    }
}
