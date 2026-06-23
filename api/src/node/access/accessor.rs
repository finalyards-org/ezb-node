

use crate::{node::ZigbeeGuard, Node, ShortAddr};

/**
* Access to the network for 'Access...' structs.
*/
#[derive(Debug, Copy, Clone)]
pub(super) struct AccessorCtx {
    /// Endpoint id of the originating EP.
    node: &'static dyn Node,
    src_ip: u8,
    dst_addr: ShortAddr,
    dst_ep: u8,
}

impl AccessorCtx {
    pub(super) fn new(node: &'static dyn Node, src_ip: u8, dst_addr: ShortAddr, dst_ep: u8) -> Self {
        Self { node, src_ip, dst_addr, dst_ep }
    }

    /**
    * Do something on the 'ezb_zigbee_lib', with locking.
    */
    #[cfg(false)]   //r
    pub(super) fn guarded<F>(&self, f: F) where F: Fn(&'static dyn Node) {
        let _guard = ZigbeeGuard::acquire();
        f(self.node);
    }
}

pub trait Accessor {
    fn accessorXX(&self) -> &AccessorCtx;   //r

    /**
    * Do something on the 'ezb_zigbee_lib', with locking.
    */
    fn guarded<F>(&self, f: F) where F: Fn(&'static dyn Node) {
        let _guard = ZigbeeGuard::acquire();
        f(self.get().node);
    }
}
