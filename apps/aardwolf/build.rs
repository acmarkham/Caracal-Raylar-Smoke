fn main() {
    if std::env::var("TARGET").unwrap_or_default().contains("none") {
        for arg in ["--nmagic", "-Tlink.x", "-Tdefmt.x"] {
            println!("cargo:rustc-link-arg-bins={arg}");
        }
    }
}
