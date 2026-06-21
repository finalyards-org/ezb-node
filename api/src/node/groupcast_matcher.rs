/*
* Finding, binding and accessing them.
*/
use core::pin::Pin;
use core::marker::PhantomPinned;

// tbd. Not sure how this ends up. We likely want to make it more generic, once it works for "color dimmable light device".

use ezb_node_raw::{
    ezb_zcl_cluster_id_e,
    ezb_zdo_match_desc_req_s,
    ezb_af_profile_id_e,
    ezb_zdp_match_desc_req_field_s,
};

use crate::{
    Node,
    ShortAddr,
};

#[cfg(feature = "bind_color_dimmable_light")]
pub trait FindAndBindColorDimmableLightDevice {

    async fn find_and_bind(&self) -> AccessColorDimmableLight {

        let matcher = BroadcastMatcher::new();

        matcher.find_and_bind() .await;
    }

}

/**
* Carries out a groupcast call on the Zigbee network, listing all devices (which are receiving on idle) that
* match the provided set of 'in' and 'out' cluster id's.
*
* Lifespan:
*   Remains valid throughout the matching mechanism. This also ensures that what we place in the
*   fields remains available for the C API, until the matching has ended.
*/
struct BroadcastMatcher {
    // For safety, keep pinned until the end of the matching process. (C API could refer to it after
    // initial call).
    req: ezb_zdp_match_desc_req_field_s,

    // Buffer pointed to by 'req'
    _clusters_buf: Box<[u16]>,

    // Pins 'Self' to heap, not to be moved.
    _pin: PhantomPinned,
}

impl BroadcastMatcher {

    /**
    * Create a pinned matcher (lives in the heap; stationary, i.e. fields can be passed to C APIs).
    */
    fn new() -> Pin<Box<Self>> {
        // tbd. eventually comes from a generic?
        const IN_CLUSTERS: [ezb_zcl_cluster_id_e;3] = [
            ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_ON_OFF,
            ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_LEVEL,
            ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_COLOR_CONTROL,
        ];
        const OUT_CLUSTERS: [ezb_zcl_cluster_id_e;0] = [];

        // We need to make this in stages, since it's becoming a self-referential struct (
        // '.req.cluster_list' will point to '.clusters_buf').
        //
        // Pin the 'Self' first (including its buffer), then write '.req.cluster_list'.
        //
        let mut me_pinned: Pin<Box<Self>> = {
            // The eventual 'Box<[u16]>' (both input and output cluster id's) needs to be available potentially throughout
            // the match protocol.
            //
            let clusters_tmp: Box<[u16]> = {
                let mut vec = Vec::with_capacity(IN_CLUSTERS.len() + OUT_CLUSTERS.len());
                vec.extend_from_slice(&IN_CLUSTERS);
                vec.extend_from_slice(&OUT_CLUSTERS);

                vec.into_iter().map(|x| x as u16).collect()
            };

            let me_tmp = Self {
                req: ezb_zdp_match_desc_req_field_s {
                    nwk_addr_of_interest: ShortAddr::GROUPCAST.0, // 0xFFFD
                    profile_id,
                    num_in_clusters: IN_CLUSTERS.len() as u8,
                    num_out_clusters: OUT_CLUSTERS.len() as u8,
                    cluster_list: std::ptr::null_mut(), // will be set
                },
                _clusters_buf: clusters_tmp,
                _pin: PhantomPinned,
            };

            Box::pin(me_tmp)
        };

        // Now that the '.clusters_buf' is pinned and stationary, we can write
        // '.req.cluster_list' (required by the C API) to point to it.
        unsafe {
            //let mut_ref = pinned_boxed.as_mut().get_unchecked_mut();
            //mut_ref.req.cluster_list = mut_ref.clusters_buf.as_ptr();

            me_pinned.req.cluster_list = me_pinned._clusters_buf.as_mut_ptr();
        }

        me_pinned
    }

    async fn r#match(&self) {
        unimplemented!()
    }

    async fn find_and_bind<T>(&self) -> T {
        unimplemented!()
    }
}
