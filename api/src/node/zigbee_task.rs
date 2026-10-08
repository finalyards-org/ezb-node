//use bitflags::bitflags;
use log;
use std::{
    boxed::Box,
    collections::BTreeMap,
    sync::atomic::AtomicBool
};

use embassy_sync::{
    blocking_mutex::ThreadModeMutex,
    channel::{
        Channel,
        DynamicReceiver,
        DynamicSender,
    }
};

use ezb_node_raw::{
    //esp_zigbee_config_t,
    //esp_zigbee_init,
    esp_zigbee_launch_mainloop,
    esp_zigbee_start,
    //ezb_af_create_device_desc,
    //ezb_af_device_add_endpoint_desc,
    //ezb_af_device_desc_register,
    //ezb_af_device_desc_t,
    //ezb_af_endpoint_get_cluster_desc,
    ezb_app_signal_add_handler,
    ezb_app_signal_get_params,
    ezb_app_signal_get_type,
    ezb_app_signal_t,
    //ezb_aps_secur_enable_distributed_security,
    //ezb_bdb_comm_mode_t,
    ////ezb_bdb_is_factory_new,
    //ezb_bdb_set_primary_channel_set,
    //ezb_bdb_set_secondary_channel_set,
    ////ezb_bdb_start_top_level_commissioning,
    //ezb_nwk_get_current_channel,
    //ezb_nwk_get_extended_panid,
    //ezb_nwk_get_panid,
    //ezb_nwk_get_short_address,
    ezb_secur_set_ic_required,
    //ezb_zcl_basic_cluster_desc_add_attr,
    //ezb_zcl_basic_server_attr_t,
    //ezb_zcl_cluster_desc_t,
    //ezb_zcl_cluster_id_e,
    ezb_zcl_core_action_callback_id_e,
    ezb_zcl_core_action_callback_id_t,
    ezb_zcl_core_action_handler_register,
    //ezb_zha_create_color_dimmable_light,
};

use crate::{
    AppSignal,
    Config,
    EndpointCreator,
    Error::{
        self,
        AlreadyInUse,
        InitializationFailed
    },
    EspError,
    EzbError,
    IeeeAddr,
    ZclEvent,
    config_views::ConfigAccess,
    utils::PascalString,
    ZigbeeDev,
};

use ezb_node_config::{ChannelMask, CommonFields, Endpoint};
use ezb_node_raw::ezb_zcl_basic_server_attr_t::EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID;

const TASK_NAME: &str = "Zigbee_main";
const TASK_STACK_SIZE: usize = 20 * 1024;   // note: C example uses 4k, Rust may need more

static BEEN_THERE: AtomicBool = AtomicBool::new(false);

/**
* Launch the separate, background FreeRTOS task for interacting with the 'esp_zigbee_lib' callbacks (C side).
*
* @note Initialization etc. is done within the new thread; this is _mainly_ to remain as close to the C examples
*       as possible.
*/
pub(crate) fn zigbee_spawn(cfg: &'static Config, auto_start: bool) -> Result<(),std::io::Error> {

    // We should ever be called just once.
    if BEEN_THERE.swap(true, std::sync::atomic::Ordering::Relaxed) {
        panic!("'zigbee_spawn' twice");
    }

    let ca: ConfigAccess = cfg.into();
    let endpoint_cfg: EndpointCreator = cfg.into();

    let res = std::thread::Builder::new()
        .name(TASK_NAME.to_string())    // visible e.g. in FreeRTOS monitoring
        .stack_size(TASK_STACK_SIZE)
        .spawn(move || {
            log::info!("Zigbee task running");

            // tbd. !! try calling these from outside of the 'spawn' - that way we get reasonable
            //      chance of reacting to errors (here we don't).
            //
            let _dev = ZigbeeDev::new(endpoint_cfg) .unwrap();

            let () = unsafe {
                ezb_zcl_core_action_handler_register(Some(crate::node::zigbee_task::zcl_action_handler));
            };

            ca.init() .unwrap();

            // It seems (based on discussion with google.ai; tbd. CONFIRM FROM THE 'esp_zigbee_lib'
            // SOURCES!!) that having 'install_policy: false' (= allow open pairing: false) WOULD NOT
            // by itself ban attempts to use a default install code.
            //
            // This is weird. In the mean time, let's call this and see what happens.
            //
            // NOTE: 'allow_open_pairing: false' and _not_ having any pre-configured devices feels
            //      like a configuration mismatch. Check whether that is so and catch in 'config'. #tbd.
            //
            #[cfg(feature = "coordinator")]
            {
                // Should we call this? "Enable Trust Center to require install code for joining."
                EzbError::wrap( unsafe {
                    ezb_secur_set_ic_required(true)
                }) .unwrap();
            }


            //ESP_ERROR_CHECK(ezb_app_signal_add_handler(esp_zigbee_app_signal_handler));

            EzbError::wrap(unsafe {
                ezb_app_signal_add_handler(Some(app_signal_handler))
            }) .unwrap();
                // Rust FFI note: 'Option<&fn>' is Rust FFI's way to present a function pointer, heading to a C interface.
                //      Rust has a strict "never-null" rule for function pointers; this jumps around that.

            local_zigbee_run(auto_start) .unwrap();
            loop {}
        });

    Ok(())  // right after the thread is successfully spawned (app can start waiting on the channel)
}

/**
* Run the Zigbee task.
*
* @param auto_start
*   'true' for automatic start of the Zigbee stack
*   'false' for delayed start, needing a call to '.start_top_level_commissioning()' at a later stage.
*/
//ESP_ERROR_CHECK(esp_zigbee_start(false));
//esp_zigbee_launch_mainloop();
//
// tbd. Could merge this whole function to the place where it's called.
fn local_zigbee_run(auto_start: bool) -> Result<!,EspError> {
    EspError::wrap( unsafe {
        esp_zigbee_start(auto_start)
    })?;

    EspError::wrap( unsafe {
        esp_zigbee_launch_mainloop()
    })?;

    unreachable!();     // C code continues with the below; do we ever get here?

    //esp_zigbee_deinit();
    //vTaskDelete(NULL);
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

    let Some(sig) = AppSignal::from(p_type, p_params) else {
        log::error!("Unexpected app signal: {}, {:?}", p_type, p_params);
        return false;
    };

    log::debug!("Received: {}", sig);

    let tx = super::CHANNEL.sender();

    if let Err(err) = tx.try_send(super::Payload::AppSignal(sig)) {
        log::error!("[internal] Failure in channel, likely full ('AppSignal' skipped!!): {:?}", err);
        return false;   // not handled (could also say "handled"; this should not really be allowed to happen)
    }

    // todo: 'true' is from C examples, but we will need a mechanism for the application to declare the logic, when
    //      messages should be marked as handled, when skipped. #revisit
    true
}

// ezb_zcl_core_action_callback_t
extern "C" fn zcl_action_handler(action_id: ezb_zcl_core_action_callback_id_t /*u32*/, msg: *mut ::core::ffi::c_void) {

    // Note: It's possible the '.out' field of 'msg' would need to be written. .. #later

    let Some(action_e) = ezb_zcl_core_action_callback_id_e::parse(action_id) else {
        log::error!("Unexpected ZCL core action (skipped): {}", action_id);
        return;
    };

    let Some(ev) = ZclEvent::parse(action_e, msg) else {
        log::error!("Failed to parse ZCL core action (skipped; bad data?): {}", action_e);
        return;
    };

    let tx = super::CHANNEL.sender();

    // tbd. How are errors processed? Why do we pass 'Ok(ev)', below???

    if let Err(err) = tx.try_send(super::Payload::ZclEvent(Ok(ev))) {
        log::error!("[internal] Failure in channel, likely full ('ZclEvent' skipped!!): {:?}", err);
        return;
    }

    ()
}

/// Helper function (for debugging).
///
/// @return true if we are within an ISR; false if (more safely) in Zigbee task (Thread mode).
#[cfg(false)]
fn debug_is_it_isr() -> bool {
    todo!()

    // 'esp-idf-sys' does not seem to provide such, but if we FFI to C level 'xPortInIsrContext()'
    // that gives the info.
}
