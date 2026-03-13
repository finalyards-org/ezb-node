/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*
* We call GNU 'make' from within the 'build.rs'. This allows us to build in the normal Cargo way,
* yet benefit from file system dependency trees.
*/
use std::{
    env,
    process::Command
};

/*
*/
fn main() {
    // Needed by IDF machinery.
    //  E.g. "emits the necessary cfg flags for conditional compilation" (and likely way more..)
    embuild::espidf::sysenv::output();

    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    {
        if env::var("__CFBundleIdentifier").is_ok() {
            panic!();   // try to avoid spending a _lot_ of time, building ESP-IDF on the IDE
            //return;  // skip the rest
        }
    }

    // DEBUG: Show what we know about the compilation.
    //  <<
    //    DEP_ESP_IDF_ROOT=/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-853a2667a8c024cb/out
    //    DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH=/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-853a2667a8c024cb/out/espressif/esp-idf/v5.5.3
    //    OUT_DIR=/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp_zb_raw-8250c09cd209b0ce/out
    //  <<
    #[cfg(false)]
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
    }

    idf_stuff();

    //---
    // Config sanity checks
    {
        // nada. If there are conflicting feature combinations, give an error here.
    }

    // Expose some env.vars to 'Makefile'.
    // Note: Writing them into a file means we can more freely develop the make itself.
    {
        use std::{env,fs};
        const FN: &str = ".BUILD_ENV";

        // Note: 'DEP_ESP_IDF_...' have much shared path. We *could* deduce the latter from
        //      the first, but it's unnecessary work.
        let arr = [
          "DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH",
          "DEP_ESP_IDF_ROOT",
          "ESP_IDF_TOOLS_INSTALL_DIR",
          //"ESP_IDF_VERSION",
          "MCU"
        ].map(|x| {
          let val = env::var(x)
            .unwrap_or_else(|_| panic!("{x} to have a value") );
          format!("{x}={val}")
        });

        // Values in a format GNU Makefile can gulp in.
        let text = format!("\
#
# Created by 'cargo build --release'. CHANGES WILL BE LOST!
#
{}", arr.join("\n"));

        fs::write(FN, text)
          .unwrap_or_else(|e| panic!("Unable to write {FN}: {e}"));
    }

    // make stuff
    //
    let st = Command::new("make")
        .arg( format!("tmp/bindings.rs") )      // generate the Rust bindings
        .output()
        .expect("to be able to launch `make`")   // shown if 'make' not found on PATH
        .status;

    if !st.success() {
        panic!("[ERROR!]: Running 'make' failed. \
            SUGGESTION: run 'make manual' on the command line to see more error information. \
        ");
    }

    // Link arguments
    //
    {
        for s in [
            "-Tlinkall.x",
            "-Tdefmt.x"     // required by 'defmt'
        ] {
            println!("cargo::rustc-link-arg={}", s);
        }

        // disabled for ever
        //if std::env::var("TEST").is_ok() {  // 'cargo test' run
        //    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
        //}
    }

    println!("cargo:rustc-link-search=tmp");
    //_! println!("cargo:rustc-link-lib=static=vendor_uld{}", X);
}

fn idf_stuff() {
    // Do NOT allow build if a system-wide ESP-IDF is active.
    {
        if env::var("IDF_PATH").is_ok() {
            panic!("❗️Please build with a shell that doesn't know of system-wide esp-idf. 'IDF_PATH' env.var. detected.");
        }
    }

    //println!(r#"cargo::rustc-check-cfg=cfg(esp_idf_version_major, values("5"))"#);
    //println!(r#"cargo::rustc-check-cfg=cfg(esp_idf_version, values("5.3", "5.4", "5.5"))"#);
}
