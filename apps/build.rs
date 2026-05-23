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

    #[cfg(false)]   // DEBUG
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
    }

    // Needed for linking of executables to succeed.
    embuild::espidf::sysenv::output();

    // Write some env.vars to the file system. This allows the developer to see where the last build
    // wrote stuff (in particular the TOML-parsed '.in' snippets).
    #[cfg(false)]
    {
        use std::fs;
        const FN: &str = ".BUILD_ENV";

        let mut bad = Vec::new();
        let arr = [
            "OUT_DIR",
        ].map(|x| {
            let val = env::var(x).unwrap_or_else(|_| {
                bad.push(x); String::default()
            });
            format!("{x}={val}")
        });

        if !bad.is_empty() {
            let suffix = if bad.len() > 1 { "s" } else { "" };
            panic!("❗Missing env.var{suffix}: {}", bad.join(", "))
        }

        let text = format!("\
#
# Created by 'cargo build'.
#
{}", arr.join("\n"));

        fs::write(FN, text)
            .unwrap_or_else(|e| panic!("❗Unable to write {FN}: {e}"));
    }

    // Turn 'bin/{*}/app.toml' -> '{out_dir}/{*}_conf.rs'
    //
    // For _all_ bin targets detected ('light', 'switch'), convert TOML to a program snippet that
    // provides the configuration. Also mark such TOMLs as triggers for re-running 'build.rs'.
    //
    // Note: This needs to be done for all such targets, each time, because of the 'build.rs' execution
    //      model. It's not related to individual builds, but for providing dynamically built pre-compilation
    //      dependencies for any builds.
    //
    // Note2:
    //      If you create a new 'bin' target, you MUST inform the Cargo system about it manually.
    //      Either a 'cargo clean' (harsh!) or just 'touch build.rs'.
    //
    // Note3:
    //      "If two binary targets (light and switch) exist within the same Cargo crate, they share the OUT_DIR."
    {
        let out_dir = env::var("OUT_DIR")
            .expect("OUT_DIR environment variable not set");

        let dir_entries = fs::read_dir("bin")?
            .flatten()
            .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false));

        for entry in dir_entries {
            let toml_path = entry.path().join("app.toml");
            if toml_path.exists() {
                println!("cargo::rerun-if-changed={}", toml_path.to_string_lossy());

                let content = fs::read_to_string(&toml_path)
                    .with_context(|| format!("Not found: {}", toml_path.to_string_lossy()))?;

                let snippet = convert_toml(&content)
                    .context("TOML parsing")?;

                let bin_name = entry.file_name();   // e.g. "light"|"switch"

                let fn_ = format!("{out_dir}/{}_conf.in", bin_name.to_string_lossy());
                fs::write(&fn_, snippet).with_context(
                    || format!("Unable to write {fn_}"),
                )?;
                println!("cargo::warning={}", format!("Created snippet: {fn_}"));
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
