
use core::{
    fmt,
    time::Duration
};

use esp_zb_raw::{
    esp_err_t,
    esp_zb_app_signal_type_t,
    esp_zb_zdo_signal_device_annce_params_t,
    esp_zb_app_signal_get_params,
    esp_zb_zdo_signal_leave_params_t,
    esp_zb_nwk_signal_device_associated_params_t,
    esp_zb_zdo_signal_leave_indication_params_t,
    esp_zb_zdo_signal_can_sleep_params_t,
    esp_zb_zdo_signal_device_authorized_params_t,
    esp_zb_zdo_signal_device_update_params_t,
    esp_zb_zdo_signal_nwk_status_indication_params_t,
    esp_zb_zdo_device_unavailable_params_t,
};

#[cfg(feature = "touchlink")]
use esp_zb_raw::{
    esp_zb_bdb_signal_touchlink_nwk_started_params_t,
    esp_zb_bdb_signal_touchlink_nwk_joined_router_t,
    esp_zb_bdb_signal_touchlink_nwk_started_params_t,
};

#[cfg(feature = "gp")]  // only for consistency; not aiming to support
use esp_zb_raw::{
    esp_zb_zgp_signal_commissioning_params_t,
    esp_zb_zgp_signal_approve_comm_params_t,
};

use esp_idf_sys::{
    EspError,
    ESP_FAIL,
    ESP_ERR_INVALID_STATE
};

use crate::utils::IeeeAddr;

/**
* Enumeration of Zigbee APP signals
*/
#[derive(Debug)]
pub enum Signal {

    /**
    * Overview:
    * - The device has started in non-BDB commissioning mode.
    *
    * When generated:
    * - After the device has started and completed non-BDB commissioning.
    * - In case of a commissioning error.
    */
    // Status code:
    //  - ESP_OK: Device has started and joined the network.
    //  - ESP_FAIL: Device startup failure.
    //
    // Payload:
    //  - None
    //
    ZdoSignalDefaultStart{ joined: bool },

    /**
    * Overview:
    * - Stack framework (scheduler, buffer pool, NVRAM, etc.) startup complete,
    *   ready for initializing bdb commissioning.
    *
    * When generated:
    * - When the stack starts using the esp_zb_start(false) method.
    */
    // Status code:
    //  - ESP_OK: Stack framework has been initialized.
    //
    // Payload:
    //  - None
    //
    ZdoSignalSkipStartup,

    /**
    * Overview:
    * - Indicates that a Zigbee device has joined or rejoined the network.
    *
    * When generated:
    * - Upon receiving the device_annce command.
    */
    // Status code:
    //  - ESP_OK: device_annce command was received.
    //
    // Payload:
    //  - Refer to 'esp_zb_zdo_signal_device_annce_params_t'
    //
    ZdoSignalDeviceAnnce{
        device_short_addr: u16,
        ieee_addr: IeeeAddr,
        capability_X: u8    // tbd. consider if we want to make an enum
    },

    /**
    * Overview:
    * - Indicates that the device itself has left the network.
    *
    * When generated:
    * - Upon receiving the Leave command.
    */
    // Status code:
    //  - ESP_OK: Leave command was received.
    //
    // Payload:
    //  - Refer to 'esp_zb_zdo_signal_leave_params_t'
    //
    ZdoSignalLeave{
        leave_type_X: u8,   // really enum of 'esp_zb_nwk_leave_type_t'
    },

    /**
    * Overview:
    * - Indicates corrupted or incorrect signal information.
    *
    * When generated:
    * - When incorrect signal information is detected.
    */
    // Status code:
    //  - None
    //
    // Payload:
    //  - None
    ZdoSignalError,

    /**
    * Overview:
    * - Indicate the basic network information of factory new device has been initialized,
    *   ready for Zigbee commissioning
    *
    * When generated:
    * - Upon the basic device behavior has been initialization
    */
    // Status code:
    //  - ESP_OK: Factory new device initialization complete
    //  - ESP_FAIL: Factory new device commissioning failed
    //
    // Payload:
    //  - None
    BdbSignalDeviceFirstStart{ success: bool },

    /**
    * Overview:
    * - Indicate device joins or rejoins network from the configured network information.
    *
    * When generated:
    * - Upon the device joining or rejoining Zigbee network using configuration network information.
    */
    // Status code:
    //  - ESP_OK: Join or rejoin successfully
    //  - ESP_FAIL: Join or rejoin failed
    //
    // Payload:
    //  - None
    BdbSignalDeviceReboot{ success: bool },

    /**
    * Overview:
    * - Indicates that the Touchlink initiator has successfully started a network with the target and is ready
    *   for rejoining.
    *
    * When generated:
    * - Upon receiving the Network Start response during the Touchlink commissioning procedure.
    */
    // Status code:
    //  - ESP_OK: The new network has been started successfully.
    //
    // Payload:
    //  - Refer to esp_zb_bdb_signal_touchlink_nwk_started_params_t
    #[cfg(feature = "touchlink")]
    BdbSignalTouchlinkNwkStarted { x: esp_zb_bdb_signal_touchlink_nwk_started_params_t },

    /**
    * Overview:
    * - Indicate Touchlink target has join(ed) the initiator network.
    *
    * When generated:
    * - Upon Touchlink initiator receives the Network Start response during the Touchlink commissioning procedure.
    */
    // Status code:
    //  - ESP_OK: Touchlink target join successfully
    //
    // Payload:
    //  - Refer to esp_zb_bdb_signal_touchlink_nwk_joined_router_t
    #[cfg(feature = "touchlink")]
    BdbSignalTouchlinkNwkJoinedRouter { x: esp_zb_bdb_signal_touchlink_nwk_joined_router_t },

    /**
    * Overview:
    * - Indicates the result of the Touchlink initiator commissioning process.
    *
    * When generated:
    * - When the Touchlink initiator initiates network commission.
    */
    // Status code:
    //  - ESP_OK: Commissioning successful.
    //  - ESP_FAIL: No valid scan response received.
    //
    // Payload:
    //  - None
    #[cfg(feature = "touchlink")]
    BdbSignalTouchlink{ success: bool },

    /**
    * Overview:
    * - Indicates the completion of BDB network steering.
    *
    * When generated:
    * - When the device initiates the network steering commissioning process.
    */
    // Status code:
    //  - ESP_OK: Network steering completed successfully.
    //  - ESP_FAIL: Network steering failed or was canceled.
    //
    // Payload:
    //  - None
    BdbSignalSteering { success: bool },

    /**
    * Overview:
    * - Indicates the completion of BDB network formation.
    *
    * When generated:
    * - When the device initiates the network formation commissioning process.
    *   ^^--- tbd. THIS IS CLEARLY WRONG in upstream (esp-zigbee-lib sources): same as above entry!
    */
    // Status code:
    //  - ESP_OK: Network formation completed successfully.
    //  - ESP_FAIL: Network formation failed or was canceled.
    //
    // Payload:
    //  - None
    BdbSignalFormation { success: bool },

    /**
    * Overview:
    * - Indicates the completion of BDB finding and binding (F&B) for a target endpoint.
    *
    * When generated:
    * - When F&B target timeout.
    */
    // Status code:
    //  - ESP_OK: F&B target identifying time is expired.
    //  - ESP_FAIL: F&B target identifying is cancelled.
    //
    // Payload:
    //  - None
    BdbSignalFindingAndBindingTargetFinished { success: bool },
        // tbd. is 'expired' considered a success?
        //      likely we should/could join with the next one

    /**
    * Overview:
    * - Indicates the BDB F&B with a Target succeeded or F&B initiator timeout expired or cancelled.
    *
    * When generated:
    * - When F&B target timeout.
    */
    // Status code:
    //  - ESP_OK: On success.
    //  - ESP_FAIL: Expired or cancelled.
    //
    // Payload:
    //  - None
    BdbSignalFindingAndBindingInitiatorFinished { success: bool },
        // see above

    /**
    * Overview:
    * - Indicates that the Touchlink target is preparing to commission with the initiator.
    *
    * When generated:
    * - When the Touchlink procedure starts on the target device.
    */
    // Status code:
    //  - ESP_OK: Waiting for the commissioning procedure to proceed.
    //  - ESP_FAIL: Touchlink procedure failed or was canceled.
    //
    // Payload:
    //  - None
    #[cfg(feature = "touchlink")]
    BdbSignalTouchlinkTarget{ success: bool },

    /**
    * Overview:
    * - Indicates that the Touchlink target network has started.
    *
    * When generated:
    * - When the Touchlink target starts the network upon receiving a start_network, join_router, or join_ed request.
    */
    // Status code:
    //  - ESP_OK: Network started successfully.
    //  - ESP_FAIL: Network start failed or was canceled.
    //
    // Payload:
    //  - None
    #[cfg(feature = "touchlink")]
    BdbSignalTouchlinkNwk{ success: bool },

    /**
    * Overview:
    * - Indicates that the Touchlink target commissioning procedure has finished.
    *
    * When generated:
    * - When the Touchlink target times out or completes the commissioning procedure.
    */
    // Status code:
    //  - ESP_OK: Commissioning procedure completed successfully.
    //
    // Payload:
    //  - None
    #[cfg(feature = "touchlink")]
    BdbSignalTouchlinkTargetFinished,

    /**
    * Overview:
    * - Indicates that a new device has initiated an association procedure.
    *
    * When generated:
    * - When a new device is associated.
    */
    // Status code:
    //  - ESP_OK: The new device was successfully associated.
    //
    // Payload:
    //  - Refer to esp_zb_nwk_signal_device_associated_params_t
    NwkSignalDeviceAssociated{
        device_addr: IeeeAddr
    },

    /**
    * Overview:
    * - Indicates that a child device has left the network.
    *
    * When generated:
    * - When the leave command is received from the child device.
    */
    // Status code:
    // - ESP_OK: The child device left the network successfully.
    //
    // Payload:
    // - Refer to esp_zb_zdo_signal_leave_indication_params_t
    ZdoSignalLeaveIndication{
        short_addr: u16,
        device_addr: IeeeAddr,
        rejoin: bool
    },

    /**
    * Overview:
    * - Indicates the GPCB (Green Power Combo Basic) commissioning signal.
    *
    * When generated:
    * - When a device is commissioned or decommissioned by the GPCB.
    */
    // Status code:
    //  - ESP_OK: Commissioning or decommissioning completed successfully.
    //
    // Payload:
    //  - Refer to esp_zb_zgp_signal_commissioning_params_t
    #[cfg(feature = "gp")]
    ZgpSignalCommissioning { x: esp_zb_zgp_signal_commissioning_params_t },

    /**
    * Overview:
    * - Indicates the device can enter sleep mode.
    *
    * When generated:
    * - When the stack determines that the device is eligible to enter sleep mode.
    */
    // Status code:
    //  - ESP_OK: The device can enter sleep mode.
    //
    // Payload:
    //  - Refer to esp_zb_zdo_signal_can_sleep_params_t
    CommonSignalCanSleep{
        sleep_duration: Duration,
    },

    /**
    * Overview:
    * - Indicates whether a specific part of the production configuration was found.
    *
    * When generated:
    * - After restoring the production configuration.
    */
    // Status code:
    //  - ESP_OK: Production configuration successfully loaded from storage.
    //  - ESP_FAIL: No production configuration found in storage.
    //
    // Payload:
    // - None
    ZdoSignalProductionConfigReady{ success: bool },

    /**
    * Overview:
    * - Indicates that the Neighbor Table has expired, and no active route links remain.
    *
    * When generated:
    * - When all routes have expired.
    */
    // Status code:
    //  - ESP_OK: All route[r]s have expired.
    //                     ^-- TYPO in #upstream; #report
    // Payload:
    // - None
    NwkSignalNoActiveLinksLeft,

    /**
    * Overview:
    * - Indicates that a new device has been authorized by the Trust Center in the network.
    *
    * When generated:
    *  - Upon successful authorization.
    *  - Upon authorization failure.
    *  - Upon authorization timeout.
    */
    // Status code:
    //  - ESP_OK: New device is authorized.
    //
    // Payload:
    //  - Refer to esp_zb_zdo_signal_device_authorized_params_t
    ZdoSignalDeviceAuthorized{
        long_addr: IeeeAddr,
        short_addr: u16,
        authorization_typeX: u8,    // tbd. enum?
        authorization_statusX: u8,  // tbd. enum?  ..perheps as '.authorization', together with the type?
    },

    /**
    * Overview:
    * - Indicates that a device has joined, rejoined, or left the network from the Trust Center or its parents.
    *
    * When generated:
    *  - Standard device secured rejoin.
    *  - Standard device unsecured join.
    *  - Device left.
    *  - Standard device trust center rejoin.
    */
    // Status code:
    // - ESP_OK: New device information updated.
    //
    // Payload:
    // - Refer to esp_zb_zdo_signal_device_update_params_t
    ZdoSignalDeviceUpdate{
        long_addr: IeeeAddr,
        short_addr: u16,
        statusX: u8,    // tbd. enum?
        tc_actionX: u8,    // tbd. enum?
        parent_short: u16
    },

    /**
    * Overview:
    * - Detects a PAN ID conflict and inquires for a resolution.
    *
    * When generated:
    *  - Upon detecting a PAN ID conflict.
    */
    // Status code:
    //  - ESP_OK: On success.
    //
    // Payload:
    //  - None
    NwkSignalPanidConflictDetected,

    /**
    * Overview:
    * - Indicates that a network failure has been detected.
    *
    * When generated:
    * - Triggered when the network encounters a failure and the application can implement
    *   error handling based on the reported status.
    */
    // Status code:
    //  - ESP_OK: On success.
    //
    // Payload:
    //  - Refer to esp_zb_zdo_signal_nwk_status_indication_params_t
    NlmeStatusIndication {
        statusX: u8,    // tbd. enum
        network_addr: u16,
        unknown_command_id: u8
    },

    /**
    * Overview:
    * - Indicates that the Trust Center rejoin procedure has been completed.
    *
    * When generated:
    *  - Upon successful completion of the TC rejoin procedure by the device.
    */
    // Status code:
    //  - ESP_OK: TC rejoin completed successfully.
    //  - ESP_FAIL: Rejoin failed.
    //
    // Payload:
    //  - None
    BdbSignalTcRejoinDone{ success: bool },

    /**
    * Overview:
    * - Indicates the status of the network (open or closed).
    *
    * When generated:
    *  - When the network is opened.
    *  - When the network is closed.
    */
    // Status code:
    //  - ESP_OK: On successful operation.
    //
    // Payload:
    //  - Pointer to 'uint8_t', indicating:
    //      0: network is closed
    //      >0: network is open for {this number of} seconds
    NwkSignalPermitJoinStatus{ isOpened: Option<IsOpenedForSecs> },

    /**
    * Overview:
    * - Indicates the result of cancelling BDB steering.
    *
    * When generated:
    *  - When `esp_zb_bdb_cancel_steering()` is processed.
    */
    // Status code:
    //  - ESP_OK: Steering is cancelled successfully.
    //  - ESP_ERR_INVALID_STATE: Steering is not in progress.
    //  - ESP_FAIL: Failed to cancel steering.
    //
    // Payload:
    //  - None
    BdbSignalSteeringCancelled(Result<(),EspError>),

    /**
    * Overview:
    * - Notifies the result of cancelling BDB formation.
    *
    * When generated:
    *  - When `esp_zb_bdb_cancel_formation()` is processed.
    */
    // Status code:
    //  - ESP_OK: Formation is cancelled successfully.
    //  - ESP_ERR_INVALID_STATE: Formation is not in progress.
    //  - ESP_FAIL: Failed to cancel formation.
    //
    // Payload:
    //  - None
    BdbSignalFormationCancelled(Result<(),EspError>),

    /**
    * Overview:
    * - Indicates a ZGP mode change.
    *
    * When generated:
    *  - When a GPCB Sink changes mode between operational mode and commissioning mode.
    */
    // Status code:
    //  - ESP_OK: On success.
    //
    // Payload:
    //  - None
    #[cfg(feature = "gp")]
    ZgpSignalModeChange,

    /**
    * Overview:
    * - Notify that the destination device is unavailable.
    *
    * When generated:
    *  - When the stack could not send a packet over NWK or APS, for example:
    *      - No ACK on the MAC layer;
    *      - No response to a network address request;
    *      - No APS-ACK to an APS packet.
    */
    // Status code:
    //  - ESP_OK: On success.
    //
    // Payload:
    //  - Refer to esp_zb_zdo_device_unavailable_params_t
    ZdoDeviceUnavailable{
        long_addr: IeeeAddr,
        short_addr: u16
    },

    /**
    * Overview:
    * - ZGP Approve Commissioning.
    *
    * When generated:
    *  - When the ZGP subsystem is ready to create a new pairing, but the APP should check
    *    if the GPD application functionality matches to continue the pairing.
    */
    // Status code:
    //  - ESP_OK: On success.
    //
    // Payload:
    //  - Refer to esp_zb_zgp_signal_approve_comm_params_t
    #[cfg(feature = "gp")]
    ZgpSignalApproveCommissioning { x: esp_zb_zgp_signal_approve_comm_params_t },

    SignalEnd,
}

impl Signal {
    /**
    * Convert the application signal from Zigbee C level to Rust so that
    *   - all information is contained in a single, values-only enum
    *
    * For some signals, this means fetching extraneous information, using the C API global functions.
    */
    pub(crate) fn from(p_app_signal: *const esp_zb_app_signal_type_t, st: esp_err_t) -> Option<Self> {
        use esp_zb_app_signal_type_t::*;    // signal values

        assert!(!p_app_signal.is_null());
        let app_signal = unsafe { *p_app_signal };
            // ..but keep 'p_app_signal', may be used to claim more info

        let err: Option<EspError> = EspError::from(st);

        /* Helpers. Apply given value or closure when 'err' is known. Otherwise, provide 'None'.
        */
        #[cfg(false)]
        let if_OK = |f: Fn() -> Self| -> Option<Self> {
            err.is_none().then(f)   // 'then()' is lazy
        };
        macro_rules! if_ok {
            (|| $body:expr) => {
                err.is_none().then(|| $body)
            };
        }
        #[cfg(false)]
        let if_OK_FAIL = |f: Fn(bool) -> Self| -> Option<Self> {
            match err {
                None => Some(f(true)),
                Some(ESP_FAIL) => Some(f(false)),
                _ => None
            }
        };
        macro_rules! if_ok_fail {
            (|$name:ident| $body:expr) => {
                match err {
                    None => Some(true),
                    //Some(EspError(ESP_FAIL)) => Some(false),
                    Some(e) if e.code() == ESP_FAIL => Some(false),
                    _ => None
                }.map(|x| (|$name| $body)(x))
            };
        }
        #[cfg(false)]
        let if_OK_FAIL_INVALIDSTATE = |f: Fn(bool) -> Self| -> Option<Self> {
            match err {
                None => Some(f(None)),
                Some(ESP_FAIL) => Some(f(false)),
                Some(ESP_INVALIDSTATE) => Some(f(false)),
                _ => None
            }
        };
        macro_rules! if_ok_fail_invalidstate {
            (|$name:ident| $body:expr) => {
                match err {
                    None => Some(Result::Ok(())),
                    //Some(v@ EspError(ESP_FAIL)) |
                    //Some(v@ EspError(ESP_ERR_INVALID_STATE)) => Some(Result::Err(v)),
                    Some(e) if e.code() == ESP_FAIL || e.code() == ESP_ERR_INVALID_STATE => Some(Result::Err(e)),
                    _ => None
                }.map(|x| (|$name| $body)(x))
            };
        }

        match app_signal {
            ESP_ZB_ZDO_SIGNAL_DEFAULT_START => { // 0x00
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!( |joined| Self::ZdoSignalDefaultStart { joined } )
            },
            ESP_ZB_ZDO_SIGNAL_SKIP_STARTUP => { // 0x01
                // assert: 'st' ∈ ESP_OK
                if_ok!(|| Self::ZdoSignalSkipStartup)
            },
            ESP_ZB_ZDO_SIGNAL_DEVICE_ANNCE => { // 0x02
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  uint16_t device_short_addr;           /*!< address of device that recently joined to network */
                //  esp_zb_ieee_addr_t   ieee_addr;       /*!< The 64-bit (IEEE) address assigned to the device. */
                //  uint8_t       capability;             /*!< The capability of the device. */
                //
                if_ok!(|| {
                    let x = get_param::<esp_zb_zdo_signal_device_annce_params_t>(p_app_signal);
                    Self::ZdoSignalDeviceAnnce {
                        device_short_addr: x.device_short_addr,
                        ieee_addr: x.ieee_addr.into(),
                        capability_X: x.capability
                    }
                })
            },
            ESP_ZB_ZDO_SIGNAL_LEAVE => { // 0x03
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  uint8_t leave_type;             /*!< Leave type, refer to esp_zb_nwk_leave_type_t */

                if_ok!(|| {
                    let x = get_param::<esp_zb_zdo_signal_leave_params_t>(p_app_signal);
                    Self::ZdoSignalLeave {
                        leave_type_X: x.leave_type
                    }
                })
            },
            ESP_ZB_ZDO_SIGNAL_ERROR => { // 0x04
                // assert: 'st' ∈ ESP_OK
                if_ok!(|| Self::ZdoSignalError)
            },
            ESP_ZB_BDB_SIGNAL_DEVICE_FIRST_START => { // 0x05
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalDeviceFirstStart { success })
            },
            ESP_ZB_BDB_SIGNAL_DEVICE_REBOOT => { // 0x06
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalDeviceReboot { success })
            },
            #[cfg(feature = "touchlink")]
            ESP_ZB_BDB_SIGNAL_TOUCHLINK_NWK_STARTED => { // 0x07
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  esp_zb_ieee_addr_t device_ieee_addr;    /*!< The ieee address of touchlink target */
                //  uint8_t endpoint;                       /*!< The endpoint id on the touchlink target */
                //  uint16_t profile_id;                    /*!< The profile id of touchlink profile */
                let x = get_param::<esp_zb_bdb_signal_touchlink_nwk_started_params_t>();
                Self::BdbSignalTouchlinkNwkStarted { x }
            },
            #[cfg(feature = "touchlink")]
            ESP_ZB_BDB_SIGNAL_TOUCHLINK_NWK_JOINED_ROUTER => { // 0x08
                // assert: 'st' ∈ ESP_OK
                // payload: *tbd. describe*
                let x = get_param::<esp_zb_bdb_signal_touchlink_nwk_joined_router_t>();
                Self::BdbSignalTouchlinkNwkJoinedRouter { x }
            },
            #[cfg(feature = "touchlink")]
            ESP_ZB_BDB_SIGNAL_TOUCHLINK => { // 0x09
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalTouchlink { success })
            },
            ESP_ZB_BDB_SIGNAL_STEERING => { // 0x0a
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalSteering { success })
            },
            ESP_ZB_BDB_SIGNAL_FORMATION => { // 0x0b
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalFormation { success })
            },
            ESP_ZB_BDB_SIGNAL_FINDING_AND_BINDING_TARGET_FINISHED => { // 0x0c
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalFindingAndBindingTargetFinished { success })
            },
            ESP_ZB_BDB_SIGNAL_FINDING_AND_BINDING_INITIATOR_FINISHED => { // 0x0d
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalFindingAndBindingInitiatorFinished { success })
            },
            #[cfg(feature = "touchlink")]
            ESP_ZB_BDB_SIGNAL_TOUCHLINK_TARGET => { // 0x0e
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalTouchlinkTarget { success })
            },
            #[cfg(feature = "touchlink")]
            ESP_ZB_BDB_SIGNAL_TOUCHLINK_NWK => { // 0x0f
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalTouchlinkNwk { success })
            },
            #[cfg(feature = "touchlink")]
            ESP_ZB_BDB_SIGNAL_TOUCHLINK_TARGET_FINISHED => { // 0x10
                // assert: 'st' ∈ ESP_OK
                if_ok!(|| Self::BdbSignalTouchlinkTargetFinished)
            },
            // "reserved 0x11"
            ESP_ZB_NWK_SIGNAL_DEVICE_ASSOCIATED => { // 0x12
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  esp_zb_ieee_addr_t device_addr; /*!< address of associated device */
                if_ok!(|| {
                    let x = get_param::<esp_zb_nwk_signal_device_associated_params_t>(p_app_signal);
                    Self::NwkSignalDeviceAssociated {
                        device_addr: x.device_addr.into(),
                    }
                })
            },
            ESP_ZB_ZDO_SIGNAL_LEAVE_INDICATION => { // 0x13
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  uint16_t short_addr;                    /*!< Short address of device requested to leave or leaving device*/
                //  esp_zb_ieee_addr_t device_addr;         /*!< Long address of device requested to leave or leaving device*/
                //  uint8_t rejoin;                         /*!< 1 if this was leave with rejoin; 0 - otherwise */

                if_ok!(|| {
                    let x = get_param::<esp_zb_zdo_signal_leave_indication_params_t>(p_app_signal);
                    Self::ZdoSignalLeaveIndication {
                        short_addr: x.short_addr,
                        device_addr: x.device_addr.into(),
                        rejoin: match x.rejoin {
                            0 => false,
                            _ => true,
                        }
                    }
                })
            },
            // "reserved 0x14"
            #[cfg(feature = "gp")]
            ESP_ZB_ZGP_SIGNAL_COMMISSIONING => { // 0x15
                // assert: 'st' ∈ ESP_OK
                // payload: *tbd. describe*
                let x = get_param::<esp_zb_zgp_signal_commissioning_params_t>();
                Self::ZdoSignalComissioning { x }
            },
            ESP_ZB_COMMON_SIGNAL_CAN_SLEEP => { // 0x16
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  uint32_t sleep_duration; /*!< sleep duration in milliseconds */
                if_ok!(|| {
                    let x = get_param::<esp_zb_zdo_signal_can_sleep_params_t>(p_app_signal);
                    Self::CommonSignalCanSleep {
                        sleep_duration: Duration::from_millis(x.sleep_duration as u64)
                    }
                })
            },
            ESP_ZB_ZDO_SIGNAL_PRODUCTION_CONFIG_READY => { // 0x17
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::ZdoSignalProductionConfigReady { success })
            },
            ESP_ZB_NWK_SIGNAL_NO_ACTIVE_LINKS_LEFT => { // 0x18
                // assert: 'st' ∈ ESP_OK
                if_ok!(|| Self::NwkSignalNoActiveLinksLeft)
            },
            // "reserved: 0x19 - 0x2e"
            ESP_ZB_ZDO_SIGNAL_DEVICE_AUTHORIZED => { // 0x2f
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  esp_zb_ieee_addr_t long_addr; /*!< Long Address of the updated device */
                //  uint16_t short_addr;          /*!< Short Address of the updated device */
                //  uint8_t authorization_type;   /*!< Type of the authorization procedure */
                //  uint8_t authorization_status; /*!< Status of the authorization procedure which depends on authorization_type */
                if_ok!(|| {
                    let x = get_param::<esp_zb_zdo_signal_device_authorized_params_t>(p_app_signal);
                    Self::ZdoSignalDeviceAuthorized {
                        long_addr: x.long_addr.into(),
                        short_addr: x.short_addr,
                        authorization_typeX: x.authorization_type,
                        authorization_statusX: x.authorization_status
                    }
                })
            },
            ESP_ZB_ZDO_SIGNAL_DEVICE_UPDATE => { // 0x30
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  esp_zb_ieee_addr_t long_addr;   /*!< Long Address of the updated device */
                //  uint16_t short_addr;            /*!< Short Address of the updated device */
                //  uint8_t status;                 /*!< Indicates the updated status of the device, refer to esp_zb_zdo_update_dev_status_t */
                //  uint8_t tc_action;              /*!< Trust center action,  refer to esp_zb_zdo_update_dev_tc_action_t */
                //  uint16_t parent_short;          /*!< The short address of device's parent */
                if_ok!(|| {
                    let x = get_param::<esp_zb_zdo_signal_device_update_params_t>(p_app_signal);
                    Self::ZdoSignalDeviceUpdate {
                        long_addr: x.long_addr.into(),
                        short_addr: x.short_addr,
                        statusX: x.status,
                        tc_actionX: x.tc_action,
                        parent_short: x.parent_short
                    }
                })
            },
            ESP_ZB_NWK_SIGNAL_PANID_CONFLICT_DETECTED => { // 0x31
                // assert: 'st' ∈ ESP_OK
                if_ok!(|| Self::NwkSignalPanidConflictDetected)
            },
            ESP_ZB_NLME_STATUS_INDICATION => { // 0x32
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  uint8_t status;               /*!< Error code associated with the failure, refer to esp_zb_nwk_command_status_t */
                //  uint16_t network_addr;        /*!< Network device address associated with the status information */
                //  uint8_t unknown_command_id;   /*!< Unknown command ID */
                if_ok!(|| {
                    let x = get_param::<esp_zb_zdo_signal_nwk_status_indication_params_t>(p_app_signal);
                    Self::NlmeStatusIndication {
                        statusX: x.status,
                        network_addr: x.network_addr,
                        unknown_command_id: x.unknown_command_id
                    }
                })
            },
            // "reserved: 0x33, 0x34"
            ESP_ZB_BDB_SIGNAL_TC_REJOIN_DONE => { // 0x35
                // assert: 'st' ∈ ESP_OK, ESP_FAIL
                if_ok_fail!(|success| Self::BdbSignalTcRejoinDone { success })
            },
            ESP_ZB_NWK_SIGNAL_PERMIT_JOIN_STATUS => { // 0x36
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  '*uint8_t', "indicating the network status (open or closed)"
                if_ok!(|| {
                    let x = get_param::<u8>(p_app_signal);
                    Self::NwkSignalPermitJoinStatus{ isOpened: IsOpenedForSecs::from_raw(x) }
                })
            },
            ESP_ZB_BDB_SIGNAL_STEERING_CANCELLED => { // 0x37
                // assert: 'st' ∈ ESP_OK, ESP_FAIL, ESP_ERR_INVALID_STATE
                //
                if_ok_fail_invalidstate!(|res| {
                    Self::BdbSignalSteeringCancelled(res)
                })
            },
            ESP_ZB_BDB_SIGNAL_FORMATION_CANCELLED => { // 0x38
                // assert: 'st' ∈ ESP_OK, ESP_FAIL, ESP_ERR_INVALID_STATE
                //
                if_ok_fail_invalidstate!(|res| {
                    Self::BdbSignalFormationCancelled(res)
                })
            },
            // "reserved: 0x39, 0x3a"
            #[cfg(feature = "gp")]
            ESP_ZB_ZGP_SIGNAL_MODE_CHANGE => { // 0x3b
                // assert: 'st' ∈ ESP_OK
                Self::ZgpSignalModeChange
            },
            ESP_ZB_ZDO_DEVICE_UNAVAILABLE => { // 0x3c
                // assert: 'st' ∈ ESP_OK
                // payload:
                //  esp_zb_ieee_addr_t long_addr; /*!< Long address of the unavailable device */
                //  uint16_t short_addr;          /*!< Short address of unavailable device */
                if_ok!(|| {
                    let x = get_param::<esp_zb_zdo_device_unavailable_params_t>(p_app_signal);
                    Self::ZdoDeviceUnavailable {
                        long_addr: x.long_addr.into(),
                        short_addr: x.short_addr
                    }
                })
            },
            #[cfg(feature = "gp")]
            ESP_ZB_ZGP_SIGNAL_APPROVE_COMMISSIONING => { // 0x3d
                // assert: 'st' ∈ ESP_OK
                // payload: *tbd. describe*
                let x = get_param::<esp_zb_zgp_signal_approve_comm_params_t>();
                Self::ZgpSignalApproveCommissioning { x }
            },

            // google.ai: marker to show that some list or process has been successfully gone through.
            //
            ESP_ZB_SIGNAL_END => { // 0x3e
                // assert: 'st' ∈ (unknown, not described in 'esp-zigbee-lib' comments)
                if_ok!(|| Self::SignalEnd)
            },
            _ => None
        }
    }
}

/* Define 'Display' but use '{:?}' internally.
* - enough for debug needs, and takes care that caller does not need to use '{:?}'
*/
impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/*
* The 'p_app_signal' pointer can point not only to 'u8' but sometimes to signal
* specific structures with more information. Get them.
*/
fn get_param<T: Copy>(p_app_signal: *const esp_zb_app_signal_type_t) -> T {
    let p = unsafe { esp_zb_app_signal_get_params(p_app_signal as *mut _) } as *const T;

    assert!(!p.is_null());  // 'esp-zigblee-lib' would not scr*w, right?
    unsafe { *p }
}

//---
// Parameter for 'NwkSignalPermitJoinStatus'
//
#[derive(Debug)]
pub struct IsOpenedForSecs(pub u8);

impl IsOpenedForSecs {
    fn from_raw(x: u8) -> Option<Self> {
        match x {
            0 => None,
            _ => Some( IsOpenedForSecs(x) )
        }
    }
}
