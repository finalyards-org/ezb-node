/*
*
*/
use esp_zb::{node::{Node, Router}, IsOpenedForSecs, ManufacturerInfo, Signal};
use esp_zb::node::CommissioningModesMask;

use embassy_time::{Duration};

use crate::{
    AppError,
    scheduler::schedule
};

const ONE_SEC: Duration = Duration::from_millis(1000);

pub(crate) struct LightRouter where Self: Router {

}

//? const COLOR_DIMMABLE_LIGHT_ENDPOINT: u8 = 10;

impl LightRouter {
    pub(crate) fn new() -> Result<Self, AppError> {
        <Self as Router>::init(10)?;

        let cfg: esp_zb_color_dimmable_light_cfg_t = esp_zb_color_dimmable_light_cfg_t::default();
        let ep: = esp_zb_color_dimmable_light_ep_create(, &cfg);

        let ep = endpoint::ColorDimmableLight::new(COLOR_DIMMABLE_LIGHT_ENDPOINT);

        let me = Self{};
        me.add_ep_basic_manufacturer_info(ep, HA_COLOR_DIMMABLE_LIGHT_ENDPOINT, &info);
        me.device_register(ep);
        //me.action_handler_register(zb_action_handler);
        //me.set_primary_network_channel_set(...);

        Ok(me)
    }
}

impl Router for LightRouter {
    type Error = AppError;

    fn on_app_signal(&self, sig: Signal) /*? -> Result<(), AppError>*/ {
        on_app_signal(self, sig)
    }
}

impl Node for LightRouter {}

/**
*/
fn on_app_signal(rtr: &LightRouter, sig: Signal) /*? -> Result<(), AppError>*/ {
    use Signal::*;

    match sig {
        ZdoSignalSkipStartup => {
            log::info!("Initialize Zigbee stack");

            rtr.start_top_level_commissioning(CommissioningModesMask::empty()); // BDB_MODE_INITIALIZATION
        },

        BdbSignalDeviceFirstStart{ success: true } | BdbSignalDeviceReboot{ success: true } => {
            // Could do delayed hw initialization, here

            let is_virgin = rtr.is_factory_new();
            log::info!("Device started up in {} mode.", if is_virgin {"factory-reset"} else {"commissioned"});
            if is_virgin {
                log::info!("Start network steering");
                rtr.start_top_level_commissioning(CommissioningModesMask::NETWORK_STEERING);
            } else {
                log::info!("Device rebooted");
            }
        },
        BdbSignalDeviceFirstStart{ success: false } | BdbSignalDeviceReboot{ success: false } => {
            log::warn!("{} failed, retrying", sig);

            schedule( ONE_SEC, |node| {
                node.start_top_level_commissioning(CommissioningModesMask::empty());   // BDB_MODE_INITIALIZATION
            });
        },

        BdbSignalSteering{ success: true } => {
            log::info!("Joined network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                rtr.get_extended_pan_id(), rtr.get_pan_id(), rtr.get_current_channel(), rtr.get_short_address());
        },
        BdbSignalSteering{ success: false } => {
            log::info!("Network steering was not successful: {}", sig);
            schedule( ONE_SEC, |node| {
                node.start_top_level_commissioning(CommissioningModesMask::NETWORK_STEERING);
            });
        }

        NwkSignalPermitJoinStatus{ isOpened } => {
            let pan_id = rtr.get_pan_id();

            match isOpened {
                Some(IsOpenedForSecs(secs)) =>
                    log::info!("Network{} is open for {} seconds", pan_id, secs),
                None =>
                    log::info!("Network{} closed, devices joining not allowed.", pan_id)
            };
        },
        _ => {
            log::debug!("ZDO signal: {sig}");
        },
    }
    //Ok(())
}
