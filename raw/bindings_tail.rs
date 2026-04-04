/*
* Gathering the bindgen-generated binding like this allows us to attach
* 'Default' (and/or other traits) to its types.
*
* We can see this (converting C uninitialized struct parts to Rust-friendly defaults)
* as being part of the stated aim of "1-to-1 C-Rust" interface.
*/

include!("tmp/bindings_0.rs");

impl Default for uart_config_t {
    fn default() -> Self {
        todo!()
    }
}
impl Default for
