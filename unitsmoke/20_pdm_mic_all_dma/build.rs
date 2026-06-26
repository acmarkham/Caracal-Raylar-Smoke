fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let sram3_dma_script = format!("{manifest_dir}/sram3_dma.x");

    println!("cargo:rerun-if-changed=sram3_dma.x");
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-T{sram3_dma_script}");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
