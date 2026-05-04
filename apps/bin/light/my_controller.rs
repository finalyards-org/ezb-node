/*
*
*/
use alloc::collections::BTreeMap;

use esp_zb::{
    node::{Node, Controller, Router, ChannelMask, NodeConfig},
    AppSignal,
    utils::PascalString,
};
use esp_zb::node::{CommissioningModesMask};

use embassy_time::{Duration};

use crate::{AppError, scheduler::schedule};

const ONE_SEC: Duration = Duration::from_millis(1000);

pub(crate) struct LightController where Self: Controller {
    const MAX_CHILDREN: u8 = 10;
}

impl LightController {
    pub(crate) fn new(ep_id: u8, ep: ColorDimmableLightEPC) -> Result<Self, AppError> {

        <Self as Controller>::init(cfg)?;
        let me = Self{};

        Ok(me)
    }
}

impl Router for LightController {
    type Error = AppError;

    fn on_app_signal(&self, sig: AppSignal) /*? -> Result<(), AppError>*/ {
        on_app_signal(self, sig)
    }
}

impl Controller for LightController {}
impl Node for LightController {}

/**
* Behaviour of this particular node.
*/
fn on_app_signal(this: &LightController, sig: AppSignal) {
    use AppSignal::*;

    match sig {
        ZdoSignalSkipStartup => {
            log::info!("Initialize Zigbee stack");

            this.start_top_level_commissioning(CommissioningModesMask::empty()); // BDB_MODE_INITIALIZATION
        },

        BdbSignalDeviceFirstStart{ success: true } | BdbSignalDeviceReboot{ success: true } => {
            // Could do delayed hw initialization, here

            let is_new = this.is_factory_new();
            log::info!("Device started up in {} mode.", if is_new {"factory-reset"} else {"commissioned"});
            if is_new {
                log::info!("Start network steering");
                this.start_top_level_commissioning(CommissioningModesMask::NETWORK_FORMATION);
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

        BdbSignalFormation{ success: true } => {
            log::info!("Formed network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                this.get_extended_pan_id(), this.get_pan_id(), this.get_current_channel(), this.get_short_address());

            this.start_top_level_commissioning(CommissioningModesMask::NETWORK_STEERING);
        },
        BdbSignalFormation{ success: false } => {
            log::info!("Failed to form network: {}", sig);
            schedule( ONE_SEC, |node| {
                node.start_top_level_commissioning(CommissioningModesMask::NETWORK_FORMATION);
            });
        },

        BdbSignalSteering { success: true } => {
            log::info!("Network steering completed.")
        },
        BdbSignalSteering { success: false } => {
            log::info!("Failed the network steering");
            schedule( ONE_SEC, |node| {
                node.start_top_level_commissioning(CommissioningModesMask::NETWORK_STEERING);
            });
        }

        ZdoSignalDeviceAnnce { device_short_addr } => {
            log::info!("New device commissioned or rejoined: {}", device_short_addr);
        },

        NwkSignalPermitJoinStatus{ isOpenedFor } => {
            let pan_id = this.get_pan_id();

            match isOpenedFor {
                Some(dur) =>
                    log::info!("Network {} is open for {} seconds", pan_id, dur.as_seconds()),
                None =>
                    log::info!("Network {} closed, devices joining not allowed.", pan_id)
            };
        },

        ZdoSignalLeaveIndication { short_addr } => {
            log::info!("Node {} is leaving the network.", short_addr);
        },

        _ => {
            log::debug!("ZDO signal: {sig}");
        },
    }
}
