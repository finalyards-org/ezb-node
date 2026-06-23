use core::ffi::c_void;
use core::marker::PhantomPinned;
use core::pin::Pin;

use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, DynamicReceiver, DynamicSender},
};

use async_stream::stream;
use futures_util::Stream;

use ezb_node_raw::{
    ezb_af_profile_id_e,
    ezb_zcl_cluster_id_e,
    ezb_zdo_match_desc_req_result_s,
    ezb_zdo_match_desc_req_s,
    ezb_zdp_match_desc_req_field_s,
    ezb_zdp_match_desc_rsp_field_s,
    ezb_err_e,
};

use super::{
    AccessorCtx,
    MatchEvent,
};

use crate::{
    Node,
    ShortAddr,
    ZclError,
    ZdpError,
};

const CHANNEL_CAPACITY: usize = 8;

type ChannelT = Channel<ThreadModeRawMutex, MatchInnerEvent, CHANNEL_CAPACITY>;

/**
* Carries out a groupcast call on the Zigbee network, listing all devices (which are receiving on idle) that match
* the provided set of 'in' and 'out' cluster id's.
*
* Lifespan:
*   Remains valid throughout the matching mechanism. This also ensures that what we place in the
*   fields remains available for the C API, until the matching has ended.
*/
pub(super) struct MatchingContext {
    // For safety, keep pinned until the end of the matching process.
    req: ezb_zdo_match_desc_req_s,

    // pointed to by 'req.field.cluster_list'
    _clusters: Box<[u16]>,

    // needs to be pinned; pointer to this passed on as '.user_ctx' (void pointer) in C API.
    channel: ChannelT,

    // Pins 'Self' to heap, not to be moved.
    _pin: PhantomPinned,
}

impl MatchingContext {
    /**
    * Create a pinned matcher (lives in the heap; stationary, i.e. fields can be passed to C APIs).
    */
    pub(super) fn new_pinned(in_clusters: &[ezb_zcl_cluster_id_e], out_clusters: &[ezb_zcl_cluster_id_e]) -> Pin<Box<Self>> {
        use ezb_af_profile_id_e::EZB_AF_HA_PROFILE_ID; // HA = Home Automation

        // Create a channel. The sending end is placed in the '.req.useer_ctx' so we can feed it from the C callback
        // (even if there were multiple matches going on).
        //
        let channel: ChannelT = Channel::new();

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
                        profile_id: EZB_AF_HA_PROFILE_ID as u16,
                        num_in_clusters: in_clusters.len() as u8,
                        num_out_clusters: out_clusters.len() as u8,
                        cluster_list: core::ptr::null_mut(), // will be set
                    },
                    cb: Some(match_c_callback),
                    user_ctx: core::ptr::null_mut(), // will be set
                },
                _clusters: clusters,
                channel,
                _pin: PhantomPinned,
            };

            Box::pin(o)
        };

        // Now that the struct is pinned (i.e. stationary), we can:
        //  - write '.req.field.cluster_list' (required by the C API) to point to '._clusters' that owns the data
        //  - write '.user_ctx' to '.channel' (allows C callback to feed the channel)
        //
        // Note! Why not pass just 'DynamicSender' to the C callback?
        //      It's a "fat pointer", and does not cast to '*const c_void'. 'Channel' does.
        {
            let poke = unsafe { me_pinned.as_mut().get_unchecked_mut() };

            poke.req.field.cluster_list = poke._clusters.as_mut_ptr();
            poke.req.user_ctx = &poke.channel as *const _ as *mut c_void;
        }

        me_pinned
    }

    /**
    * Initiate the matching.
    *
    * @note: In application task.
    */
    pub(super) fn start_matching<F>(&self, node: &dyn Node, src_ep: u8) -> impl Stream<Item = MatchEvent> {

        let rx: DynamicReceiver<MatchInnerEvent> = self.channel.dyn_receiver();

        let acc_fact = |dst_addr: ShortAddr, dst_ep: u8| {
            AccessorCtx::new(node, src_ep, dst_addr, dst_ep)    // picks 'node', 'src_ep' from the environment
        };

        stream! {
            // This code block gets executed, in a lazy manner, only once the application reads the stream.

            loop {
                let tmp = rx.receive() .await;
                match tmp {
                    MatchInnerEvent::Bound(short_addr, eps) => {

                        for ep in eps {
                            let acc = acc_fact(short_addr, ep);
                            yield MatchEvent::Bound(acc);
                        }
                    },
                    MatchInnerEvent::Error(ezb_err_e::_ERR_TIMEOUT) => {
                        todo!()
                    },
                    MatchInnerEvent::ZdpError(v) => {
                        todo!()
                    },
                }
            }

            /***R
            while let Ok(res) = rx.recv().await {

                res match {
                    Ok() =>


                }

                yield event;

                if matches!(event, MatchResult::Finished | MatchResult::Failed(_)) {
                    break;
                }
            }***/
        };
    }
}

/**
* The C callback.
*
* @note Called within the Zigbee task.
*/
extern "C" fn match_c_callback(
    response: *const ezb_zdo_match_desc_req_result_s,
    user_ctx: *mut c_void
) {
    // Cast context back. We only need the writer.
    let tx = {
        let channel: &ChannelT = unsafe { &*(user_ctx as *const ChannelT) };
        channel.dyn_sender()
    };

    let Some(resp) = parse(response) else {
        log::error!("Unable to parse match response (skipped).");
        return;
    };

    if let Err(err) = tx.try_send(resp) {
        log::error!("Unable to send 'MatchResult' (skipped!); please try increasing the 'CHANNEL_CAPACITY'.");
    }
}

/**
* The _inner_ delivery mechanism, from Zigbee task to application task.
*/
pub(super) enum MatchInnerEvent {
    Bound(ShortAddr, Vec<u8>),

    /// Zigbee Device Profile errors; we did reach the other node
    ZdpError(ZdpError),

    /// Local (early) errors; did not reach outer nodes
    Error(ezb_err_e)
        // ezb_err_e::_ERR_NONE as u8,  // 0
        // ezb_err_e::_ERR_FAIL as u8,  // -1 (0xff)
        // ezb_err_e::_ERR_NO_MEM as u8,
        // ezb_err_e::_ERR_INV_ARG as u8,
        // ezb_err_e::_ERR_INV_STATE as u8,
        // ezb_err_e::_ERR_INV_SIZE as u8,
        // ezb_err_e::_ERR_NOT_FOUND as u8,
        // ezb_err_e::_ERR_NOT_SUPPORTED as u8,
        // ezb_err_e::_ERR_TIMEOUT as u8,
        // ezb_err_e::_ERR_ABORT as u8,
        // ezb_err_e::_ERR_BUSY as u8,
        // ezb_err_e::_ERR_NOT_FINISHED as u8,
        // ezb_err_e::_ERR_NOT_ALLOWED as u8,
        // ezb_err_e::_ERR_PARSE as u8,
        // ezb_err_e::_ERR_EMPTY_DATA as u8,
        // ezb_err_e::_ERR_DROP as u8,
        // ezb_err_e::_ERR_SECURITY as u8,
}

/**
* Parse the match response from C structure.
*
* @return None if the data is bad (NULL pointers); Some if valid.
*/
//pub struct ezb_zdo_match_desc_req_result_s {
//     ///< Error code of the match descriptor request operation
//     pub error: ezb_err_t,
//     ///< Pointer to the match descriptor response field, NULL if error occurred
//     pub rsp: *mut ezb_zdp_match_desc_rsp_field_t,
// }
//
//pub struct ezb_zdp_match_desc_rsp_field_s {
//     ///< Status of the match descriptor request, see @ref ezb_zdp_status_t
//     pub status: ezb_zdp_status_t,
//     ///< Network address of the device that was queried
//     pub nwk_addr_of_interest: ezb_shortaddr_t,
//     ///< Number of endpoints on the remote device that match the request
//     /// criteria
//     pub match_length: u8,
//     ///< Pointer to array of endpoint numbers (uint8_t) that match the criteria
//     pub match_list: *mut u8,
// }
fn parse(p: *const ezb_zdo_match_desc_req_result_s) -> Option<MatchInnerEvent> {
    let resp = unsafe { p.as_ref() }?;

    if resp.error != 0 {
        Some( MatchInnerEvent::Error(resp.error) )
    } else {
        let rsp = unsafe { resp.rsp.as_ref() }?;

        let st = crate::ZdpError::parse(rsp.status)?;
        if let Some(e) = st {
            Some( MatchInnerEvent::ZdpError(e) )
        } else {
            let ezb_zdp_match_desc_rsp_field_s {
                nwk_addr_of_interest,
                match_length,
                match_list,
                status: _ // ignore
            } = *rsp;

            let short_addr = ShortAddr::from(nwk_addr_of_interest);

            let eps = unsafe {
                match_list.as_ref().map(|x| {
                    core::slice::from_raw_parts(match_list, match_length as usize)
                })
            }?;

            Some( MatchInnerEvent::Bound(short_addr, eps.to_vec()) )
        }
    }
}
