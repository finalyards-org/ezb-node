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
    ezb_aps_secur_enable_distributed_security,
    ezb_app_signal_add_handler,
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

    let inner = || -> Result<(),EspError> {
        // esp_err_t esp_zigbee_init(const esp_zigbee_config_t *config);
        //
        EspError::from(unsafe { esp_zigbee_init(cc) }).map_or(Ok(()), Err)?;
            //
            // 'esp_zigbee_init()' takes a pointer, so we must assume it can read that memory, later.
            // Providing it a 'static', non-changing struct is safe.

        EspError::from(unsafe { ezb_aps_secur_enable_distributed_security(false) }).map_or(Ok(()), Err)?;

        // Since the configs are validated at compilation, we are not expecting a failure.
        //
        [ezb_bdb_set_primary_channel_set, ezb_bdb_set_secondary_channel_set].into_iter()
            .zip( channel_masks.map(|cm| cm.bits() ))
            .for_each(|(f, mask)| {
                let err = unsafe { f(mask) };
                    // EZB_ERR_NONE
                    // EZB_ERR_INVALID_ARG  should not happen: 'ChannelMask'

                EspError::from(err).unwrap_or_else(|e| {
                    panic!("Unexpected problem setting channel masks: {}", e);
                });
            });

        EspError::from({ unsafe { ezb_app_signal_add_handler(Some(app_signal_handler)) } })
            .map_or(Ok(()), Err)
            // Rust FFI note: 'Option<&fn>' is Rust FFI's way to present a function pointer, heading to a C interface.
            //      Rust has a strict "never-null" rule for function pointers; this jumps around that.
    };
    inner().map_err(|e| { InitializationFailed(e) })
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

/**
* Handler for Zigbee APP signals.
*/
// typedef void *ezb_app_signal_t;
// typedef uint16_t ezb_app_signal_type_t;
//
#[unsafe(no_mangle)]
extern "C" fn app_signal_handler(p_app_signal: *const ezb_app_signal_t) -> bool {

    let p_type = unsafe { ezb_app_signal_get_type(p_app_signal) };
    let p_params = unsafe { ezb_app_signal_get_params(p_app_signal) };

    AppSignal::from(p_type, p_params)
        .map(|sig| {
            log::info!("Received: {}", sig);

            // todo: Push 'sig' to a channel
        })
        .unwrap_or_else(|| {
            log::error!("Unexpected app signal: {}, {:?}", p_type, p_params);
        });

    todo!()     // when to return 'true'?
}
