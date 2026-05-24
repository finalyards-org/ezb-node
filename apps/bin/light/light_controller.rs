//use alloc::collections::BTreeMap;

use core::time::Duration;

use ezb_node::{
    AppSignal,
    BdbStatus,
    Config,
    Node,
    //PlatformDeviceView,
    node::{
        CommissioningModesMask,
    },
    //r utils::PascalString,
};

//r use embassy_time::{Duration};

use crate::{scheduler::schedule};

const ONE_SEC: Duration = Duration::from_millis(1000);

const CONFIG: &Config = include!(concat!(env!("OUT_DIR"), "/light_conf.in"));

pub(crate) struct LightController where Self: Node {
    // can have state here
}

impl LightController {
    pub fn new() -> Result<Self,ezb_node::Error> {
        let c = CONFIG;
        Self::init(c)?;
        Self::add_endpoints(c)?;

        Ok(Self {})
    }

    #[cfg(false)] //R; in API
    /**
    * Launch the task that receives Zigbee events, and pumps them to our '.on_app_signal()'.
    */
    //* @note Here (and in not 'Node'), because needs 'std::thread'. TEMP
    //
    // 'auto_start': we might get rid of this parameter. It has to do with the application initialization logic.
    //      C example uses delayed hardware init. If the value is 'true', the Zigbee network needs to be later
    //      activated by a call to '...'.
    //
    //      We could do a different kind of arrangement, in Rust (while retaining the freedom)... #tbd
    //
    fn spawn(&self, auto_start: bool) -> Result<(), std::io::Error> {
        use std::thread;

        let _ = thread::Builder::new()
            .name(TASK_NAME.to_string())    // visible e.g. in FreeRTOS monitoring
            .stack_size(TASK_STACK_SIZE)    // Rust: 20000
            .spawn(move || {
                log::info!("Zigbee task running");

                self.run(auto_start).unwrap_or_else(|e| {
                    log::error!("Zigbee task failed: {:?}", e);
                });
            })?;

        // Right after the thread is successfully spawned.
        Ok(())
    }
}

impl Node for LightController {
    /**
    * Behaviour of this particular node.
    *
    * @note Gets called in the application RTOS thread.
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
            log::debug!("ZDO signal: {sig}");
        },
    }
}
