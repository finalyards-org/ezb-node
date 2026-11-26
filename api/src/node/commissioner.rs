#![cfg(feature = "coordinator")]

use embassy_time::Duration;

use ezb_node_raw::{
    ezb_bdb_open_network,
    ezb_bdb_start_top_level_commissioning,
    ezb_nwk_get_current_channel,
    ezb_nwk_get_extended_address,
    ezb_nwk_get_extended_panid,
    ezb_nwk_get_panid,
    ezb_nwk_get_short_address
};

use crate::{
    AddrMode,
    AppSignal,
    AuthStatus,
    BdbMode,
    BdbStatus,
    ClusterId,
    ReportConfig,
    Config,
    Error,
    EzbError,
    IeeeAddr,
    ZclEvent
};

use super::{Node, ZigbeeGuard};

/// Allows the application to be informed of what happens in the Zigbee controller's commissioning.
///
pub trait Commissioner: Node {
    /// A way for the application to follow what's happening.
    async fn on_commissioning(&mut self, ev: CommissioningEvent);
}

// Implement whenever 'Commissioner' is implemented - but remains hidden from out-of-crate (application).
impl<T: Commissioner> CommissionerInternal for T {}

/// Implementation details
trait CommissionerInternal: Commissioner {

    /**
    * @brief  Start top level commissioning procedure with specified mode mask.
    *
    * @note This function is intended to be a lower level, common tool. Applications should likely use
    *       node type -specific helper methods (and not directly the modes).
    *
    * Note: Some of the modes apply only to certain node types (NETWORK_FORMATION only to Coordinator
    *       role).
    *
    * Note: Modes are bit patterns.
    *
    * @note It's a little bit strange that the C library only lists 'EZB_ERR_NONE' as a return value.
    *       Does it always succeed? Is this a claim that future version might give errors? #feedback
    */
    fn start_top_level_commissioning(&self, mask: BdbMode) -> Result<(),EzbError> {
        let _guard = ZigbeeGuard::acquire();

        EzbError::wrap(unsafe {
            ezb_bdb_start_top_level_commissioning(mask.into())
        })
    }

    /**
    * Open the network for joining.
    *
    * @param for_how_long After this time, the permit expires. Range is 1..254 seconds.
    *       Typical use in examples involving manual feeding of install codes is 180s.
    *
    * @note Since the call is synchronous (it only starts the join), error situations are not
    *       well understood: the C side can err on "not memory" (panic might be justified) and
    *       "on failed" (what is that, really??) - which is vague (again, panic).
    */
    fn open_network(&self, secs: JoinWindowSecs) -> () {

        let _guard = ZigbeeGuard::acquire();
        EzbError::wrap( unsafe {
            ezb_bdb_open_network(secs.0)
            // docs don't care to mention the unit (2.1.0), but examples use seconds.

        }).unwrap_or_else(|e| {
            panic!("'ezb_bdb_open_network' failed with: {}", e);
        });
    }
}

/**
* Handle the Zigbee device commissioning.
*
* Called by: 'Node::run_loop'
*
* @note Having this 'async' helps with handling timings (state lives within the async block).
*/
pub(crate) async fn handle_app_signal<C: Commissioner>(&mut this: C, sig: AppSignal) {
    use AppSignal::*;

    let enter_commission_mode = |mode: BdbMode| {
        this.start_top_level_commissioning(mode)
            .expect("Unable to commit the top level commissioning state");  // tbd. does this occur?
    };

    let enter_commission_mode_after = async |dur: Duration, mode: BdbMode| {
        embassy_time::Timer::after(dur) .await;
        enter_commission_mode(mode)
    };

    // Note: Let's make this 'async' only if there is application need.
    let inform = |ev: CommissioningEvent| {
        this.on_commissioning_event(ev)
    };

    // Note: If there is an error, all we really can is log it, and keep going.

    match sig {
        ZdoSignalSkipStartup => {
            log::info!("Initialize Zigbee stack");
            enter_commission_mode(BdbMode::Initialization)
        },

        // tbd. Differentiate with the kind of message, instead of '.is_factory_new()'.
        //
        /***r
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
        },***/
        BdbSignalDeviceFirstStart(BdbStatus::Success) => {
            log::info!("Device started up: factory-reset");
            enter_commission_mode(BdbMode::NetworkFormation)
        },
        BdbSignalDeviceReboot(BdbStatus::Success) => {
            log::info!("Device started up: already commissioned");

            // Open for pairing for a moment, to collect devices that used to be in the network.
            //  ^-- @claude: Is that a correct description?
            //
            this.start_top_level_commissioning(BdbMode::NetworkSteering)
                .unwrap_or_else(|e| log::warn!("Failed to start commissioning after reboot: {}", e));
        },
        BdbSignalDeviceFirstStart(st) | BdbSignalDeviceReboot(st) => {
            log::warn!("{} failed with status = {}; retrying...", sig, st);

            enter_commission_mode_after(1.secs(), BdbMode::Initialization) .await;
        },

        BdbSignalFormation(BdbStatus::Success) => {
            log::info!("Formed network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                this.get_extended_panid(),
                this.get_panid(),
                this.get_current_channel(),
                this.get_short_address()
            );
            enter_commission_mode(BdbMode::NetworkSteering)
        },
        BdbSignalFormation(st) => {
            log::info!("Failed to form network: {}, st = {}", sig, st);

            enter_commission_mode_after(1.secs(), BdbMode::NetworkFormation) .await;
        },

        BdbSignalSteering(BdbStatus::Success) => {
            log::info!("Network steering completed.");

            // For the sensor to join, the coordinator must send "Permit Join" to the network.
            //
            // Note: Since we have the device pre-configured, no need for a long join window.
            //
            let () = this.open_network(JoinWindowSecs::_1MIN);

            log::info!("Network is now open for pairing for {} minute(s).", 1);
        },
        BdbSignalSteering(st) => {
            log::info!("Failed the network steering: st = {}", st);

            enter_commission_mode_after(1.secs(), BdbMode::NetworkSteering) .await;
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

            inform( CommissioningEvent::DeviceAuthorized { short_addr, device_addr } );
        },
        ZdoSignalDeviceAuthorized { short_addr, device_addr, status, .. } => {
            // 'Timeout' / 'Failed'. The device will leave the network.
            log::warn!("Device {:#06x} ({}) failed authorization: {}", short_addr, device_addr, status);

            inform( CommissioningEvent::DeviceAuthorizationFailed { short_addr, device_addr, auth_status: status } );
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

/// Application level events about commissioning.
///
/// Can be used e.g. to show UI changes (e.g. change the color of a LED).
///
pub enum CommissioningEvent {
    DeviceAuthorized{ short_addr: u16, device_addr: IeeeAddr },
    DeviceAuthorizationFailed{ short_addr: u16, device_addr: IeeeAddr, auth_status: AuthStatus },

    // tbd. Add events, as there is application demand.
    //...
}

/// Traditionally, a join window of 3 min is common and suggested (e.g. 'esp-zigbee-sdk' all examples
/// use it). However, this must be partly due to human delay in typing in codes, via e.g. an app.
///
/// If you provide pre-configured devices (see e.g. 'apps/demo/2/app.toml'), a window of 10s or 1min
/// should be quite enough.
///
#[derive(Debug, Clone)]
pub struct JoinWindowSecs(u8);

impl JoinWindowSecs {
    pub const _1MIN: Self = Self(60);
    /// Conventional 3 min window (e.g. all 'esp-zigbee-sdk' samples use this).
    pub const _3MIN: Self = Self(180);

    /// Allow non-default joining windows to be given. Note that 0xff is banned - as "joining forever".
    fn custom_secs(mut secs: u8) -> Self {
        if secs == 0xff {
            log::warn!("Changing {} -> {} (former would mean \"open forever\")", secs, 254);
            secs = 254;
        }
        Self(secs)
    }
}

trait DurationExt {
    fn secs(self) -> Duration;
}

impl DurationExt for i32 {
    const fn secs(self) -> Duration {
        Duration::from_secs(self as u64)
    }
}
