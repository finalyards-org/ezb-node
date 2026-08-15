/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/
#![allow(unreachable_code)]
    // Otherwise unconditional 'exit(1)' will cause warnings.

use std::env;

fn main() {
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

    #[cfg(not(any(feature = "coordinator", feature = "router")))]  // add "end_device" #later
    {
        let arr = ["coordinator", "router", "end_device"].join(", ");
        eprintln!("🛑Must have at least one feature: {}", arr);
        std::process::exit(1);
    }

    // Needed for 'ldproxy' linking (of examples) to succeed.
    embuild::espidf::sysenv::output();
}
