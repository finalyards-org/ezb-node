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

const TMP_DIR: &str = "tmp/";

fn main() -> Result<()> {

    // Detect when IDE is running us, and DO NOT ENGAGE with 'esp-idf-sys' if we're under IDE.
    // i.e. keep this before 'embuild::...' - otherwise '.espressif' or '.embuild' start occuring
    //      on the local disk.
    //
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    {
        if env::var("__CFBundleIdentifier").is_ok() ||
            env::var("REMOTE_DEV_SERVER_IS_NATIVE_LAUNCHER").is_ok() {
            panic!("IDE build cut short");   // try to avoid spending a _lot_ of time, building ESP-IDF on the IDE
            //return;  // skip the rest
        }
    }

    #[cfg(false)]   // DEBUG
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
    }

    ensure_no_sdkconfig();

    // Turn _all_ 'demo/{**}/app.toml' -> 'tmp/{bin-name}_conf.rs'
    //
    // Note: This needs to be done for all such targets, at once, whenever anything moves. This is
    //      because of the 'build.rs' execution model: it's NOT RELATED TO INDIVIDUAL BUILDS, but
    //      for providing dynamically built pre-compilation dependencies for any builds.
    //
    // Note 2:
    //      If you create a new 'bin' target, you MUST add it here. That edit of 'build.rs' then
    //      also triggers a new run.
    //
    // Note 3:
    //      The output is in 'tmp', not 'OUT_DIR' (as often is customary) so that also the IDE
    //      (we assume it's host side) gets to see the generated contents.
    {
        let demos = [   // (path, bin-name)
            ("1/light", "1-light"),
            ("1/switch", "1-switch"),
            ("2", "2-door")
        ];

        //r let out_dir = env::var("OUT_DIR").unwrap();
        let out_dir = TMP_DIR;

        for (sub_path, bin_name) in demos {
            let toml_path = std::path::Path::new("demo")
                .join(sub_path)
                .join("app.toml");

            if !toml_path.exists() {
                // If you get this, check the 'demos' above.
                println!("cargo::warning=🛑Internal error, '{}' not found!", toml_path.display());
                std::process::exit(1);
            } else {
                let toml_path_s = toml_path.to_string_lossy();
                println!("cargo::rerun-if-changed={}", toml_path_s);

                let content = fs::read_to_string(&toml_path)
                    .with_context(|| format!("Not found: {}", toml_path_s))?;

                // Note: If this happens, there's not really much debugging info. The output file
                //      has not been created.
                //  <<
                //        Error: TOML parsing
                //
                //   Caused by:
                //       Internal error (syntax error in generated code): expected `,`
                //  <<
                let snippet = convert_toml(&content)
                    .context("TOML parsing")?;

                let fn_ = format!("{}/{}_conf.in", out_dir, bin_name);
                fs::write(&fn_, snippet)
                    .with_context(|| format!("Unable to write {fn_}"))?;

                println!("cargo::warning=Created snippet: {fn_}");
            }
        }
    }

    // Needed for linking of executables to succeed.
    embuild::espidf::sysenv::output();

    // Run _this build script_ again, if these change:
    //
    // Note! Recompilation will happen, if 'src/*' changes, but 'build.rs' won't get run. Which is good.
    //      Only environment changes that *affect the 'build.rs' output* need be mentioned.
    //
    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}

/**
* Ensure there isn't a 'sdkconfig' file accidentially left at 'apps' level. It *could* get picked
* up as the truth (instead of generating one below "out dir"), and confuse builds.
*/
fn ensure_no_sdkconfig() {
    let fn_ = std::path::Path::new("sdkconfig");
    if fn_.exists() {
        panic!(
            "❗️There's an 'sdkconfig' file. It is generally UNWELCOME. Please remove it and try again. 🙂"
        );
    }
}
