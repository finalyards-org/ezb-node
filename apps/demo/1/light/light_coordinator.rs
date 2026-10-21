use ezb_node::{
    AppSignal,
    BdbMode,
    BdbStatus,
    Node,
    NodeApi,    // all 'esp_zigbee_lib' access
    ZclEvent,
};

#[path = ".conf.rs"]
mod cfg;

pub(crate) struct LightCoordinator;

impl LightCoordinator {
    pub fn new() -> Result<Self, ezb_node::Error> {
        let () = <Self as Node>::init(cfg::get())?;
        Ok(Self {})
    }
}

impl Node for LightCoordinator {
    /**
    * Run the received signal through our logic match.
    *
    * @note All access to 'self' is serialized by the 'ezb-node'.
    */
    async fn on_app_signal(&mut self, sig: AppSignal) {
        use AppSignal::*;

        // Used so much it warrants us to make
        let enter_commission_mode = |mode: BdbMode| {
            self.start_top_level_commissioning(mode)
                .expect("Unable to commit the top level commissioning state");  // tbd. does this occur?
        };

        match sig {
            ZdoSignalSkipStartup => {
                log::info!("Initialize Zigbee stack");
                enter_commission_mode(BdbMode::Initialization)
            },

            BdbSignalDeviceFirstStart(BdbStatus::Success) | BdbSignalDeviceReboot(BdbStatus::Success) => {
                // Could do delayed hw initialization, here

                let is_new = self.is_factory_new();
                log::info!("Device started up in {} mode.", if is_new {"factory-reset"} else {"commissioned"});
                if is_new {
                    enter_commission_mode(BdbMode::NetworkFormation);
                } else {
                    log::info!("Device rebooted");
                }
            },
            BdbSignalDeviceFirstStart(st) | BdbSignalDeviceReboot(st) => {
                log::warn!("{} failed with status = {}; retrying...", sig, st);

                embassy_time::Timer::after_secs(1).await;
                enter_commission_mode(BdbMode::Initialization);
            },

            BdbSignalFormation(BdbStatus::Success) => {
                log::info!("Formed network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                    self.get_extended_panid(),
                    self.get_panid(),
                    self.get_current_channel(),
                    self.get_short_address()
                );
                enter_commission_mode(BdbMode::NetworkSteering);
            },
            BdbSignalFormation(st) => {
                log::info!("Failed to form network: {}, st = {}", sig, st);

                embassy_time::Timer::after_secs(1).await;
                enter_commission_mode(BdbMode::NetworkFormation);
            },

            BdbSignalSteering(BdbStatus::Success) => {
                log::info!("Network steering completed.")
            },
            BdbSignalSteering(st) => {
                log::info!("Failed the network steering: st = {}", st);

                embassy_time::Timer::after_secs(1).await;
                enter_commission_mode(BdbMode::NetworkSteering);
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
    fn on_zcl_event(&mut self, ev: ZclEvent) {
        use ZclEvent::*;

        // Obs.!
        //
        //      We might want to know more about failed events. If so, the C side has 'CommonInfo'.
        //      We can pass an 'Either' here (instead of 'Result', that gives wrong vibes).
        //
        match ev {
            SetAttrValue { .. } => {
                todo!();

                //set_attr_value(message);  // i.e. steer the light (color, intensity, on/off)
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
