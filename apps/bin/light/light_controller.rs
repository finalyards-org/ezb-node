use alloc::collections::BTreeMap;

use ezb_node::{
    AppSignal,
    BdbStatus,
    Config,
    Platform,
    node::{
        CommissioningModesMask,
        Node
    },
    utils::PascalString,
};

use embassy_time::{Duration};

use crate::{AppError, scheduler::schedule};

const ONE_SEC: Duration = Duration::from_millis(1000);

pub(crate) struct LightController where Self: Node {
    // can have state here
}

impl LightController {
    pub fn new(c: &Config) -> Result<Self,ezb_node::Error> {
        Self::init(c)
    }
}

impl Node for LightController {
    /**
    * Behaviour of this particular node.
    */
    fn on_app_signal(&self, sig: AppSignal) {
        on_app_signal(self, sig)
    }
}

fn on_app_signal(this: &LightController, sig: AppSignal) {
    use AppSignal::*;

    match sig {
        ZdoSignalSkipStartup => {
            log::info!("Initialize Zigbee stack");

            this.start_top_level_commissioning(CommissioningModesMask::empty()); // BDB_MODE_INITIALIZATION
        },

        BdbSignalDeviceFirstStart(BdbStatus::Success) | BdbSignalDeviceReboot(BdbStatus::Success) => {
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
        BdbSignalDeviceFirstStart(st) | BdbSignalDeviceReboot(st) => {
            log::warn!("{} failed with status = {}; retrying...", sig, st);

            schedule( ONE_SEC, |node| {
                node.start_top_level_commissioning(CommissioningModesMask::empty());   // BDB_MODE_INITIALIZATION
            });
        },

        BdbSignalFormation(BdbStatus::Success) => {
            log::info!("Formed network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                this.get_extended_pan_id(), this.get_pan_id(), this.get_current_channel(), this.get_short_address());

            this.start_top_level_commissioning(CommissioningModesMask::NETWORK_STEERING);
        },
        BdbSignalFormation(st) => {
            log::info!("Failed to form network: {} (st = {})", sig, st);
            schedule( ONE_SEC, |node| {
                node.start_top_level_commissioning(CommissioningModesMask::NETWORK_FORMATION);
            });
        },

        BdbSignalSteering(BdbStatus::Success) => {
            log::info!("Network steering completed.")
        },
        BdbSignalSteering(st) => {
            log::info!("Failed the network steering: st = {}", st);
            schedule( ONE_SEC, |node| {
                node.start_top_level_commissioning(CommissioningModesMask::NETWORK_STEERING);
            });
        }

        ZdoSignalDeviceAnnce { short_addr, .. } => {
            log::info!("New device commissioned or rejoined: {}", short_addr);
        },

        NwkSignalPermitJoinStatus{ is_opened_for } => {
            let pan_id = this.get_pan_id();

            match is_opened_for {
                Some(dur) =>
                    log::info!("Network {} is open for {} seconds", pan_id, dur.as_secs()),
                None =>
                    log::info!("Network {} closed, devices joining not allowed.", pan_id)
            };
        },

        ZdoSignalLeaveIndication { short_addr, .. } => {
            log::info!("Node {} is leaving the network.", short_addr);
        },

        _ => {
            log::debug!("ZDO signal: {sig}");
        },
    }
}
