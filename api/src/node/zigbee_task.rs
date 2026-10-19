//use bitflags::bitflags;
use log;
use std::{
    boxed::Box,
    collections::BTreeMap,
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
    esp_zigbee_launch_mainloop,
    esp_zigbee_start,
    ezb_app_signal_add_handler,
    ezb_app_signal_get_params,
    ezb_app_signal_get_type,
    ezb_app_signal_t,
    ezb_secur_set_ic_required,
    ezb_zcl_core_action_callback_id_e,
    ezb_zcl_core_action_callback_id_t,
    ezb_zcl_core_action_handler_register,
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

// 'true' for now, to keep similarity with C examples.
const AUTO_START: bool = true;

/**
* Separate, background FreeRTOS task for interacting with the 'esp_zigbee_lib' callbacks (C side).
*
* @note Corresponds closely with C samples' 'esp_zigbee_stack_main_task'.
*/
// tbd. !! try doing initialization outside of the Zigbee task (if that works, it allows better
//      error management than here). e.g. a 'zigbee_spawn()' that runs within the application task.
//
pub(crate) fn zigbee_task(cfg: &'static Config) -> ! {
    let ca: ConfigAccess = cfg.into();

    //    esp_zigbee_config_t config = ESP_ZIGBEE_DEFAULT_CONFIG();
    // ^-- Creating '&'static Config' corresponds to this

    log::info!("Zigbee task launched");
    log::trace!("TESTING");
    panic!("12");

    //    ESP_ERROR_CHECK(esp_zigbee_init(&config));
    //
    // It's important this is the first call - 'esp_zigbee_init()' before any other calls.
    //
    log::info!("TRACE: ennen 'ca.init()'");
    ca.init() .unwrap();
    log::info!("TRACE: jälkeen 'ca.init()'");

    // esp_zigbee_setup_commissioning():
    //    ezb_aps_secur_enable_distributed_security(false);
    //     ESP_ERROR_CHECK(ezb_bdb_set_primary_channel_set(ESP_ZIGBEE_PRIMARY_CHANNEL_MASK));
    //     ESP_ERROR_CHECK(ezb_bdb_set_secondary_channel_set(ESP_ZIGBEE_SECONDARY_CHANNEL_MASK));
    //     ESP_ERROR_CHECK(ezb_app_signal_add_handler(esp_zigbee_app_signal_handler));

    let endpoint_cfg: EndpointCreator = cfg.into();

    //      ESP_ERROR_CHECK(esp_zigbee_create_zha_color_dimmable_light_device());
    //
    let _dev = ZigbeeDev::new(endpoint_cfg) .unwrap();

    log::info!("TRACE: ezb_zcl_core_action_handler_register");
    let () = unsafe {
        ezb_zcl_core_action_handler_register(Some(crate::node::zigbee_task::zcl_action_handler));
    };

    // tbd. CONFIRM THIS:
    //
    // It seems (based on discussion with google.ai) that having 'install_policy: false'
    // (= allow open pairing: false) WOULD NOT by itself ban attempts to use a default install code.
    //
    // This is weird. In the mean time, let's call this and see what happens.
    //
    // THE C SAMPLES ARE OF NO USE since they have the loose security (allowing open keys).
    #[cfg(feature = "coordinator")]
    {
        log::info!("TRACE: ezb_secur_set_ic_required (true)");
        // Should we call this? "Enable Trust Center to require install code for joining."
        EzbError::wrap( unsafe {
            ezb_secur_set_ic_required(true)
        }) .unwrap();
    }

    //ESP_ERROR_CHECK(ezb_app_signal_add_handler(esp_zigbee_app_signal_handler));

    log::info!("TRACE: ezb_app_signal_add_handler");
    EzbError::wrap(unsafe {
        ezb_app_signal_add_handler(Some(app_signal_handler))
    }) .unwrap();
        // Rust FFI note: 'Option<&fn>' is Rust FFI's way to present a function pointer, heading to a C interface.
        //      Rust has a strict "never-null" rule for function pointers; this jumps around that.

    //    ESP_ERROR_CHECK(esp_zigbee_start(false));
    {
        EspError::wrap( unsafe {
            esp_zigbee_start(AUTO_START)
        }) .unwrap();

        log::trace!("esp_zigbee_launch_mainloop");
        EspError::wrap( unsafe {
            esp_zigbee_launch_mainloop()
        }) .unwrap();
    }
    unreachable!()
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

    if let Err(err) = tx.try_send(super::Payload::ZclEvent(ev)) {
        log::error!("[internal] Failure in channel; 'ZclEvent' skipped: {:?}", err);
        return;
    }
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
