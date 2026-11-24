fn main() {
    let target = std::env::var("TARGET").expect("Cargo sets TARGET");
    assert_eq!(
        target, "thumbv7em-none-eabihf",
        "Evo L4 requires --target thumbv7em-none-eabihf; the workspace default is U5"
    );
    for arg in ["--nmagic", "-Tlink.x", "-Tdefmt.x"] {
        println!("cargo:rustc-link-arg-bins={arg}");
    }
}
