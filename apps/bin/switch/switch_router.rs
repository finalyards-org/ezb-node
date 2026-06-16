use embassy_sync::channel::{
    DynamicReceiver
};
use embassy_time::{
    Duration
};

use ezb_node::{
    AppSignal,
    BdbMode,
    BdbStatus,
    Config,
    Node,
    ZclEvent,
    ZclError,
};

//? use crate::{scheduler::schedule};

const ONE_SEC: Duration = Duration::from_millis(1000);

pub(crate) struct LightSwitchRouter
where Self: Node {
    // can have state here
}

impl Node for LightSwitchRouter {}

impl LightSwitchRouter {
    pub fn new(c: &'static Config) -> Result<Self, ezb_node::Error> {
        const AUTO_START: bool = false;
        // Note: This might disappear, see comment of 'Node::init()'.

        let () = <Self as Node>::init(c, AUTO_START)?;
        Ok(Self {})
    }

    /**
    * Behaviour of this particular node.
    */
    pub async fn run(mut self) -> ! {
        <Self as Node>::run(self,
  |this, sig| this.on_app_signal(sig),
  |this, ev| this.on_zcl_event(ev)
        ).await
    }

    /**
    * Run the received signal through our logic match.
    */
    fn on_app_signal(&mut self, sig: AppSignal) {
        use AppSignal::*;

        match sig {
            ZdoSignalSkipStartup => {
                log::info!("Initialize Zigbee stack");
                self.start_top_level_commissioning(BdbMode::Initialization);
            },

            BdbSignalDeviceFirstStart(BdbStatus::Success) | BdbSignalDeviceReboot(BdbStatus::Success) => {
                // Could do delayed hw initialization, here

                let is_new = self.is_factory_new();
                log::info!("Device started up in {} mode.", if is_new {"factory-reset"} else {"commissioned"});
                if is_new {
                    self.start_top_level_commissioning(BdbMode::NetworkFormation);
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
                    self.get_extended_panid(),
                    self.get_panid(),
                    self.get_current_channel(),
                    self.get_short_address()
                );
                self.start_top_level_commissioning(BdbMode::NetworkSteering);
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
                let pan_id = self.get_panid();
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
    fn on_zcl_event(&mut self, ev_res: Result<ZclEvent, ZclError>) {
        use ZclEvent::*;

        // Note: It may be that we need to know more about the event, when errors arise. Let's, however, keep the
        //      'ZclEvent' for successful ones, and curry the 'ZclError' with extra information ('CommonInfo'?) if
        //      there is a need. All this information is available in the C level, but dividing it to success/fail
        //      will make applications easier to read.

        let Ok(ev) = ev_res else {
            log::warn!("ZCL event error: {}", ev_res.err().unwrap());
            return;
        };

        match ev {
            SetAttrValue { .. } => {
                unimplemented!();

                //set_attr_value(message);  // i.e. steer the light (color, intensity, on/off)
                log::debug!("Setting light to: {}", "..something..");   // TEMP
            },
            DefaultResp { err, .. } => {
                //ezb_zcl_cmd_default_rsp_message_t *default_rsp = (ezb_zcl_cmd_default_rsp_message_t *)message;
                log::info!("Received ZCL Default Response, status: {}",
                    match err {
                        Some(e) => e.to_string(),
                        None => "success".into(),
                    }
                );
            },
            _ => {
                log::warn!("ZCL Core Action: {:?}", ev);
            }
        }
    }
}
