/*
* build.rs
*/
fn main() {

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

        let arr = ["coordinator", "router", "end_device"].join(", ");

        if roles == 0 {
            eprintln!("🛑 Please enable one of: {}", arr);
            std::process::exit(1);
        } else if roles > 1 {
            eprintln!("🛑 A Zigbee node needs to have one precise role. Please enable one of: {}", arr);
            std::process::exit(1);
        }
    }
}
