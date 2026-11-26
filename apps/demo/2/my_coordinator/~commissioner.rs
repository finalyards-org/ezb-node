use ezb_node::{AppSignal, AuthStatus, BdbMode, BdbStatus, JoinWindowSecs};
use ezb_node::AppSignal::{BdbSignalDeviceFirstStart, BdbSignalDeviceReboot, BdbSignalFormation, BdbSignalSteering, NwkSignalPermitJoinStatus, ZdoSignalDeviceAnnce, ZdoSignalDeviceAuthorized, ZdoSignalLeaveIndication, ZdoSignalSkipStartup};

pub(crate) struct MyCommissioner;

impl Commissioner for MyCommissioner {
    /**
    * Handle the Zigbee device commissioning.
    *
    * @note Having this 'async' helps with handling timings (state lives within the async block).
    */
    async fn on_app_signal(&mut self, sig: AppSignal) {
        use AppSignal::*;

        let enter_commission_mode = |mode: BdbMode| {
            self.start_top_level_commissioning(mode)
                .expect("Unable to commit the top level commissioning state");  // tbd. does this occur?
        };

        // Note: If there is an error, all we really can is log it, and keep going.

        match sig {
            ZdoSignalSkipStartup => {
                log::info!("Initialize Zigbee stack");
                enter_commission_mode(BdbMode::Initialization)
            },

            // tbd. Differentiate with the kind of message, instead of '.is_factory_new()'.
            //
            BdbSignalDeviceFirstStart(BdbStatus::Success) | BdbSignalDeviceReboot(BdbStatus::Success) => {
                let is_new = self.is_factory_new();
                log::info!("Device started up in {} mode.", if is_new {"factory-reset"} else {"commissioned"});
                if is_new {
                    enter_commission_mode(BdbMode::NetworkFormation)
                } else {
                    // Open for pairing for a moment, to collect devices that used to be in the network.
                    //  ^-- @claude: Is that a correct description?
                    //
                    self.start_top_level_commissioning(BdbMode::NetworkSteering)
                        .unwrap_or_else(|e| log::warn!("Failed to start commissioning after reboot: {}", e));
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

            // tbd. how do we provide information to the app about this?
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
            //      the CIE address and enrollment from the earlier join.
            //
            ZdoSignalDeviceAuthorized { short_addr, device_addr, status: AuthStatus::Success, .. } => {
                log::info!("Device authorized: {:#06x} ({})", short_addr, device_addr);
                self.sensor = Some((short_addr, device_addr));

                // Tell the sensor who its CIE is. Many zone devices send "Zone Enroll Request" only after this.
                //
                // Note: Done for any authorized device; the only one we expect is the door/window sensor.
                //
                log::info!("Writing our IEEE address ({}) as IAS CIE address to {:#06x}, ep {}",
                    self.get_extended_address(), short_addr, SENSOR_EP);

                self.write_ias_cie_address(short_addr, SENSOR_EP, my_ias_cie_ep)
                    .unwrap_or_else(|e| log::error!("Failed to write IAS CIE address: {}", e));
            },
            ZdoSignalDeviceAuthorized { short_addr, device_addr, status, .. } => {
                // 'Timeout' / 'Failed'. The device will leave the network.
                log::warn!("Device {:#06x} ({}) failed authorization: {}", short_addr, device_addr, status);
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
}
