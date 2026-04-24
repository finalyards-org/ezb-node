/*
*
*/
use std::time::Duration;
use esp_zb::{
    node::{Node, Router},
    Signal
};
use esp_zb::node::CommissioningMode;
use crate::AppError;

pub(crate) struct LightRouter where Self: Router {

}

impl LightRouter {
    pub(crate) fn new() -> Result<Self, AppError> {
        Router::init(10);
        Ok(Self {})
    }
}

impl Router for LightRouter {
    type Error = AppError;

    fn on_app_signal(&self, sig: Signal) /*? -> Result<(), AppError>*/ {
        on_app_signal(self, sig)
    }
}

impl Node for LightRouter {}

const ONE_SEC: Duration = Duration::from_millis(1000);

/**
*/
fn on_app_signal(rtr: &LightRouter, sig: Signal) /*? -> Result<(), AppError>*/ {
    use Signal::*;

    match sig {
        ZdoSignalSkipStartup => {
            log::info!("Initialize Zigbee stack");

            rtr.start_commissioning(CommissioningMode::empty()); // BDB_MODE_INITIALIZATION
        },

        BdbSignalDeviceFirstStart{ success: true } | BdbSignalDeviceReboot{ success: true } => {
            // Could do delayed hw initialization, here

            // tbd. what does the "is factory new" actually mean?
            let x = rtr.bdb_is_factory_new();
            log::info!("Device started up in {}factory-reset mode",  if x {""} else {"non"});
            if x {
                log::info!("Start network steering");
                rtr.bdb_start_top_level_commissioning(CommissioningMode::NETWORK_STEERING);
            } else {
                log::info!("Device rebooted");
            }
        },
        BdbSignalDeviceFirstStart{ success: false } | BdbSignalDeviceReboot{ success: false } => {
            log::warn!("{} failed, retrying", sig);

            rtr.schedule( ONE_SEC, |node| {
                node.bdb_start_top_level_commissioning(0);   // BDB_MODE_INITIALIZATION
            });
        },

        BdbSignalSteering{ success: true } => {
            log::info!("Joined network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: 0x{:04x}",
                rtr.get_extended_pan_id(), rtr.get_pan_id(), rtr.get_current_channel(), rtr.get_short_address());
        },
        BdbSignalSteering{ success: false } => {
            log::info!("Network steering was not successful: {}", sig);
            rtr.schedule( ONE_SEC, |node| {
                node.bdb_start_top_level_commissioning_cb(CommissioningMode::NETWORK_STEERING);
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
