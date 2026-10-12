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

    // Exactly one node type should be enabled.
    //
    // Note: Though 'config' would check this as well, it's good to be early. (or late: may be that
    //      config check happens before us). Anyhow.
    {
        let roles = 0;
        #[cfg(feature = "coordinator")]
        let roles = roles +1;
        #[cfg(feature = "router")]
        let roles = roles +1;
        //#[cfg(feature = "end_device_UNTESTED")]
        //let roles = roles +1;

        let role_features = ["coordinator", "router" /*, "end_device"*/].join(", ");

        if roles != 1 {
            eprintln!("🛑 Please enable exactly one of: {}", role_features);
            std::process::exit(1);
        }
    }
}
