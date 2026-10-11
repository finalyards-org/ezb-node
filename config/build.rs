/*
* build.rs
*/
fn main() {

    // Enforce correct number of node types enabled.
    {
        let roles = 0;
        #[cfg(feature = "coordinator")]
        let roles = roles +1;
        #[cfg(feature = "router")]
        let roles = roles +1;
        //#[cfg(feature = "end_device_UNTESTED")]
        //let roles = roles +1;

        let role_features = ["coordinator", "router" /*, "end_device"*/].join(", ");

        // If 'gen':
        //  - multiple node types can be enabled (one needs to be)
        //
        #[cfg(feature = "gen")]
        {
            if roles == 0 {
                eprintln!("🛑 [generation] Please enable one (or more) of: {}", role_features);
                std::process::exit(1);
            }
        }

        // If using in an application:
        //  - only one node type (coordinator, router, end device) is to be enabled
        //
        #[cfg(not(feature = "gen"))]
        {
            if roles == 0 {
                eprintln!("🛑 Please enable one of: {}", role_features);
                std::process::exit(1);
            } else if roles > 1 {
                eprintln!("🛑 A Zigbee node's role is rather fixed. Please enable only one of: {}", role_features);
                std::process::exit(1);
            }
        }
    }

    // Only a Router may be without endpoints - and without any device types.
    {
        #[cfg(not(feature = "router"))]
        #[cfg(not(feature = "_any_dt"))]
        {
            eprintln!("🛑 A coordinator or end device needs to have at least one Device Type ('dt_...') enabled.");
            std::process::exit(1);
        }
    }
}
