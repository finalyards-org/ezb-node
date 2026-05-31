//use alloc::collections::BTreeMap;

use core::time::Duration;

use embassy_sync::channel::{
    DynamicReceiver
};

use ezb_node::{AppSignal, BdbStatus, Config, Node, node::{
    CommissioningModesMask,
}, ConfigAccess};

use crate::{scheduler::schedule};

const ONE_SEC: Duration = Duration::from_millis(1000);

pub(crate) struct LightCoordinator
where Self: Node {
    // can have state here
}

impl LightCoordinator {
    pub fn new(c: &'static Config) -> Result<Self,ezb_node::Error> {
        const AUTO_START: bool = false;
            // Note: This might disappear, see comment of 'Node::init()'.

        let () = <Self as Node>::init(c,AUTO_START)?;

        Self::add_endpoints(c)?;

        Ok(Self {})
    }

    /**
    * Behaviour of this particular node.
    */
    pub async fn run(&self) {
        let rx = self.receiver();

        loop {
            let sig: AppSignal = rx.receive() .await;

            if let Some(x) = on_app_signal(self, sig) {
                log::debug!("No match for: {x}");
            }
        }
    }
}

/**
* Run the received signal through our logic match.
*
* @return 'Some<AppSignal>' if the signal did not find a match (remains to be processed).
*       'None' if the signal has been taken care of
*/
fn on_app_signal(this: &LightCoordinator, sig: AppSignal) -> Option<AppSignal> {
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
                this.get_extended_panid(), this.get_panid(), this.get_current_channel(), this.get_short_address());

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
            let pan_id = this.get_panid();

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
            return Some(sig);    // no match!
        },
    }
    None
}
