
mod access;
pub(self) use access::Access;

#[cfg(feature = "bind_color_dimmable_light")]
mod access_color_dimmable_light;
#[cfg(feature = "bind_color_dimmable_light")]
pub use access_color_dimmable_light::*;

//mod matcher;
//pub(self) use matcher::*;

#[cfg(feature = "bind_color_dimmable_light")]
mod match_color_dimmable_light;
#[cfg(feature = "bind_color_dimmable_light")]
pub use match_color_dimmable_light::*;

mod matching_context;
pub(self) use matching_context::*;
use crate::node::ZigbeeGuard;

pub(self) trait AccessTools {
    /**
    * Do something on the 'ezb_zigbee_lib', with locking.
    */
    fn guarded<F>(&self, f: F) where F: Fn() {
        let _guard = ZigbeeGuard::acquire();
        f();
    }
}
