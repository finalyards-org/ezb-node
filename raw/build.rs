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

const SUB_PATH: &str = "../.sub";

/*
*/
fn main() {
    // Run 'git submodule update --init' *automatically*, if it hasn't been done, yet.
    //
    // Note: requires 'git' only if the submodule is not there, i.e. if you run that command
    //      manually, 'git' is not needed.
    //
    ensure_submodule_inited("esp-zigbee-sdk");  // panics if there is a problem

    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    {
        if env::var("__CFBundleIdentifier").is_ok() {
            panic!();   // try to avoid spending a _lot_ of time, building ESP-IDF on the IDE
            //return;  // skip the rest
        }
    }

    // Needed by IDF machinery.
    //  E.g. "emits the necessary cfg flags for conditional compilation" (and likely way more..)
    #[cfg(false)]   // DO WE NEED IT ALREADY IN 'RAW' - or only for linking?
    embuild::espidf::sysenv::output();

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

    //---
    // Config sanity checks
    {
        // nada. If there are conflicting feature combinations, give an error here.
    }

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
            "ESP_IDF_VERSION",                      //"5.5.3"
            "MCU",
            "OUT_DIR",                              //"{target}/riscv32imac-esp-espidf/release/build/esp-zb-raw-f7c4340a8cec066b/out"
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
        .arg( format!("tmp/bindings.rs") )      // generate the Rust bindings
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

    // Check that keys in 'sdkconfig.defaults' are valid. CMake itself would only WARN about them,
    // and those warnings are lost in the build noise.
    #[cfg(false)]   // didn't work like that; R
    {
        use embuild::espidf;
        use embuild::cmake::Config;

        let idf = espidf::EspIdf::try_from_env().unwrap();  // ESP-IDF environment, expects 'IDF_PATH' env.var.
        let known = idf.config().all_known_configs();

        let defaults = Config::from_file("sdkconfig.defaults")
            .expect("aaa"); // tbd.

        let mut bad = Vec::new();

        for (key, _) in defaults.iter() {
            if !known.contains(key.as_str()) {
                bad.push(key);
            }
        }

        if !bad.is_empty() {
            panic!("❗️Unknown 'sdkconfig.defaults' key{}: {}",
                if bad.len() == 1 { "" } else "s",
                bad.join(",")
            );
        }
    }

    //println!(r#"cargo::rustc-check-cfg=cfg(esp_idf_version_major, values("5"))"#);
    //println!(r#"cargo::rustc-check-cfg=cfg(esp_idf_version, values("5.3", "5.4", "5.5"))"#);
}

/*
* Ensure that the named submodule is inited. Doing this in 'build.rs' (automatically) means:
*   - user should have 'git' available
*   - we don't need to mention it in README
*/
fn ensure_submodule_inited(name: &str) {
    use std::{fs, path::Path};

    let path = Path::new(SUB_PATH).join(name);

    // If the folder's occupied, nothing needs to be done.
    //
    let has_contents = match fs::read_dir(&path) {
        Ok(mut entries) /*if entries.next().is_some()*/ => {    // Rust: cannot have mutable vars "within the pattern guard"
            entries.next().is_some()
        },
        _ => false  // empty or non-existing folder
    };

    if has_contents {
        println!("cargo:warning=🟩 DEBUG: submodule '{name}' is already populated.");  // DEBUG
        return;
    } else {
        println!("cargo:warning=🟨 submodule '{name}' is empty, attempting to initialize submodules...");

        let git_res = Command::new("git")
            .args(["submodule", "update", "--init"])
            .status();

        match git_res {
            Err(e) => {
                panic!(
                    "❗️Failed to run 'git submodule update --init': {e}\n\
                     Make sure 'git' is installed and available in PATH."
                );
            }
            Ok(status) if !status.success() => {
                panic!(
                    "❗️'git submodule update --init' failed with exit code: {:?}",
                    status.code()
                );
            }
            Ok(_) => {}
        }

        // Confirm the folder now is populated
        match fs::read_dir(&path) {
            Ok(mut entries) => if entries.next().is_none() {
                panic!(
                    "❗️Submodule directory `{}` is still empty after update.\n\
                    Check that the submodule is correctly defined in .gitmodules.",
                    path.display()
                );
            },
            Err(_) => {
                panic!(
                    "❗️Submodule directory `{}` is still non-existent after update.\n\
                    Check that the submodule is correctly defined in .gitmodules.",
                    path.display()
                );
            }
        };
    }
}
