use embassy_time::{
    Duration
};

use ezb_node::{
    AppSignal,
    BdbMode,
    BdbStatus,
    Node,
    NodeApi,
    ZclEvent,
    //ZclError,
};

#[path = ".conf.rs"]
mod cfg;

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

                // 180 s (3 min) is the Zigbee recommendation for a pairing window.
                const HOW_LONG: Duration = Duration::from_secs(3*60);
                let () = self.open_network( HOW_LONG );

                log::info!("Network is now open for pairing for {} seconds.", HOW_LONG.as_secs());
            },
            BdbSignalSteering(st) => {
                log::info!("Failed the network steering: st = {}", st);

                embassy_time::Timer::after_secs(1).await;
                enter_commission_mode(BdbMode::NetworkSteering)
            }

            ZdoSignalDeviceAnnce { short_addr, device_addr, .. } => {
                log::info!("New device commissioned or rejoined: {}", short_addr);

                let my_ieee = self.get_extended_address();
                log::info!("Sending IAS CIE Address registration to the sensor... {}", my_ieee);

                todo!();    // should we???
                //let x: ! = write_ieee_to_remote(short_addr, my_ieee);
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
                todo!();    // tbd. not sure we need this - don't have any attributes, right?
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
                // Once the door sensor provides "Zone Enroll Request", we'll see it here. #temp
                //
                log::warn!("ZCL Core Action: {:?}", ev);
            }
        }
    }
}

/// Write our IEEE address to the sensor's:
///  - "IAS Zone" (0x0500) cluster
///  - IAS_CIE_ADDRESS attribute (0x0010)
#[cfg(false)]   // may NOT be needed???
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
