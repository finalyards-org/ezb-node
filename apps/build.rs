/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/
use anyhow::*;

use std::env;

use esp_zb_toml;

fn main() -> Result<()> {

    // Detect when IDE is running us, and DO NOT ENGAGE with 'esp-idf-sys' if we're under IDE.
    // i.e. keep this before 'embuild::...' - otherwise '.espressif' or '.embuild' start occuring
    //      on the local disk.
    //
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    {
        if env::var("__CFBundleIdentifier").is_ok() {
            panic!();   // try to avoid spending a _lot_ of time, building ESP-IDF on the IDE
            //return;  // skip the rest
        }
    }

    // Needed for linking of executables to succeed.
    embuild::espidf::sysenv::output();

    const OUT_DIR: String = env::var("OUT_DIR").unwrap();

    // Turn 'bin/{name}/app.toml' -> 'tmp/{name}_conf.rs'
    {
        use std::fs;

        // Process the app config (iff building a 'bin')
        if let Ok(bin_name) = std::env::var("CARGO_BIN_NAME") {
            let toml_path = format!("bin/{}/app.toml", bin_name);

            let content = fs::read_to_string(&toml_path)
                .with_context(|| format!("Not found: {}", toml_path))?;

            let snippet = esp_zb_toml::parse_config(&content)
                .context("TOML parsing")?;

            let _fn = format!("{OUT_DIR}/{app}_conf.in");
            fs::write(_fn, snippet).with_context(
                || format!("Unable to write {_fn}")
            )?;
        }

        // Rerun 'build.rs' if _any_ 'bin/*/app.toml' changes
        //
        if let Ok(entries) = fs::read_dir("bin") {
            for entry in entries.flatten() {
                let toml_path = entry.path().join("app.toml");
                if toml_path.exists() {
                    println!("cargo::rerun-if-changed={}", toml_path.display());
                }
            }
        }
    }

    // Run _this build script_ again, if these change:
    //
    // Note! Recompilation will happen, if 'src/*' changes, but 'build.rs' won't get run. Which is good.
    //      Only environment changes that *affect the 'build.rs' output* need be mentioned.
    //
    println!("cargo::rerun-if-changed=build.rs");

    //r println!("cargo::rustc-check-cfg=cfg(esp_idf_version, values(\"5\"))");

    Ok(())
}
