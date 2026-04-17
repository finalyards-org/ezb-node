/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/

use std::env;

fn main() {
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

    /***R
    // Inject env.vars for 'esp-idf-sys' build, pointing to the 'partitions.csv' as an absolute
    // path.
    //
    // HACK. Needed because 'esp-idf-sys' does not properly map the relative paths in 'sdkconfig.defaults'.
    {
        use std::path::PathBuf;

        let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let partition_file = PathBuf::from(&project_dir).join("partitions.csv");
        println!("cargo:warning=PARTITIONS FILE: {}", partition_file.display());
        unimplemented!();

        assert!(partition_file.exists(), "'partitions.csv' not found");

        println!("cargo:rustc-env=ESP_IDF_SDKCONFIG_CUSTOM_PARTITION_TABLE={}", partition_file.display());

            // Joissain versioissa tämä on varmempi tapa:
            println!("cargo:rustc-env=SDKCONFIG_DEFAULTS={}/sdkconfig.defaults", project_dir);
        }***/

    // Needed for linking of executables to succeed.
    embuild::espidf::sysenv::output();

    //r println!("cargo::rustc-check-cfg=cfg(esp_idf_version, values(\"5\"))");
}
