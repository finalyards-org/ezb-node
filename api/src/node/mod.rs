/*
* Design:
*   - Singleton; there's only one Node in a running system
*   - Methods, not globals. Helps e.g. in IDE auto-completion: seeing only meaningful methods, based on context.
*       In comparison, C API has everything as global functions, which is overwhelming.
*   - Application RTOS task does not know about the underlying Zigbee task. The application is based
*       on Embassy's 'async' for its own concurrency.
*/
mod zigbee_task;

//r use std::sync::atomic::{AtomicBool, Ordering};
use zigbee_task::zigbee_task;

mod zigbee_dev;
pub(crate) use zigbee_dev::ZigbeeDev;

mod zigbee_guard;
pub(crate) use zigbee_guard::ZigbeeGuard;

#[cfg(feature = "dt_ias_cie")]
mod dt_ias_cie;
#[cfg(feature = "dt_ias_cie")]
pub use dt_ias_cie::{NodeApiIasCie, IasZoneEnrollResponse};

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel
};

use ezb_node_raw::{
    ezb_bdb_is_factory_new,
    ezb_bdb_open_network,
    ezb_bdb_start_top_level_commissioning,
    ezb_nwk_get_current_channel,
    ezb_nwk_get_extended_panid,
    ezb_nwk_get_extended_address,
    ezb_nwk_get_panid,
    ezb_nwk_get_short_address,
};

use crate::{
    AddrMode,
    AppSignal,
    BdbMode,
    ClusterId,
    ReportConfig,
    Config,
    Error,
    EzbError,
    IeeeAddr,
    ZclEvent,
};

// Channel:
//  - fed by the Zigbee task (or ISR, not sure); lock not needed within an ISR
//  - consumed by the _application task_; async; lock for every call
//
pub(self) static CHANNEL: Channel<CriticalSectionRawMutex, Payload, 10> = Channel::new();
    // tx: in 'zigbee_task'
    // rx: us, application task

const ZIGBEE_TASK_NAME: &str = "Zigbee_main";

//const ZIGBEE_TASK_STACK_SIZE: usize = 10 * 1024;   // note: C example uses 4k, Rust may need more
//  Let's just use the default, unless there's a specific reason to be ...specific.

#[derive(Debug, Clone)]
enum Payload {
    AppSignal(AppSignal),
    ZclEvent(ZclEvent)
}

/**
* Provides access to the radio. The struct implementing this decides the role (Coordinator/Router/EndDevice)
* that this has.
*
* tbd. We can provide 'Coordinator: Node' etc. traits, later, as a building block for custom nodes.
*
* A **singleton** - you can create only one and it has "endless" (static) lifespan.
*
* @thread Application RTOS thread, throughout.
*/
#[allow(async_fn_in_trait)]
pub trait Node {
    // Provided by the upper trait:
    async fn on_app_signal(&mut self, app_signal: AppSignal);
    fn on_zcl_event(&mut self, zcl_event: ZclEvent);

    /**
    * Initialize the Zigbee side of things.
    *
    * Launches the task that receives Zigbee events, converts them to Rust-friendly structs, and
    * sends them over to us, the application task.
    */
    fn init(cfg: &'static Config) -> Result<(), crate::Error> where Self: Sized {

        let _handle = std::thread::Builder::new()
            .name(ZIGBEE_TASK_NAME.to_string())    // visible e.g. in FreeRTOS monitoring
            //.stack_size(ZIGBEE_TASK_STACK_SIZE)

            .spawn(move || zigbee_task(cfg))
            .map_err(|e| { Error::SpawnFailed(e) })?;

        Ok(())
    }

    /**
    * Run the event loop, passing Zigbee events from the channel (an implementation detail) to the
    * handlers.
    */
    async fn run_loop(mut self) -> ! where Self: Sized {
        let rx = CHANNEL.receiver();

        loop {
            let x = rx.receive().await;
            log::trace!("Received: {:?}", &x);

            match x {
                Payload::AppSignal(x) =>  self.on_app_signal(x) .await,
                Payload::ZclEvent(x) => self.on_zcl_event(x)
            }
        }
    }
}

/// Provide applications' event handlers access to (parts of) the C level API.
///
impl<T: Node> NodeApi for T {}

/// Access to methods of the 'esp_zigbee_lib' from a context (application RTOS thread) that
/// requires locking.
///
/// @design
///     We could provide a '.guarded(|x| { x.aaa(); x.bbb() } )' interface that has the benefit
///     of allowing multiple calls per one lock. Currently, we don't. Everything is guarded,
///     but the application developer _does not need_ to know about it.
///
pub trait NodeApi {
    /**
    * Get the PAN ID of the network.
    */
    fn get_panid(&self) -> u16 {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            ezb_nwk_get_panid()
        }
    }

    /**
    * Get the extended PAN ID of the network.
    */
    fn get_extended_panid(&self) -> IeeeAddr {
        let _guard = ZigbeeGuard::acquire();
        let v = unsafe {
            ezb_nwk_get_extended_panid()
        };
        IeeeAddr(v)
    }

    /**
    * Get the extended (IEEE) address of the device.
    */
    fn get_extended_address(&self) -> IeeeAddr {
        let _guard = ZigbeeGuard::acquire();
        let v = unsafe {
            ezb_nwk_get_extended_address()
        };
        IeeeAddr(v)
    }

    /**
    * Get the network (short) address of the device.
    */
    fn get_short_address(&self) -> u16 {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            ezb_nwk_get_short_address()
        }
    }

    /**
    * Get the currently used channel.
    */
    fn get_current_channel(&self) -> u8 {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            ezb_nwk_get_current_channel()
        }
    }

    /**
    * @brief  Start top level commissioning procedure with specified mode mask.
    *
    * @note This function is intended to be a lower level, common tool. Applications should likely use
    *       node type -specific helper methods (and not directly the modes).
    *
    * Note: Some of the modes apply only to certain node types (NETWORK_FORMATION only to Coordinator
    *       role).
    *
    * Note: Modes are bit patterns.
    *
    * @note It's a little bit strange that the C library only lists 'EZB_ERR_NONE' as a return value.
    *       Does it always succeed? Is this a claim that future version might give errors? #feedback
    */
    fn start_top_level_commissioning(&self, mask: BdbMode) -> Result<(),EzbError> {
        let _guard = ZigbeeGuard::acquire();

        EzbError::wrap(unsafe {
            ezb_bdb_start_top_level_commissioning(mask.into())
        })
    }

    /**
    * Get the "factory new" status.
    *
    * A factory new device:
    *   - is not part of any network
    *   - has no stored keys
    *   - all settings (bindings, intervals) are at their defaults
    */
    fn is_factory_new(&self) -> bool {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            ezb_bdb_is_factory_new()
        }
    }

    /**
    * Open the network for joining.
    *
    * @param for_how_long After this time, the permit expires. Range is 1..254 seconds.
    *       Typical use in examples involving manual feeding of install codes is 180s.
    *
    * @note Since the call is synchronous (it only starts the join), error situations are not
    *       well understood: the C side can err on "not memory" (panic might be justified) and
    *       "on failed" (what is that, really??) - which is vague (again, panic).
    */
    fn open_network(&self, secs: JoinWindowSecs) -> () {

        let _guard = ZigbeeGuard::acquire();
        EzbError::wrap( unsafe {
            ezb_bdb_open_network(secs.0)
                // docs don't care to mention the unit (2.1.0), but examples use seconds.

        }).unwrap_or_else(|e| {
            panic!("'ezb_bdb_open_network' failed with: {}", e);
        });
    }

    /**
    * Read attributes of a remote device's cluster (ZCL "Read Attributes").
    *
    * @param dst_addr   Short (network) address of the remote device.
    * @param dst_ep     Remote endpoint hosting the cluster (server).
    * @param src_ep     Our endpoint.
    * @param attr_ids   Attribute IDs (cluster specific).
    *
    * @note The values arrive later, as a 'ZclEvent::ReadAttrResp'. With a sleepy end device, this happens when it
    *       next polls its parent (seconds; or longer).
    */
    fn read_attributes(&self, dst_addr: u16, dst_ep: u8, src_ep: u8, cluster: ClusterId, attr_ids: &[u16]) -> Result<(),EzbError> {
        use ezb_node_raw::{
            ezb_zcl_cmd_ctrl_t,
            ezb_zcl_read_attr_cmd_req,
            ezb_zcl_read_attr_cmd_t,
        };

        let mut ids: Vec<u16> = attr_ids.to_vec();     // C wants '*mut'

        let mut cmd: ezb_zcl_read_attr_cmd_t = crate::utils::c_zeroed();
        cmd.cmd_ctrl = ezb_zcl_cmd_ctrl_t {
            dst_addr: AddrMode::Short(dst_addr).into(),
            dst_ep,
            src_ep,
            cluster_id: cluster as u16,
            ..crate::utils::c_zeroed()  // standard command, direction "to server", default response enabled
        };
        cmd.payload.attr_number = ids.len() as u8;
        cmd.payload.attr_field = ids.as_mut_ptr();

        let _guard = ZigbeeGuard::acquire();

        // STRONG ASSUMPTION: the command is serialized during the call (i.e. 'ids' need not outlive it). #feedback
        EzbError::wrap(unsafe {
            ezb_zcl_read_attr_cmd_req(&cmd)
        })
    }

    /**
    * Configure a remote device to report attributes of one of its clusters to us (ZCL "Configure Reporting").
    *
    * The reports are sent to the destinations in the remote device's *binding table*, so normally you'd first
    * call 'bind_to_us' (for the same cluster).
    *
    * @note The result arrives as a 'ZclEvent::ConfigReportResp'; the reports themselves as 'ZclEvent::ReportAttr'.
    */
    fn configure_reporting(&self, dst_addr: u16, dst_ep: u8, src_ep: u8, cluster: ClusterId, records: &[ReportConfig]) -> Result<(),EzbError> {
        use ezb_node_raw::{
            ezb_zcl_cmd_ctrl_t,
            ezb_zcl_config_report_cmd_req,
            ezb_zcl_config_report_cmd_t,
            ezb_zcl_config_report_record_t,
        };

        let mut recs: Vec<ezb_zcl_config_report_record_t> = records.iter().map(|r| {
            let mut c: ezb_zcl_config_report_record_t = crate::utils::c_zeroed();
            c.direction = 0;    // EZB_ZCL_REPORTING_SEND: the remote (server) sends reports
            c.attr_id = r.attr_id;
            c.__bindgen_anon_1.client.attr_type = r.change.attr_type();
            c.__bindgen_anon_1.client.min_interval = r.min_secs;
            c.__bindgen_anon_1.client.max_interval = r.max_secs;
            c.__bindgen_anon_1.client.reportable_change = r.change.to_c();
            c
        }).collect();

        let mut cmd: ezb_zcl_config_report_cmd_t = crate::utils::c_zeroed();
        cmd.cmd_ctrl = ezb_zcl_cmd_ctrl_t {
            dst_addr: AddrMode::Short(dst_addr).into(),
            dst_ep,
            src_ep,
            cluster_id: cluster as u16,
            ..crate::utils::c_zeroed()
        };
        cmd.payload.record_number = recs.len() as u16;
        cmd.payload.record_field = recs.as_mut_ptr();

        let _guard = ZigbeeGuard::acquire();

        // STRONG ASSUMPTION: as in 'read_attributes'. #feedback
        EzbError::wrap(unsafe {
            ezb_zcl_config_report_cmd_req(&cmd)
        })
    }

    /**
    * Ask a remote device to bind one of its clusters to our endpoint (ZDO "Bind Request").
    *
    * Binding tells the remote where to send e.g. attribute reports. The binding is stored in the remote's
    * non-volatile memory.
    *
    * @param dst_addr   Short address of the remote device (the one whose binding table is changed).
    * @param src_ieee   IEEE address of the remote device (the source of the binding; i.e. the same device).
    * @param src_ep     Remote endpoint hosting the cluster.
    * @param our_ep     Our endpoint (the destination of the binding).
    *
    * @note The result is only logged (by the Zigbee task), for now.
    */
    fn bind_to_us(&self, dst_addr: u16, src_ieee: IeeeAddr, src_ep: u8, cluster: ClusterId, our_ep: u8) -> Result<(),EzbError> {
        use ezb_node_raw::{
            ezb_addr_t,
            ezb_zdo_bind_req,
            ezb_zdo_bind_req_t,
            ezb_zdp_bind_req_field_t,
            ezb_zdp_bind_req_result_t,
        };

        // Called in the Zigbee task. Must not block.
        unsafe extern "C" fn on_bind_result(result: *const ezb_zdp_bind_req_result_t, _user_ctx: *mut core::ffi::c_void) {
            let Some(r) = (unsafe { result.as_ref() }) else {
                log::error!("Bind result: (null)");
                return;
            };
            match unsafe { r.rsp.as_ref() } {
                Some(rsp) if r.error == 0 && rsp.status == 0 => log::info!("Bind: success"),
                Some(rsp) => log::warn!("Bind failed: error {}, ZDP status {:#04x}", r.error, rsp.status),
                None => log::warn!("Bind failed: error {} (no response)", r.error),
            }
        }

        let _guard = ZigbeeGuard::acquire();

        let our_ieee = unsafe { ezb_nwk_get_extended_address() };

        let req = ezb_zdo_bind_req_t {
            dst_nwk_addr: dst_addr,
            field: ezb_zdp_bind_req_field_t {
                src_addr: src_ieee.into(),
                src_ep,
                cluster_id: cluster as u16,
                dst_addr_mode: 0x03,    // 64-bit extended address (+ endpoint)
                dst_addr: ezb_addr_t { extended_addr: IeeeAddr(our_ieee).into() },
                dst_ep: our_ep,
            },
            cb: Some(on_bind_result),
            user_ctx: core::ptr::null_mut(),
        };

        // STRONG ASSUMPTION: as in 'read_attributes'. #feedback
        EzbError::wrap(unsafe {
            ezb_zdo_bind_req(&req)
        })
    }

    /**
    * @brief Perform "factory reset" procedure
    * @note The device will completely erase the `zb_storage` partition and then restart
    */
    #[cfg(false)]
    fn factory_reset(&self) {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            esp_zb_factory_reset()
        }
    }
}

/// Traditionally, a join window of 3 min is common and suggested (e.g. 'esp-zigbee-sdk' all examples
/// use it). However, this must be partly due to human delay in typing in codes, via e.g. an app.
///
/// If you provide pre-configured devices (see e.g. 'apps/demo/2/app.toml'), a window of 10s or 1min
/// should be quite enough.
///
#[derive(Debug, Clone)]
pub struct JoinWindowSecs(u8);

impl JoinWindowSecs {
    pub const _1MIN: Self = Self(60);
    /// Conventional 3 min window (e.g. all 'esp-zigbee-sdk' samples use this).
    pub const _3MIN: Self = Self(180);

    /// Allow non-default joining windows to be given. Note that 0xff is banned - as "joining forever".
    fn custom_secs(mut secs: u8) -> Self {
        if secs == 0xff {
            log::warn!("Changing {} -> {} (former would mean \"open forever\")", secs, 254);
            secs = 254;
        }
        Self(secs)
    }
}
