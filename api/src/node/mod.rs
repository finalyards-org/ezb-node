/*
* Design:
*   - Singleton; there's only one Node in a running system
*   - Methods, not globals. Helps e.g. in IDE auto-completion (and generally not being able to see weird stuff
*       outside their context). In comparison, C API has everything as global functions, which is overwhelming.
*   - Application task does not (need to) know about the underlying Zigbee task; it's an Embassy application
*     and can use 'async' for its own concurrency.
*
* Note:
*   - Native mode only; no RCP (radio co-processor) implementation. C API has both.
*/

mod zigbee_task;
use zigbee_task::zigbee_spawn;

use std::{
    boxed::Box,
    sync::OnceLock,
};

use bitflags::bitflags;
use esp_idf_svc::{
    sys::EspError
};
use log;

use ezb_node_raw::{
    esp_zigbee_init,
    ezb_app_signal_t,
    ezb_bdb_start_top_level_commissioning,
    ezb_bdb_is_factory_new,
    ezb_nwk_get_panid,
    ezb_nwk_get_extended_panid,
    ezb_nwk_get_short_address,
    ezb_nwk_get_current_channel,
    ezb_bdb_comm_mode_t,
    esp_zigbee_config_t,
    ezb_bdb_set_primary_channel_set,
    ezb_bdb_set_secondary_channel_set,
    esp_zigbee_launch_mainloop,
    esp_zigbee_start,
    ezb_app_signal_get_params,
    ezb_app_signal_get_type,
};

use crate::{
    AppSignal,
    DeviceDescriptorView,
    IeeeAddr,
    Error::{
        AlreadyInUse,
        InitializationFailed
    },
    config_views::PlatformDeviceView,
};

use ezb_node_config::ChannelMask;

/**
* Provides access to the radio. The application struct implementing this decides the role (Coordinator/Router/EndDevice)
* that this has.
*
* A **singleton** - you can create only one and it has "endless" (static) lifespan.
*
* @thread Call from application RTOS thread.
*/
pub trait Node {
    /**
    * Callback on Zigbee events.        // tbd. EDIT!
    */
    //r fn on_app_signal(&self, sig: AppSignal);
    fn poll_app_signal(&self) -> Option<AppSignal> {
        todo!()
    }

    /**
    * Initialize a 'Node' from a given configuration.
    */
    #[cfg(false)] //R
    fn init<'a>(cv: impl Into<PlatformDeviceView<'a>>) -> Result<(),crate::Error> {
        let (cc, channel_masks) = cv.into().expand();

        // esp_err_t esp_zigbee_init(const esp_zigbee_config_t *config);
        //
        let err = unsafe { esp_zigbee_init(cc) };
            //
            // 'esp_zigbee_init()' takes a pointer, so we must assume it can read that memory, later.
            // Providing it a 'static', non-changing struct is safe.

        EspError::from(err).map_or(Ok(()), |e| { Err(InitializationFailed(e)) })?;

        Self::set_channel_sets(&channel_masks);
        Ok(())
    }

    /**
    * Add endpoints to an initialized 'Node' (before starting it).
    */
    fn add_endpoints<'a>(cv: impl Into<DeviceDescriptorView<'a>>) -> Result<(),crate::Error> {
        todo!()
    }

    /**
    * Launch the task that receives Zigbee events, converts them to Rust structs, and sends them to a FIFO that
    * the application task can read.
    */
    // 'auto_start': we might get rid of this parameter. It has to do with the application initialization logic.
    //      C example uses delayed hardware init. If the value is 'true', the Zigbee network needs to be later
    //      activated by a call to '...'.
    //
    fn spawn(self, cfg: &'static PlatformDeviceView, auto_start: bool) -> Result<(), std::io::Error> where Self: Sized {
        zigbee_spawn(cfg, auto_start)
    }

    /**
    * Get the PAN ID of the network.
    */
    fn get_panid(&self) -> u16 {
        unsafe {
            ezb_nwk_get_panid()
        }
    }

    /**
    * Get the extended PAN ID of the network.
    */
    fn get_extended_panid(&self) -> IeeeAddr {
        let v = unsafe {
            ezb_nwk_get_extended_panid()
        };
        IeeeAddr::from(v)
    }

    /**
    * Get the network (short) address of the device.
    */
    fn get_short_address(&self) -> u16 {
        unsafe {
            ezb_nwk_get_short_address()
        }
    }

    /**
    * Get the currently used channel.
    */
    fn get_current_channel(&self) -> u8 {
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
    */
    fn start_top_level_commissioning(&self, mask: CommissioningModesMask) -> Option<EspError> {
        let err= unsafe {
            ezb_bdb_start_top_level_commissioning(mask.bits())
        };
        EspError::from(err) // provides 'Option'
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
        unsafe {
            ezb_bdb_is_factory_new()
        }
    }

    /**
    * @brief Perform "factory reset" procedure
    * @note The device will completely erase the `zb_storage` partition and then restart
    */
    #[cfg(false)]
    fn factory_reset(&self) {
        unsafe {
            esp_zb_factory_reset()
        }
    }
}

type BdbMode = ezb_bdb_comm_mode_t;
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CommissioningModesMask: u8 {
        const INITIALIZATION = BdbMode::EZB_BDB_MODE_INITIALIZATION.0 as u8; // 1
        #[cfg(feature = "touchlink")]
        const TOUCHLINK_INITIATOR = BdbMode::EZB_BDB_MODE_TOUCHLINK_INITIATOR.0 as u8; // 2
        const NETWORK_STEERING = BdbMode::EZB_BDB_MODE_NETWORK_STEERING.0 as u8; // 4
        #[cfg(feature = "coordinator")]
        const NETWORK_FORMATION = BdbMode::EZB_BDB_MODE_NETWORK_FORMATION.0 as u8; // 8
        const FINDING_N_BINDING = BdbMode::EZB_BDB_MODE_FINDING_N_BINDING.0 as u8; // 16
        #[cfg(feature = "touchlink")]
        const TOUCHLINK_TARGET = BdbMode::EZB_BDB_MODE_TOUCHLINK_TARGET.0 as u8; // 32

        // Declare all bits as "known". Recommended for 'bitflags', when working with C library APIs.
        const _ = !0;
    }
}

/*
* Handler for Zigbee APP signals.
*/
// typedef void *ezb_app_signal_t;
// typedef uint16_t ezb_app_signal_type_t;
//
#[unsafe(no_mangle)]
extern "C" fn esp_zb_app_signal_handler(p_app_signal: *const ezb_app_signal_t) {

    let p_type = unsafe { ezb_app_signal_get_type(p_app_signal) };
    let p_params = unsafe { ezb_app_signal_get_params(p_app_signal) };

    AppSignal::from(p_type, p_params)
        .map(|sig| {
            log::info!("Received: {}", sig);
        })
        .unwrap_or_else(|| {

            log::error!("Unexpected app signal: {}, {:?}", p_type, p_params);
        });
}

/**
* Internal helper.
*
* Collects about-to-be-registered endpoints, and passes them to the C 'esp-zigbee-lib', all at once.
*/
#[cfg(false)]   // is it needed in 2.0?
struct RegState(*mut esp_zb_ep_list_t);

#[cfg(false)]
impl RegState {
    fn new() -> Self {
        let l = unsafe { esp_zb_ep_list_create() };
        Self(l)
    }

    fn add(&self, cluster_list: *mut esp_zb_cluster_list_t, ep_cfg: esp_zb_endpoint_config_t) {
        unsafe {
            esp_zb_ep_list_add_ep(self.0, cluster_list, ep_cfg);   // tbd. handle error
        }
    }

    fn register(self) {
        unsafe {
            esp_zb_device_register(self.0);    // tbd. errors
        }
    }
}
