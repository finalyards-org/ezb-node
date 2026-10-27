#![cfg(feature = "_remote_any")]

use core::{
    ffi::c_void,
    marker::{PhantomData, PhantomPinned},
    pin::Pin,
};

use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, DynamicReceiver, TrySendError},
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
};

use super::{
    Accessor,
    MatchEvent,
};

use crate::{
    error::EzbError,
    ShortAddr,
    ZdpError
};

const CHANNEL_CAPACITY: usize = 8;

type ChannelT = Channel<ThreadModeRawMutex, Option<(ShortAddr,MatchResponse)>, CHANNEL_CAPACITY>;

/**
* Carries out a groupcast call on the Zigbee network, listing all devices (which are receiving on
* idle) that match the provided set of 'in' and 'out' cluster id's.
*
* Lifespan:
*   Remains valid throughout the matching mechanism. This also ensures that what we place in the
*   fields remains available for the C API, until the matching has ended.
*/
pub(super) struct MatchingContext<A: Accessor> {
    // For safety, keep pinned until the end of the matching process.
    //  .req.field.cluster_list -> _clusters
    //  .req.user_ctx -> self
    req: ezb_zdo_match_desc_req_s,

    // pointed to by 'req.field.cluster_list'
    _clusters: Box<[u16]>,

    // needs to be pinned; pointer to this passed on as '.user_ctx' (void pointer) in C API.
    channel: ChannelT,

    // Pins 'Self' to heap, not to be moved.
    _pin: PhantomPinned,

    _marker: PhantomData<A>,
}

// By defining 'MatchingContextPinned', we can create methods. On 'Pin<Box<..>>', we cannot.
//
#[cfg(false)]   // complex
struct MatchingContextPinned(Pin<Box<MatchingContext>>);

impl<A: Accessor> MatchingContext<A> {
    /**
    * Create a pinned matcher (lives in the heap; stationary, i.e. fields can be passed to C APIs).
    */
    pub(super) fn new_pinned() -> Pin<Box<Self>> {
        use ezb_af_profile_id_e::HA_PROFILE_ID; // HA = Home Automation

        let in_cls: &[ezb_zcl_cluster_id_e] = A::IN_CLUSTERS;
        let out_cls: &[ezb_zcl_cluster_id_e] = A::OUT_CLUSTERS;

        // Create a channel. The sending end is placed in the '.req.user_ctx' so we can feed it from the C callback
        // (even if there were multiple matches going on).
        //
        let channel: ChannelT = Channel::new();

        let clusters: Box<[u16]> = {
            let mut vec = Vec::with_capacity(in_cls.len() + out_cls.len());
            vec.extend_from_slice(&in_cls);
            vec.extend_from_slice(&out_cls);

            vec.into_iter().map(|x| x as _).collect()
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
                        profile_id: HA_PROFILE_ID as u16,
                        num_in_clusters: in_cls.len() as u8,
                        num_out_clusters: out_cls.len() as u8,
                        cluster_list: core::ptr::null_mut(), // will be set
                    },
                    cb: Some(match_c_callback ::<A>),
                    user_ctx: core::ptr::null_mut(), // will be set
                },
                _clusters: clusters,
                channel,
                _pin: PhantomPinned,
                _marker: PhantomData,
            };

            Box::pin(o)
        };

        // Now that the struct is pinned (i.e. stationary), we can:
        //  - write '.req.field.cluster_list' (required by the C API) to point to '._clusters' that owns the data
        //  - write '.user_ctx' to 'self' (we only need the channel, but..)
        {
            let poke = unsafe { me_pinned.as_mut().get_unchecked_mut() };

            poke.req.field.cluster_list = poke._clusters.as_mut_ptr();
            poke.req.user_ctx = &poke as *const _ as *mut c_void;    // points to 'Self'
        }

        me_pinned
    }

    /**
    * Initiate the matching.
    *
    * @note: In application task.
    */
    pub(super) fn start_matching(self: Pin<Box<MatchingContext<A>>>) -> impl Stream<Item = (ShortAddr, MatchEvent<A>)>
    {
        // Note: It matters to the lifetimes that things are done *within* the 'stream!' block. The inside lasts
        //      longer than the function body.
        //
        stream! {
            // This code block gets executed, in a lazy manner, only once the application reads the stream.

            let rx: DynamicReceiver<'_, Option<(ShortAddr,MatchResponse)>> = self.channel.dyn_receiver();

            // 'None' is a marker that the channel will terminate
            //
            while let Some((remote_addr,ev)) = rx.receive() .await {

                match ev {
                    MatchResponse::Matches(eps) => {
                        // When 'eps' is empty (remote node reporting they don't have matching end points),
                        // it gets consumed here - no yield, because no accessor generated.

                        // We are at a decision, here. tbd.
                        //
                        //  Who does actual binding?
                        //      Leaving it now to the application level, but - at least when we deal with
                        //      pre-configured devices - we could (should!) simply bind automatically.
                        //
                        for ep in eps {
                            let acc = A::new();
                            yield (remote_addr, MatchEvent::Found(ep, acc));
                        }
                    },
                    MatchResponse::Error(v) => {
                        yield (remote_addr, MatchEvent::Error(v));
                    }
                }
            }

            // Extra safety - avoids dropping.
            //
            //  - we let the 'MatchingContext' remain on heap (leaking memory), with its
            //      '.req.user_ctx' set to NULL. This will show us, whether getting '.error'
            //      really is the final call (from C library).
            //
            if cfg!(feature = "_extra_safety") {
                log::debug!("Extra caution: writing '.user_ctx' to NULL");

                let ptr = core::ptr::addr_of!(self.req.user_ctx) as *mut *mut c_void;
                unsafe{ ptr.write( core::ptr::null_mut() ) };

                core::mem::forget(self);
            }

            // End of stream will also drop the channel.
        } // stream!
    }
}

/**
* The C callback.
*
* @note Called within the Zigbee task.
*
* @note Google AI strongly suggests using the 'user_ctx' to point to the _whole_ 'MatchingContext<A>'.
*       We only really need the channel, or a sender to it. It's reasons are aliasing based concerns.
*/
extern "C" fn match_c_callback<A: Accessor>(
    response: *const ezb_zdo_match_desc_req_result_s,
    user_ctx: *mut c_void
) {
    // If this happens, the callback gets called even after a payload carrying an error.
    // (needs 'feature = "_extra_safety"' to be set)
    assert!( !user_ctx.is_null(), "C callback: 'user_ctx'==NULL");

    let Some(res) = parse(response) else {
        log::error!("Unable to parse match response (skipped): {:?}", unsafe { *response });
        return;
    };

    let ctx = & unsafe { &*(user_ctx as *const MatchingContext<A>) };
    let tx = ctx.channel.sender();  // preferable over '.dyn_sender()' (less CPU; relatively frequent calls)

    match res {
        // If a local error (or timeout), terminate the whole binding process.
        Err(e) => {
            log::info!("Match callback received an error (local error or timeout) -> not observing more events: {}", e);

            // Embassy channel has no '.close()' - so we use a sentinel to stop the consumption.
            //
            if let Err(err) = tx.try_send(None) {
                match err {
                    TrySendError::Full(_) => {
                        log::error!("Channel full; cannot close!");
                    },
                    // exhaustive match
                }
            }
            return;
        },
        Ok(ev) => {
            if let Err(err) = tx.try_send(Some(ev)) {
                match err {
                    TrySendError::Full(_) => {
                        log::error!("Channel full; capacity ({}) reached! (event lost)", ctx.channel.capacity());
                    },
                    // exhaustive match
                }
            }
        }
    }
}

/**
* The _inner_ delivery mechanism. This is only for the inter-task communications (Zigbee task ->
* application task).
*/
#[derive(Debug)]
pub(super) enum MatchResponse {
    /// Successful binding
    ///
    /// @note Same Zigbee message may carry multiple endpoints (rare, but may).
    ///
    Matches(Vec<u8>),

    /// Zigbee Device Profile errors; response from another node.
    Error(ZdpError),
}

/**
* Parse the match response from C structure.
*
* @return None if the data is bad (NULL pointers)
*       Some if valid - can still carry an error from the remote device.
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
fn parse(p: *const ezb_zdo_match_desc_req_result_s) -> Option<Result<(ShortAddr,MatchResponse),EzbError>> {
    let resp = unsafe { p.as_ref() }?;

    if let Err(e) = EzbError::wrap(resp.error) {
        Some(Err(e))

    } else {
        // Success response
        let rsp = unsafe { resp.rsp.as_ref() }?;

        let ezb_zdp_match_desc_rsp_field_s {
            nwk_addr_of_interest,
            match_length,
            match_list,
            status
        } = *rsp;

        let zdperr_opt = ZdpError::parse(status)?;

        let event = if let Some(e) = zdperr_opt {
            MatchResponse::Error(e)

        } else {
            let tmp = unsafe { match_list.as_ref() };

            let eps = match tmp {
                None if match_length == 0 => {
                    // Zigbee note: NULL is perfectly okay (combined with 'match_length' == 0): the
                    //      remote end point responds that it does not have matching clusters.
                    //
                    &[]
                },
                Some(ptr) if match_length > 0 => {
                    unsafe {
                        core::slice::from_raw_parts(ptr, match_length as usize)
                    }
                },

                // Rest are corrupt cases (OR 'match_list' != NULL, 'match_length' ==0 can be tolerated,
                // taking the pointer as one that should not be used. Change the above case if it needs to.
                //
                _ => {
                    return None;
                }
            };

            MatchResponse::Matches(eps.to_vec())
        };

        let remote_addr = ShortAddr::from(nwk_addr_of_interest);

        Some(Ok((remote_addr, event)))
    }
}
