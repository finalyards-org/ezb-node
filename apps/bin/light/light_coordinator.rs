//use alloc::collections::BTreeMap;

use core::time::Duration;

use embassy_sync::channel::{
    DynamicReceiver
};

use ezb_node::{
    AppSignal,
    BdbMode,
    BdbStatus,
    Config,
    Node,
    ConfigAccess,
    ZclEvent,
};

use crate::{scheduler::schedule};

const ONE_SEC: Duration = Duration::from_millis(1000);

pub(crate) struct LightCoordinator
where Self: Node {
    // can have state here
}

impl LightCoordinator {
    pub fn new(c: &'static Config) -> Result<Self, ezb_node::Error> {
        const AUTO_START: bool = false;
        // Note: This might disappear, see comment of 'Node::init()'.

        let () = <Self as Node>::init(c, c, AUTO_START)?;

        //r Self::add_endpoints(c)?;

        Ok(Self {})
    }

    /**
    * Behaviour of this particular node.
    */
    pub async fn run(&mut self) -> ! {
        Node::run(self,
  |this, sig| this.on_app_signal(sig),
  |this, ev| this.on_zcl_event(ev)
        ).await;
    }

    /**
    * Run the received signal through our logic match.
    */
    // Note: In the prototypes, 'this: &mut Self' to underline that the method is called
    //      indirectly, via the 'Node' mechanism.
    //
    fn on_app_signal(this: &mut Self, sig: AppSignal) {
        use AppSignal::*;

        match sig {
            ZdoSignalSkipStartup => {
                log::info!("Initialize Zigbee stack");
                this.start_top_level_commissioning(BdbMode::Initialization);
            },

            BdbSignalDeviceFirstStart(BdbStatus::Success) | BdbSignalDeviceReboot(BdbStatus::Success) => {
                // Could do delayed hw initialization, here

                let is_new = this.is_factory_new();
                log::info!("Device started up in {} mode.", if is_new {"factory-reset"} else {"commissioned"});
                if is_new {
                    this.start_top_level_commissioning(BdbMode::NetworkFormation);
                } else {
                    log::info!("Device rebooted");
                }
            },
            BdbSignalDeviceFirstStart(st) | BdbSignalDeviceReboot(st) => {
                log::warn!("{} failed with status = {}; retrying...", sig, st);

                schedule(ONE_SEC, |node| {
                    node.start_top_level_commissioning(BdbMode::Initialization);
                });
            },

            BdbSignalFormation(BdbStatus::Success) => {
                log::info!("Formed network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                    this.get_extended_panid(),
                    this.get_panid(),
                    this.get_current_channel(),
                    this.get_short_address()
                );
                this.start_top_level_commissioning(BdbMode::NetworkSteering);
            },
            BdbSignalFormation(st) => {
                log::info!("Failed to form network: {}, st = {}", sig, st);
                schedule(ONE_SEC, |node| {
                    node.start_top_level_commissioning(BdbMode::NetworkFormation);
                });
            },

            BdbSignalSteering(BdbStatus::Success) => {
                log::info!("Network steering completed.")
            },
            BdbSignalSteering(st) => {
                log::info!("Failed the network steering: st = {}", st);
                schedule(ONE_SEC, |node| {
                    node.start_top_level_commissioning(BdbMode::NetworkSteering);
                });
            }

            ZdoSignalDeviceAnnce { short_addr, .. } => {
                log::info!("New device commissioned or rejoined: {}", short_addr);
            },

            NwkSignalPermitJoinStatus { is_opened_for } => {
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
                log::info!("Zigbee APP signal: {sig}");
            },
        }
    }

    /**
    * Run the received ZCL event through our logic.
    */
    fn on_zcl_event(this: &mut Self, ev: ZclEvent) {
        use ZclEvent::*;

        match ev {
            SetAttrValue { .. } => {
                unimplemented!()
                //set_attr_value(message);  // i.e. steer the light (color, intensity, on/off)
                log::debug!("Setting light to: {}", "..something..");   // TEMP
            },
            DefaultResp { in_status_code, .. } => {
                //ezb_zcl_cmd_default_rsp_message_t *default_rsp = (ezb_zcl_cmd_default_rsp_message_t *)message;
                log::info!("Received ZCL Default Response, status_code: {}", in_status_code);
            },
            _ => {
                log::warn!("ZCL Core Action: {?:}", ev);
            }
        }
    }
}
