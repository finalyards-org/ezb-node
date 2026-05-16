/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/
use anyhow::*;

use std::{env, fs};

use ezb_node_config::convert_toml;

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

    let out_dir = env::var("OUT_DIR")
        .expect("OUT_DIR environment variable not set");

    // Turn 'bin/{name}/app.toml' -> '{OUT_DIR}/{name}_conf.rs'
    {
        use std::fs;
        use std::result::Result::Ok;

        // Process the app config (iff building a 'bin')
        if let Ok(bin_name) = env::var("CARGO_BIN_NAME") {
            let toml_path = format!("bin/{}/app.toml", bin_name);

            let content = fs::read_to_string(&toml_path)
                .with_context(|| format!("Not found: {}", toml_path))?;

            let snippet = convert_toml(&content)
                .context("TOML parsing")?;

            let ref _fn = format!("{out_dir}/{bin_name}_conf.in");
            fs::write(_fn, snippet).with_context(
                || format!("Unable to write {_fn}")
            )?;
        }
    }

    // Rerun 'build.rs' if _any_ 'bin/*/app.toml' changes
    {
        let dir_entries = fs::read_dir("bin")?
            .flatten()
            .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false));

        for entry in dir_entries {
            let toml_path = entry.path().join("app.toml");
            if toml_path.exists() {
                println!("cargo::rerun-if-changed={}", toml_path.display());
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
