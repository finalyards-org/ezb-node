/*
* Finding other nodes and binding to them.
*/
use core::pin::Pin;
use core::marker::PhantomPinned;
use std::ffi::c_void;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, DynamicSender, DynamicReceiver},
};

use ezb_node_raw::{ezb_zcl_cluster_id_e, ezb_zdo_match_desc_req_s, ezb_zdp_match_desc_req_field_s, ezb_zdo_match_desc_req_result_s, ezb_zdp_match_desc_rsp_field_s, ezb_af_profile_id_e};

use crate::{
    ShortAddr,
    ZclError,
    ZdpError,
};

const CHANNEL_CAPACITY: usize = 8;

type ChannelT = Channel<CriticalSectionRawMutex, MatchResult, CHANNEL_CAPACITY>;

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

    pub(super) async fn start_matching(&self, src_ep: u8) {
        unimplemented!()
    }
}

//---
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
* An entry passed from the C side to Rust.
*/
pub(crate) type MatchResult = Result<MatchSuccess,MatchError>;

pub(crate) struct MatchSuccess {
    short_addr: ShortAddr,
    eps: Vec<u8>
}

pub(crate) enum MatchError {
    /// Zigbee Device Profile level (routing, timeout)
    ZdpError(ZdpError),
    //r /// Zigbee Cluster Library level (e.g. device does not support a cluster)
    //r Zcl(ZclError),
    Error(core::ffi::c_int)
}

/***r enum MatchStatus {
    Success{ short_addr: ShortAddr, eps: Vec<u8> },
    Error(core::ffi::c_int),
    ZdpError(ZdpError)
}***/

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
fn parse(p: *const ezb_zdo_match_desc_req_result_s) -> Option<MatchResult> {
    use MatchError::{Error, ZdpError};

    let resp = unsafe { p.as_ref() }?;

    if resp.error != 0 {
        Some( Err( Error(resp.error) ) )
    } else {
        let rsp = unsafe { resp.rsp.as_ref() }?;

        let st = crate::ZdpError::parse(rsp.status)?;
        if let Some(e) = st {
            Some( Err( ZdpError(e) ))
        } else {
            let ezb_zdp_match_desc_rsp_field_s {
                nwk_addr_of_interest,
                match_length,
                match_list,
                status: _ // ignore
            } = *rsp;

            let short_addr = ShortAddr::from(nwk_addr_of_interest);

            let match_list = unsafe { match_list.as_ref() }?;

            let eps: Vec<u8> = {
                let a = unsafe {
                    core::slice::from_raw_parts(match_list, match_length as usize)
                };
                a.to_vec()
            };
            Some( Ok( MatchSuccess{ short_addr, eps }) )
        }
    }
}
