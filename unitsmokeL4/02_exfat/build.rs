fn main() {
    assert_eq!(
        std::env::var("TARGET").expect("Cargo sets TARGET"),
        "thumbv7em-none-eabihf",
        "Evo L4 requires --target thumbv7em-none-eabihf; the workspace default is U5"
    );
    println!("cargo:rerun-if-changed=stack-check.x");
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"));
    std::fs::copy("stack-check.x", out.join("l4-stack-check.x")).expect("copy linker assertion");
    println!("cargo:rustc-link-search={}", out.display());
    for arg in ["--nmagic", "-Tlink.x", "-Tdefmt.x", "-Tl4-stack-check.x"] {
        println!("cargo:rustc-link-arg-bins={arg}");
    }
}
