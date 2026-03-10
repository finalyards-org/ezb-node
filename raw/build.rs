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
use anyhow::*;

use std::{
    env,
    process::Command
};

/*
*/
fn main() -> Result<()> {

    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //
    #[allow(non_snake_case)]
    let IDE_RUN = env::var("__CFBundleIdentifier").is_ok();

    // If IDE runs, terminate early.
    if IDE_RUN { return Ok(()) };

    // DEBUG: Show what we know about the compilation.
    //
    // <<
    //   CARGO_CFG_TARGET_FEATURE=c,m
    //   CARGO_FEATURE_{..feature..}=1
    //   LD_LIBRARY_PATH=/home/ubuntu/VL53L5CX_rs.cifs/vl53l5cx_uld/target/release/deps:/home/ubuntu/VL53L5CX_rs.cifs/vl53l5cx_uld/target/release:/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib
    //   RUSTUP_TOOLCHAIN=stable-x86_64-unknown-linux-gnu
    //   TARGET=riscv32imc-unknown-none-elf
    // <<
    #[cfg(false)]
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
    }

    //---
    // Config sanity checks
    {
        // nada. If there are conflicting feature combinations, give an error here.
    }

    // Expose 'OUT_DIR' to an external (Makefile.dev) build system
    #[cfg(false)]
    {
        use std::{env,fs};
        const TMP: &str = ".OUT_DIR";

        let out_dir = env::var("OUT_DIR")
            .expect("OUT_DIR to have a value");

        fs::write(TMP, out_dir)
            .expect(format!("Unable to write {TMP}").into());
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

    // Change in Makefile or 'build.rs' itself re-triggers a build
    println!("cargo::rerun-if-changed={}", "Makefile");
    println!("cargo::rerun-if-changed={}", "build.rs");

    Ok(())
}
