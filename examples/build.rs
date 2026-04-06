/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/

/*
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

    // Needed for 'ldproxy' linking (of examples) to succeed.
    embuild::espidf::sysenv::output();

    //r println!("cargo::rustc-check-cfg=cfg(esp_idf_version, values(\"5\"))");
}
