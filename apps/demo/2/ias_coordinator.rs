use ezb_node::{
    AppSignal,
    BdbMode,
    BdbStatus,
    IasZoneEnrollResponse,
    JoinWindowSecs,
    Node,
    NodeApi,
    NodeApiIasCie,
    ZclEvent,
};
use ezb_node::config::DeviceType;

#[path = ".conf.rs"]
mod cfg;

/// Endpoint of the door/window sensor's IAS Zone (server) cluster.
///
/// Assumed; endpoint 1 is the most common for single-function sensors. Not verified for Wiser CCT591011. tbd.
const SENSOR_EP: u8 = 1;

/// Zone ID we give to the (only) enrolled sensor.
const ZONE_ID: u8 = 0;

pub(crate) struct IasCoordinator;

impl IasCoordinator {
    pub fn new() -> Result<Self, ezb_node::Error> {
        let () = <Self as Node>::init(cfg::get())?;
        Ok(Self {})
    }
}

impl Node for IasCoordinator {
    /**
    * Run the received signal through our logic match.
    */
    async fn on_app_signal(&mut self, sig: AppSignal) {
        use AppSignal::*;

        // Note: Changes to 'config' can make this be 'const fn'
        let my_ias_cie_ep = cfg::get().my_ep_for(DeviceType::IasCie)
            .expect("No 'IasCie' endpoint in the TOML");

        let enter_commission_mode = |mode: BdbMode| {
            self.start_top_level_commissioning(mode)
                .expect("Unable to commit the top level commissioning state");  // tbd. does this occur?
        };

        // If there is an error, all we really can is log it, and keep going.
        //
        match sig {
            ZdoSignalSkipStartup => {
                log::info!("Initialize Zigbee stack");
                enter_commission_mode(BdbMode::Initialization)
            },

            BdbSignalDeviceFirstStart(BdbStatus::Success) | BdbSignalDeviceReboot(BdbStatus::Success) => {
                let is_new = self.is_factory_new();
                log::info!("Device started up in {} mode.", if is_new {"factory-reset"} else {"commissioned"});
                if is_new {
                    enter_commission_mode(BdbMode::NetworkFormation)
                } else {
                    log::info!("Device reboot");

                    /*** ? where did this come from?
                    // Open for pairing for a moment.
                    self.start_top_level_commissioning(BdbMode::NetworkSteering)
                        .expect();
                        //r .unwrap_or_else(|e| log::warn!("Failed handling '{}' (NOT new): {}", sig, e));
                    ***/
                }
            },
            BdbSignalDeviceFirstStart(st) | BdbSignalDeviceReboot(st) => {
                log::warn!("{} failed with status = {}; retrying...", sig, st);

                embassy_time::Timer::after_secs(1).await;
                enter_commission_mode(BdbMode::Initialization)
            },

            BdbSignalFormation(BdbStatus::Success) => {
                log::info!("Formed network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                    self.get_extended_panid(),
                    self.get_panid(),
                    self.get_current_channel(),
                    self.get_short_address()
                );
                enter_commission_mode(BdbMode::NetworkSteering)
            },
            BdbSignalFormation(st) => {
                log::info!("Failed to form network: {}, st = {}", sig, st);

                embassy_time::Timer::after_secs(1).await;
                enter_commission_mode(BdbMode::NetworkFormation)
            },

            BdbSignalSteering(BdbStatus::Success) => {
                log::info!("Network steering completed.");

                // For the sensor to join, the coordinator must send "Permit Join" to the network.
                //
                // Note: Since we have the device pre-configured, no need for a long join window.
                //
                let () = self.open_network(JoinWindowSecs::_1MIN);

                log::info!("Network is now open for pairing for {} minute(s).", 1);
            },
            BdbSignalSteering(st) => {
                log::info!("Failed the network steering: st = {}", st);

                embassy_time::Timer::after_secs(1).await;
                enter_commission_mode(BdbMode::NetworkSteering)
            }

            ZdoSignalDeviceAnnce { short_addr, device_addr, .. } => {
                log::info!("New device commissioned or rejoined: {:#06x} ({})", short_addr, device_addr);
            },

            // Authorization = the joining device has completed the Trust Center Link Key exchange (Zigbee 3.0).
            //
            // Talk to the device (ZCL) only after this. Starting the IAS CIE address write already at
            // Device Announce led - repeatedly - to the key exchange timing out (status 1) and the sensor leaving
            // the network. A sleepy end device only polls its parent now and then, so our ZCL traffic likely
            // competed with the key exchange. (speculation)
            //
            // Note: A rejoining (already authorized) device may not cause this signal. That's fine: it keeps
            //      the CIE address and enrollment from the earlier join. (unverified)
            //
            ZdoSignalDeviceAuthorized { short_addr, device_addr, status_X: 0 /*success*/, .. } => {
                log::info!("Device authorized: {:#06x} ({})", short_addr, device_addr);

                // Tell the sensor who its CIE is. Many zone devices send "Zone Enroll Request" only after this.
                //
                // Note: Done for any authorized device; the only one we expect is the door/window sensor.
                //
                log::info!("Writing our IEEE address ({}) as IAS CIE address to {:#06x}, ep {}",
                    self.get_extended_address(), short_addr, SENSOR_EP);

                self.write_ias_cie_address(short_addr, SENSOR_EP, my_ias_cie_ep)
                    .unwrap_or_else(|e| log::error!("Failed to write IAS CIE address: {}", e));
            },
            ZdoSignalDeviceAuthorized { short_addr, device_addr, status_X, .. } => {
                // 1 = timeout, 2 = failed ('ezb_zdo_auth_status_e'). The device will leave the network.
                log::warn!("Device {:#06x} ({}) failed authorization: status {}", short_addr, device_addr, status_X);
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
                // Not expected: we don't have application attributes. tbd. remove, once confirmed in practice.
                log::warn!("Unexpected: {:?}", ev);
            },

            WriteAttrResp { .. } => {
                // Expected as a response to 'write_ias_cie_address'.
                log::info!("Write Attributes response: {:?}", ev);
            },

            IasZoneEnrollReq { ref header, zone_type, manuf_code, .. } => {
                log::info!("Zone Enroll Request: zone type {:#06x}, manufacturer {:#06x}; accepting as zone {}",
                    zone_type, manuf_code, ZONE_ID);

                self.ias_zone_enroll_response(header, ZONE_ID, IasZoneEnrollResponse::Success)
                    .unwrap_or_else(|e| log::error!("Failed to send Zone Enroll Response: {}", e));
            },

            IasZoneStatusChangeNotif { zone_status, zone_id, .. } => {
                // Bits (ZCL spec): 0 = alarm1 (e.g. open), 2 = tamper, 3 = battery low
                let open = zone_status & 0x0001 != 0;
                let tamper = zone_status & 0x0004 != 0;
                let battery_low = zone_status & 0x0008 != 0;

                log::info!("Zone {}: {}{}{} (zone status {:#06x})", zone_id,
                    if open {"OPEN"} else {"closed"},
                    if tamper {", TAMPER"} else {""},
                    if battery_low {", battery low"} else {""},
                    zone_status
                );
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
                log::warn!("ZCL Core Action (not handled): {:?}", ev);
            }
        }
    }
}
