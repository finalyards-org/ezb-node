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

#[path = ".conf.rs"]
mod cfg;

/// Our IAS CIE endpoint (see '[endpoint.1]' in 'app.toml').
const MY_EP: u8 = 1;
    // tbd. consider being able to deduce it from config. #later

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

                // Tell the sensor who its CIE is. Many zone devices send "Zone Enroll Request" only after this.
                //
                // Note: Done for any announcing device; the only one we expect is the door/window sensor.
                //
                log::info!("Writing our IEEE address ({}) as IAS CIE address to {:#06x}, ep {}",
                    self.get_extended_address(), short_addr, SENSOR_EP);

                self.write_ias_cie_address(short_addr, SENSOR_EP, MY_EP)
                    .unwrap_or_else(|e| log::error!("Failed to write IAS CIE address: {}", e));
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

/// Write our IEEE address to the sensor's:
///  - "IAS Zone" (0x0500) cluster
///  - IAS_CIE_ADDRESS attribute (0x0010)
#[cfg(false)]   //R: lukaise läpi, voi poistaa
fn write_ieee_to_remote(this: &Node, short_addr: &str, remote_addr: u128) {

    todo!()

    /*** TRASH??!!
    let mut write_req = esp_zb_zcl_write_attr_cmd_t {
        zcl_basic_cmd: esp_zb_zcl_basic_cmd_t {
            dst_addr_u: esp_zb_zcl_addr_u_t {
                // v2.x vaatii usein short_addr:n asettamisen oikeaan kenttään unionissa
                addr_short: short_addr,
            },
            // Määritetään, että osoite on 16-bittinen short address (Direct unicast)
            dst_addr_mode: esp_zb_zcl_address_mode_t_ESP_ZB_ZCL_ADDR_MODE_16_BIT,
            // Oletetaan, että anturin oletus-endpoint on 1 (yleisin antureilla).
            dst_endpoint: 1,
            src_endpoint: crate::ias_coordinator::MY_IAS_ENDPOINT,
            cluster_id: ESP_ZB_ZCL_CLUSTER_ID_IAS_ZONE as u16, // 0x0500
        },
        attr_number: 1, // Kirjoitetaan 1 attribuutti kerrallaan
        attr_field: [esp_zb_zcl_attribute_field_t {
            attr_id: ESP_ZB_ZCL_ATTR_IAS_ZONE_IAS_CIE_ADDRESS_ID as u16, // 0x0010
            attr_type: esp_zb_zcl_attr_type_t_ESP_ZB_ZCL_ATTR_TYPE_IEEE_ADDR, // 64-bit IEEE tyyppi
            attr_value: &mut attr_val as *mut u64 as *mut std::ffi::c_void,
        }; 1].as_mut_ptr(), // Työnnetään pointteri taulukkoon
    };

    self.esp_zb_zcl_write_attr_cmd_req(&mut write_req);

    log::info!("CIE Address packet dispatched to {:#06x}. Waiting for Zone Enroll Request from the sensor...", short_addr);
    ***/
}
