fn nightly_rustc() -> bool {
    fn op() -> Option<bool> {
        use std::{env, process, str};
        let rustc = env::var_os("RUSTC")?;
        let output = process::Command::new(rustc)
            .arg("--version")
            .output()
            .ok()?;
        let version = str::from_utf8(&output.stdout).ok()?;
        Some(version.contains("nightly"))
    }
    if let Some(true) = op() {
        true
    } else {
        false
    }
}

fn main() {
    if nightly_rustc() {
        println!("cargo:rustc-cfg=bitintr_nightly");
    }
}
