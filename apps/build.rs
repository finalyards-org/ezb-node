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

    // Needed for linking of executables to succeed.
    embuild::espidf::sysenv::output();

    // Turn 'demo/{**}/app.toml' -> '{out_dir}/{bin-name}_conf.rs'
    //
    // Note: It's unnecessary to do this fully dynamically. If you add a new demo, just add it here.
    //
    // Note: This needs to be done for all such targets, each time, because of the 'build.rs' execution
    //      model. The model is NOT RELATED TO INDIVIDUAL BUILDS, but for providing dynamically built
    //      pre-compilation dependencies for any builds.
    //
    // Note2:
    //      If you create a new 'bin' target, you MUST add it here. That edit of 'build.rs' then
    //      also triggers a new run.
    //
    // Note3:
    //      "If two binary targets (light and switch) exist within the same Cargo crate, they share the OUT_DIR."
    {
        let demos = [   // (path, bin-name)
            ("1/light", "1-light"),
            ("1/switch", "1-switch"),
            ("2", "2-door")
        ];

        let out_dir = env::var("OUT_DIR").unwrap();

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
                fs::write(&fn_, snippet).with_context(
                    || format!("Unable to write {fn_}"),
                )?;
                println!("cargo::warning=Created snippet: {fn_}");
            }
        }
    }

    //R EXPERIMENT!!
    #[cfg(false)]
    {
        //println!("cargo:rustc-link-arg=-lesp-zigbee");
        //println!("cargo:rustc-link-arg=-lesp-zigbee-idf.native");
        //println!("cargo:rustc-link-arg=-lesp-zigbee-core.zczr");    // ".release"?

        /***r let lib_dir =
            "/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-6fedde765f63b3e3/out/managed_components/espressif__esp-zigbee-lib/lib/esp32c6/";

        println!("cargo::rustc-link-search=native={}", lib_dir);
        ***/
        //|println!("cargo::rustc-link-arg=-Wl,--start-group");
        println!("cargo::rustc-link-lib=static=esp-zigbee");
        //println!("cargo::rustc-link-lib=static=esp-zigbee-idf");
        println!("cargo::rustc-link-lib=static=esp-zigbee-core.zczr");
        //|println!("cargo::rustc-link-arg=-Wl,--end-group");
    }

    // Run _this build script_ again, if these change:
    //
    // Note! Recompilation will happen, if 'src/*' changes, but 'build.rs' won't get run. Which is good.
    //      Only environment changes that *affect the 'build.rs' output* need be mentioned.
    //
    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}
