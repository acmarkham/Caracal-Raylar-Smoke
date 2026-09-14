fn main() {
    use std::time::{SystemTime, UNIX_EPOCH};

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    println!("cargo:rerun-if-env-changed=INTEGRATION002_FAKE_UTC_SECONDS");

    let fake_utc = std::env::var("INTEGRATION002_FAKE_UTC_SECONDS").unwrap_or_else(|_| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("build clock must be after the Unix epoch")
            .as_secs()
            .to_string()
    });
    fake_utc
        .parse::<i64>()
        .expect("INTEGRATION002_FAKE_UTC_SECONDS must be a signed integer");
    println!("cargo:rustc-env=INTEGRATION002_FAKE_UTC_SECONDS={fake_utc}");
}
