#![cfg(feature = "_remote_any")]

use crate::{
    node::ZigbeeGuard,
    ShortAddr
};

/**
* Access to the network for 'Access...' structs.
*
* @note We don't need access to the 'Node' struct (which would be difficult anyways, since it's not
*       "dyn compatible".
*/
#[derive(Copy, Clone)]
pub struct AccessorCtx {
    //pub(super) src_ep: u8,

    // Allow application access to these - at least for debug logging.
    pub dst_addr: ShortAddr,
    pub dst_ep: u8,
}

impl AccessorCtx {
    pub(super) fn new(/*src_ep: u8,*/ dst_addr: ShortAddr, dst_ep: u8) -> Self {
        Self { /*src_ep,*/ dst_addr, dst_ep }
    }

    /**
    * Do something on the 'ezb_zigbee_lib' C API, with locking.
    *
    * @note Having this here (and not within a trait) since it would make a trait not "dyn compatible".
    */
    pub(super) fn guarded<F,T>(&self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        let _guard = ZigbeeGuard::acquire();
        f(self.node)
    }
}

//pub trait Accessor: Copy + Clone {
//    fn get(&self) -> &AccessorCtx;
//}
