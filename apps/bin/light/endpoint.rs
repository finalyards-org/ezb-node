

struct MyEndpoint{
    _private: (),
}

impl MyEndpoint {

    fn new() -> Self {
        Self{_private: ()}
    }
}

// tbd. read manufacturer name, model, endpoint number from a TOML file, during build.
//      - by the endpoint type
impl EndPoint for MyEndpoint {
    const SLOT: u8 = 10; // COLOR_DIMMABLE_LIGHT_ENDPOINT;

    const MANUFACTURER_NAME: &'static str = "Your Light";
    const MANUFACTURER_MODEL: &'static str = "Your Light";
}

impl ColorDimmableLight for MyEndpoint {

    const Config = ColorDimmableLightConfig::default();

}
