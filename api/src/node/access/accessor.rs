#![cfg(feature = "_match_any")]

use crate::{node::ZigbeeGuard, Node, ShortAddr};

/**
* Access to the network for 'Access...' structs.
*/
#[derive(Copy, Clone)]
pub struct AccessorCtx {
    pub(super) node: &'static dyn Node,
    pub(super) src_ep: u8,     // needed for Zigbee protocol, e.g. where the responses shall be routed

    // Allow application access to these - at least for debug logging.
    pub dst_addr: ShortAddr,
    pub dst_ep: u8,
}

impl AccessorCtx {
    pub(super) fn new(node: &'static dyn Node, src_ep: u8, dst_addr: ShortAddr, dst_ep: u8) -> Self {
        Self { node, src_ep, dst_addr, dst_ep }
    }

    /**
    * Do something on the 'ezb_zigbee_lib' C API, with locking.
    *
    * @note Having this here (and not within a trait) since it would make a trait not "dyn compatible".
    */
    pub(super) fn guarded<F,T>(&self, f: F) -> T where F: Fn(&'static dyn Node) -> T {
        let _guard = ZigbeeGuard::acquire();
        f(self.node)
    }
}

pub trait Accessor: Copy + Clone {
    fn get(&self) -> &AccessorCtx;
}
