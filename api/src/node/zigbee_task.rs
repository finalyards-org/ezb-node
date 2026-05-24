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

const TASK_NAME: &str = "Zigbee_main";
const TASK_STACK_SIZE: usize = 20 * 1024;   // note: C example uses 4k, Rust may need more

/**
*/
pub(crate) fn zigbee_spawn(cfg: &'static PlatformDeviceView, auto_start: bool) -> Result<(),std::io::Error> {

    let _ = std::thread::Builder::new()
        .name(TASK_NAME.to_string())    // visible e.g. in FreeRTOS monitoring
        .stack_size(TASK_STACK_SIZE)    // Rust: 20000
        .spawn(move || {
            log::info!("Zigbee task running");

            zigbee_init(cfg).unwrap_or_else(|e| {
                log::error!("Zigbee initialization failed: {:?}", e);
                loop {}
            });

            zigbee_run(auto_start).unwrap_or_else(|e| {
                log::error!("Zigbee task failed: {:?}", e);
                loop {}
            });
            // tbd. revise the above
        })?;

    Ok(())  // right after the thread is successfully spawned
}

/**
* Zigbee init
*
* @note: Since this is done in the Zigbee task in the C example, thought best to do the same.
*/
fn zigbee_init(cfg: &PlatformDeviceView) -> Result<(),crate::Error> {
    let (cc, channel_masks) = cfg.expand();

    // esp_err_t esp_zigbee_init(const esp_zigbee_config_t *config);
    //
    let err = unsafe { esp_zigbee_init(cc) };
        //
        // 'esp_zigbee_init()' takes a pointer, so we must assume it can read that memory, later.
        // Providing it a 'static', non-changing struct is safe.

    EspError::from(err).map_or(Ok(()), |e| { Err(InitializationFailed(e)) })?;

    zigbee_set_channel_sets(&channel_masks);
    Ok(())
}

/**
* Set the primary and secondary channel mask, on the C library side.
*
* @note This (for primary) "should be called [...] after 'ezb_core_init()' and before 'ezb_dev_start()'".
*       "If function is not called, by default it will scan all channels or read from zb_fct NVRAM zone if available." (1.x docs)
*       -- but we call it every time.
*/
fn zigbee_set_channel_sets(channel_masks: &[ChannelMask;2]) {

    for (primary,cm) in [true,false].into_iter().zip(channel_masks) {
        let err = unsafe {
            if primary {
                ezb_bdb_set_primary_channel_set(cm.bits())
            } else {
                ezb_bdb_set_secondary_channel_set(cm.bits())
            }
        };
        // EZB_ERR_NONE
        // EZB_ERR_INVALID_ARG  should not happen: 'ChannelMask'

        // Since the config is validated already at compilation, we are not expecting a failure, here (thus panic).
        //
        EspError::from(err).map(|e| {
            panic!("Setting {} channel set failed: {}", if primary {"primary"} else {"secondary"}, e);
        });
    }
}

/**
* Zigbee main task.
*
* @param auto_start
*   'true' for automatic start of the Zigbee stack
*   'false' for delayed start, needing a call to '.start_top_level_commissioning()' at a later stage.
*/
fn zigbee_run(auto_start: bool) -> Result<!,EspError> {
    let err = unsafe {
        esp_zigbee_start(auto_start)
    };
    EspError::from(err).map_or(Ok(()), Err)?;

    let err = unsafe {
        esp_zigbee_launch_mainloop()
    };
    EspError::from(err).map_or(Ok(()), Err)?;

    // In C code, here's 'esp_zigbee_deinit()'.
    // Do we expect to return from the main loop?
    //
    unreachable!()
    //panic!("Returned from main loop, unharmed.");   // if we get here, we could do the "deinit" and just return '()'
}

