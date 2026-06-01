use std::{
    boxed::Box,
    sync::OnceLock,
};
use std::collections::BTreeMap;
use bitflags::bitflags;
use esp_idf_svc::{
    sys::EspError
};
use log;

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{
        Channel,
        DynamicReceiver,
    }
};
use embassy_sync::channel::DynamicSender;
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
    ezb_af_create_device_desc,
    ezb_af_device_desc_t,
    EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG,
    ezb_zha_create_color_dimmable_light,
    ezb_af_endpoint_get_cluster_desc,
    ezb_zcl_basic_cluster_desc_add_attr,
    ezb_af_device_add_endpoint_desc,
    ezb_af_device_desc_register,
    ezb_zcl_core_action_handler_register,
    ezb_zcl_cluster_id_e,
    ezb_zcl_cluster_desc_t,
    ezb_zcl_basic_server_attr_t,
    ClusterRole,
    ezb_zcl_core_action_callback_id_e,
};

use crate::{
    AppSignal,
    EndpointCreator,
    IeeeAddr,
    Error::{
        AlreadyInUse,
        InitializationFailed
    },
    config_views::ConfigAccess,
};

use ezb_node_config::{CommonFields, ChannelMask, EndpointConfig};
use ezb_node_raw::ezb_zcl_basic_server_attr_t::EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID;
use crate::utils::PascalString;

const TASK_NAME: &str = "Zigbee_main";
const TASK_STACK_SIZE: usize = 20 * 1024;   // note: C example uses 4k, Rust may need more

// Note: Could also just share from the 'CHANNEL'
static SENDER: OnceLock<&'static DynamicSender<AppSignal>> = OnceLock::new();

/**
* Launch the separate, background FreeRTOS task for interacting with the 'esp_zigbee_lib' callbacks (C side).
*
* @note Initialization etc. is done within the new thread; this is _mainly_ to remain as close to the C examples
*       as possible.
*/
pub(crate) fn zigbee_spawn(cfg: ConfigAccess, auto_start: bool, tx: &'static DynamicSender<AppSignal>) -> Result<(),std::io::Error> {

    // Does double duty in checking we only are called once.
    SENDER.set(tx)
        .unwrap_or_else(|_| panic!("Already initialized"));

    let _ = std::thread::Builder::new()
        .name(TASK_NAME.to_string())    // visible e.g. in FreeRTOS monitoring
        .stack_size(TASK_STACK_SIZE)
        .spawn(move || {
            log::info!("Zigbee task running");

            let (cc, channel_masks) = cfg.expand();
                //
                // 'esp_zigbee_init()' takes a pointer, so we must assume it can read that memory, later.
                // Providing it a 'static', non-changing struct is safe.

            let inner = || -> Result<!,EspError> {
                zigbee_init(cc)?;
                zigbee_setup_commissioning(channel_masks)?;
                zigbee_create_endpoints(&cfg);

                zigbee_run(auto_start)?
            };
            inner().unwrap_or_else(|e| {
                log::error!("Zigbee task failed: {:?}", e);
                loop {}
            })
        })?;

    Ok(())  // right after the thread is successfully spawned (app can start waiting on the channel)
}

/**
* Zigbee init
*/
//ESP_ERROR_CHECK(esp_zigbee_init(&config));
//
fn zigbee_init(cc: &'static esp_zigbee_config_t) -> Result<(),EspError> {

    // esp_err_t esp_zigbee_init(const esp_zigbee_config_t *config);
    //
    EspError::from(unsafe { esp_zigbee_init(cc) })
        .map_or(Ok(()), Err)
}

/**
* Zigbee ...{tbd. complete what this does}...
*/
//ezb_aps_secur_enable_distributed_security(false);
//ESP_ERROR_CHECK(ezb_bdb_set_primary_channel_set(ESP_ZIGBEE_PRIMARY_CHANNEL_MASK));
//ESP_ERROR_CHECK(ezb_bdb_set_secondary_channel_set(ESP_ZIGBEE_SECONDARY_CHANNEL_MASK));
//ESP_ERROR_CHECK(ezb_app_signal_add_handler(esp_zigbee_app_signal_handler));
//return ESP_OK;
//
fn zigbee_setup_commissioning(channel_masks: &[ChannelMask;2]) -> Result<(),EspError> {

    // In C examples, this is set to 'false'.
    //
    //  If set to `true`, the device is allowed to form or join a de-centralized Distributed Security network
    //      (where every Router can authenticate nodes and distribute keys, often used in Touchlink/ZLL commissioning).
    //      If set to `false`, the device will strictly require a Centralized Security network managed by a single
    //      Trust Center (Coordinator).
    //
    // Note: Unknown whether the default is 'false'. Setting because the C examples do.
    //
    let () = unsafe { ezb_aps_secur_enable_distributed_security(false) };

    // Since the configs are validated at compilation, we are not expecting a failure.
    //
    [ezb_bdb_set_primary_channel_set, ezb_bdb_set_secondary_channel_set].into_iter()
        .zip( channel_masks.map(|cm| cm.bits() ))
        .for_each(|(f, mask)| {
            let err = unsafe { f(mask) };
            // EZB_ERR_NONE
            // EZB_ERR_INVALID_ARG  should not happen: 'ChannelMask'

            match EspError::from(err) {
                Some(e) => panic!("Unexpected problem setting channel masks: {}", e),
                None => ()
            }
        });

    EspError::from(unsafe { ezb_app_signal_add_handler(Some(app_signal_handler)) })
        .map_or(Ok(()), Err)?;
        //
        // Rust FFI note: 'Option<&fn>' is Rust FFI's way to present a function pointer, heading to a C interface.
        //      Rust has a strict "never-null" rule for function pointers; this jumps around that.

    Ok(())
}

/**
* Create the device with configured endpoints.
*/
// ezb_af_device_desc_t                  dev_desc  = ezb_af_create_device_desc();
//  ...creating and points and adding them to 'dev_desc'
// ESP_ERROR_CHECK(ezb_af_device_desc_register(dev_desc));
// ezb_zcl_core_action_handler_register(esp_zigbee_zcl_core_action_handler);
//
fn zigbee_create_endpoints(creator: &EndpointCreator) -> Result<(), EspError> {

    // Device where the endpoints will be added to.
    let dev_desc: ezb_af_device_desc_t = unsafe { ezb_af_create_device_desc() };

    creator.create_all(dev_desc);
//rmatch entry {
//r    #[cfg(feature = "ep_color_dimmable_light")]
//r    EndpointConfig::ColorDimmableLightEPC(common_fields) => {
//r        create_color_dimmable_light(dev_desc, ep_id, common_fields);
//r    }
//r    // must be exhaustive match
//r}

    let err = unsafe { ezb_af_device_desc_register(dev_desc) };
    EspError::from(err).map_or(Ok(()), Err)?;

    unsafe {
        ezb_zcl_core_action_handler_register(Some(zcl_core_action_handler));
    }

    Ok(())
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
fn zigbee_run(auto_start: bool) -> Result<!,EspError> {
    let err = unsafe {
        esp_zigbee_start(auto_start)
    };
    EspError::from(err).map_or(Ok(()), Err)?;

    let err = unsafe {
        esp_zigbee_launch_mainloop()
    };
    EspError::from(err).map_or(Ok(()), Err)?;

    unreachable!();     // C code continues with the below; do we ever get here

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

    let sig = AppSignal::from(p_type, p_params)
        .map(|sig| {
            log::info!("Received: {}", sig);

            // todo: Push 'sig' to a channel
        })
        .unwrap_or_else(|| {
            log::error!("Unexpected app signal: {}, {:?}", p_type, p_params);
        });

    SENDER.send(sig);

    true    // handled
}

// ezb_zcl_core_action_callback_t
extern "C" fn zcl_core_action_handler(callback_id: /*ezb_zcl_core_action_callback_id_t*/ u32, msg: *mut ::core::ffi::c_void) {
    todo!()
}

