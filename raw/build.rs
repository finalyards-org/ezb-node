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
    //      ..
    //    DEP_ESP_IDF_EMBUILD_ENV_PATH=/home/ubuntu/.espressif/tools/esp-clang/esp-19.1.2_20250312/esp-clang/bin:/home/ubuntu/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20251107/riscv32-esp-elf/bin:/home/ubuntu/.espressif/tools/esp32ulp-elf/2.38_20240113/esp32ulp-elf/bin:/home/ubuntu/.espressif/tools/cmake/3.30.2/bin:/home/ubuntu/.espressif/tools/ninja/1.12.1:/home/ubuntu/.espressif/tools/esp-rom-elfs/20241011:/home/ubuntu/.espressif/python_env/idf5.5_py3.12_env/bin:/home/ubuntu/bin:/home/ubuntu/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/home/ubuntu/bin
    //    DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH=/home/ubuntu/.espressif/esp-idf/v5.5.3
    //      ..
    //    OUT_DIR=/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp_zb_raw-8250c09cd209b0ce/out
    //  <<
    #[cfg(false)]
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
    }

    idf_stuff();

    // Expose some env.vars to 'Makefile'.
    // Note: Writing them into a file means we can more freely develop the make itself.
    //
    // Note: The author failed to reach 'DEP_ESP_IDF_...' values in any other way, i.e. via
    //      'embuild::' APIs.
    {
        use std::fs;

        const FN: &str = ".BUILD_ENV";

        let mut bad = Vec::new();
        let arr = [
            "DEP_ESP_IDF_EMBUILD_ENV_PATH",
            "DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH",
            "DEP_ESP_IDF_ROOT",
                //
            "ESP_IDF_TOOLS_INSTALL_DIR",            //"global"|"out"
            "ESP_IDF_VERSION",                      //"5.5.4"
            "MCU",
            //"OUT_DIR",                              //"{target}/riscv32imac-esp-espidf/release/build/esp-zb-raw-f7c4340a8cec066b/out"
        ].map(|x| {
            let val = env::var(x)
                .or_else(|_| env::var(x.to_ascii_lowercase()))  // check e.g. "esp_idf_version"
                .unwrap_or_else(|_| {
                    bad.push(x); String::default()
                });
            format!("{x}={val}")
        });

        if !bad.is_empty() {
            let suffix = if bad.len() > 1 { "s" } else { "" };
            panic!("❗Missing env.var{suffix}: {}", bad.join(", "))
        }

        // Values in a format GNU Makefile can gulp in.
        let text = format!("\
#
# Created by 'cargo build --release'. CHANGES WILL BE LOST!
#
{}", arr.join("\n"));

        fs::write(FN, text)
          .unwrap_or_else(|e| panic!("❗Unable to write {FN}: {e}"));
    }

    // make stuff
    //
    let st = Command::new("make")
        .arg( "tmp/bindings_0.rs" ) // generate the Rust bindings
        .arg( "tmp/libezb_wrap.a" )     // + the C-sourced object archive
        .output()
        .expect("to be able to launch `make`")   // shown if 'make' not found on PATH
        .status;

    if !st.success() {
        panic!("❗[ERROR!]: Running 'make' failed. \
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
    }

    // Take in the 'tmp/libezb_wrap.a'
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        println!("cargo:rustc-link-search={}/tmp", manifest_dir);

        println!("cargo::rustc-link-lib=static=ezb_wrap");
    }

    // What files should trigger a new 'build.rs' run, if they change?
    //
    // Cargo note: 'build.rs' is _not_ executed for every build. It does not (need to) run e.g. if the sources change
    //      (generally). Here, we list the files we know will need us to run, again.
    //
    println!("cargo:rerun-if-changed=build.rs");
    // +
    println!("cargo:rerun-if-changed=bindings.rs");
    println!("cargo:rerun-if-changed=Makefile");
    println!("cargo:rerun-if-changed=wrap.h");
    //
    //#[cfg(false)]   // might also be harmful, causing rebuild loops (disabling for now, at least on ESP-IDF 6.0 builds)
    println!("cargo:rerun-if-changed=tmp/bindings_0.rs");
        //
        // This is  *output* but helps _dependent_crates_ to realize if it has been removed, and avoid a failing
        // upstream build.

    // Note 2: 'Cargo.{toml|lock}' are tracked nonetheless.
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
