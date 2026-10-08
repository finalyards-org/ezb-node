/*
* build.rs
*/
fn main() {

    // tbd. enforce that for feature 'toml', _none_ of the coordinator/router/end device is enabled!!!

    // One node can be only one of:
    //  - coordinator
    //  - router
    //  - end device
    {
        let roles = 0;
        #[cfg(feature = "coordinator")]
        let roles = roles +1;
        #[cfg(feature = "router")]
        let roles = roles +1;
        //#[cfg(feature = "end_device_UNTESTED")]
        //let roles = roles +1;

        let arr = ["coordinator", "router" /*, "end_device"*/].join(", ");

        if roles == 0 {
            eprintln!("🛑 Please enable one of: {}", arr);
            std::process::exit(1);
        } else if roles > 1 {
            eprintln!("🛑 A Zigbee node needs to have one precise role. Please enable one of: {}", arr);
            std::process::exit(1);
        }
    }

    // Only a Router may be without endpoints - and without any device types.
    {
        #[cfg(not(feature = "router"))]
        #[cfg(not(any(feature = "dt_color_dimmable_light", feature = "dt_color_dimmer_switch", feature = "dt_ias_cie")))]
        {
            eprintln!("🛑 A coordinator or end device needs to have at least one Device Type ('dt_...' feature) enabled.");
            std::process::exit(1);
        }
    }
}
