/*
* Finding other nodes and binding to them.
*/
use core::pin::Pin;
use core::marker::PhantomPinned;

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, DynamicSender, DynamicReceiver},
};

use ezb_node_raw::{
    ezb_zcl_cluster_id_e,
    ezb_zdo_match_desc_req_s,
    ezb_zdp_match_desc_req_field_s,
};

use crate::{
    ShortAddr,
    ZclError,
    ZdpError,
};

type Entry = Result<Result<(),ZdpError>,ZclError>;

const CHANNEL_CAPACITY: usize = 8;

// This stays alive, in the heap, throughout the matching operation (explicitly managed).
//
struct MatchingContext<'a> {
    // keep the necessary structs alive
    _c_matcher: Pin<Box<CGroupcastMatcher>>,
}

impl MatchingContext<'_> {
    fn tx(e: Entry) {
        todo!()
    }
}

/**
* Carries out a groupcast call on the Zigbee network, listing all devices (which are receiving on idle) that match
* the provided set of 'in' and 'out' cluster id's.
*
* Lifespan:
*   Remains valid throughout the matching mechanism. This also ensures that what we place in the
*   fields remains available for the C API, until the matching has ended.
*/
pub(super) struct CGroupcastMatcher {
    // For safety, keep pinned until the end of the matching process.
    req: ezb_zdo_match_desc_req_s,

    // pointed to by 'req.field.cluster_list'
    _clusters: Box<[u16]>,

    // own the 'channel'; use via '.tx()'
    _channel: Channel<CriticalSectionRawMutex, Entry, CHANNEL_CAPACITY>,

    // Pins 'Self' to heap, not to be moved.
    _pin: PhantomPinned,
}

impl CGroupcastMatcher {

    /**
    * Create a pinned matcher (lives in the heap; stationary, i.e. fields can be passed to C APIs).
    */
    pub(super) fn new_pinned(in_clusters: &[ezb_zcl_cluster_id_e], out_clusters: &[ezb_zcl_cluster_id_e]) -> Pin<Box<Self>> {

        // Create a channel. The sending end is placed in the '.req.useer_ctx' so we can feed it from the C callback
        // (even if there were multiple matches going on).
        //
        let channel: Channel<CriticalSectionRawMutex, Entry, CHANNEL_CAPACITY> = Channel::new();

        let tx = channel.dyn_sender();
        let rx = channel.dyn_receiver();

        let clusters: Box<[u16]> = {
            let mut vec = Vec::with_capacity(in_clusters.len() + out_clusters.len());
            vec.extend_from_slice(&in_clusters);
            vec.extend_from_slice(&out_clusters);

            vec.into_iter().map(|x| x as u16).collect()
        };

        // Make the struct in stages, writing '.field.cluster_list' at the end, once we've pinned the whole 'Self' in place,
        // since the buffer we place there resides within 'Self'. This is a self-referencing structure (not the C part, but
        // the C + our buffer).
        //
        let mut me_pinned: Pin<Box<Self>> = {

            let o = Self {
                req: ezb_zdo_match_desc_req_s {
                    dst_nwk_addr: ShortAddr::GROUPCAST.0,   // 0xFFFD
                    field: ezb_zdp_match_desc_req_field_s {
                        nwk_addr_of_interest: ShortAddr::GROUPCAST.0, // 0xFFFD
                        profile_id,
                        num_in_clusters: in_clusters.len() as u8,
                        num_out_clusters: out_clusters.len() as u8,
                        cluster_list: core::ptr::null_mut(), // will be set
                    },
                    cb: core::ptr::null_mut(),
                    user_ctx: core::ptr::null_mut(), // will be set
                },
                _clusters: clusters,
                _channel: channel,
                _pin: PhantomPinned,
            };

            Box::pin(o)
        };

        // Now that the '._clusters' is pinned and stationary, we can write '.req.field.cluster_list' (required by
        // the C API) to point to it.
        unsafe {
            let tmp = me_pinned.as_mut().get_unchecked_mut();
            tmp.req.field.cluster_list = tmp._clusters.as_mut_ptr();

            // Also set the C callback's context (now that the structure is pinned)
            tmp.req.user_ctx = tmp._channel;
        }

        me_pinned
    }

    pub(super) async fn start_matching(&self, src_ep: u8) {
        unimplemented!()
    }
}
