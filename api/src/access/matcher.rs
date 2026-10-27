#![cfg(feature = "_remote_any")]

use futures_util::Stream;

use ezb_node_raw::{
    ezb_zcl_cluster_id_e,
};

use super::{
    MatchEvent,
    MatchingContext,
};

use crate::{node::ZigbeeGuard, ShortAddr};

/**
* Matcher for finding remote end points to bind with.
*/
// tbd. This has no fields; consider making it a function instead.
pub struct Matcher<A: Accessor> {
    // (no fields needed)

    // Required to denote to Rust the type parameter matters.
    _marker: core::marker::PhantomData<A>
}

impl<A: Accessor> Matcher<A> {
    pub fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData
        }
    }

    /**
    * Start matching.
    *
    * Passes matches over as a profile specific accessor type, through which the application can
    * access the remote node, in a typed fashion!!
    *
    * @note Accessors live longer than the matching.
    */
    pub fn start(self) -> impl Stream<Item = (ShortAddr, MatchEvent<A>)> {
        let mcp = MatchingContext::<A>::new_pinned();
        mcp.start_matching()
    }
}

/// Base for all accessors.
///
pub trait Accessor {
    const SRC_EP: u8;

    /// The input clusters that the remote counter-part must provide, for a bind to occur.
    const IN_CLUSTERS: &[ezb_zcl_cluster_id_e];

    /// The output clusters that the remote counter-part must provide, for a bind to occur.
    const OUT_CLUSTERS: &[ezb_zcl_cluster_id_e];

    /// Creation of accessors happens via this function.
    fn new() -> Self;

    /// Bind to the remote. Calling this means the application wants to start using the remote end point.
    ///
    #[cfg(false)]   // #later
    async fn bind() {
        todo!()
    }

    /// Ability for the specific accessor to reach 'esp_zigbee_lib' C API's, locked.
    // tbd. move to a utility function ('pub(crate)')
    fn guarded<F,T>(f: F) -> T
    where
        F: FnOnce() -> T
    {
        let _guard = ZigbeeGuard::acquire();
        f()
    }
}
