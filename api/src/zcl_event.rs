
use core::{
    ffi::c_void,
    fmt,
    ptr::NonNull,
};
use strum;

use ezb_node_raw::{
    ezb_zcl_core_action_callback_id_e,
    ezb_zcl_status_e,
    ezb_zcl_attribute_s,
    ezb_zcl_cmd_hdr_t,
    ezb_zcl_read_attr_rsp_variable_t,
    ezb_app_signal_type_e,
    ezb_zcl_set_attr_value_message_t,
    ezb_zcl_cmd_default_rsp_message_t,
    ezb_zcl_message_info_s,
    ezb_zcl_cmd_read_attr_rsp_message_t,
    ezb_zcl_cmd_write_attr_rsp_message_t,
    RspVariableEntry,
    RspVariableIter,
};

use crate::types::{
    ClusterRole,
    CommandHeader,
    ClusterId,
    ZclAttr,
    ZclAttrResp,
    ZclError,
};

#[cfg(feature = "touchlink")]
use ezb_node_raw::{
    esp_zb_bdb_signal_touchlink_nwk_started_params_t,
    esp_zb_bdb_signal_touchlink_nwk_joined_router_t,
    esp_zb_bdb_signal_touchlink_nwk_started_params_t,
};
use ezb_node_raw::ezb_zcl_core_action_callback_id_e::EZB_ZCL_CORE_SET_ATTR_VALUE_CB_ID;

/**
* Enumeration of Zigbee Cluster Library (ZCL) "core action callback identifiers" (C term) and their data.
*
* These structs are used for the successful events.
*/
#[derive(Debug, Clone)]
pub enum ZclEvent {
    // Note: Below, the author has edited the 'raw' level doc-comments, to e.g. remove "callback", "identifier" technical
    //      terms.

    /// Triggered when an application-related attribute is changed.
    // ezb_zcl_set_attr_value_message_s
    SetAttrValue{ info: CommonInfo, attr: ZclAttr },

    /// A ZCL general ReadAttribute response is received.
    // ezb_zcl_cmd_read_attr_rsp_message_s
    ReadAttrResp{ info: CommonInfo, header: CommandHeader, variables: Vec<ZclAttrResp> },

    /// A ZCL general WriteAttribute response is received.
    // ezb_zcl_cmd_write_attr_rsp_message_s
    WriteAttrResp{ info: CommonInfo, header: CommandHeader, variables: Vec<ZclAttrResp> },

    /// A ZCL general ConfigureReporting response is received.
    // ezb_zcl_cmd_config_report_rsp_message_s
    #[cfg(false)]
    ConfigReportResp(HeaderAndVariables),

    /// A ZCL general ReadReportingConfiguration response is received.
    // ezb_zcl_cmd_read_report_config_rsp_message_s
    #[cfg(false)]
    ReadReportConfigResp(HeaderAndVariables),

    /// A ZCL General ReportAttribute command is received.
    // ezb_zcl_cmd_report_attr_message_s
    #[cfg(false)]
    ReportAttr(HeaderAndVariables),

    /// A ZCL general DiscoverAttributes response is received.
    // ezb_zcl_cmd_discover_attributes_rsp_message_s
    #[cfg(false)]
    DiscAttrResp(HeaderAndVariables),

    /// A ZCL general Discover response is received.
    // ezb_zcl_cmd_discover_commands_rsp_message_s
    #[cfg(false)]
    DiscCmdResp(HeaderAndIsRecvAndIsCompletedAndVariables),

    /// A ZCL general DefaultResponse response is received.
    ///
    /// @note 'cmd_id' is the command id, within the cluster, of the original request. E.g. 0x01 for *On* in
    ///         an On/Off cluster.
    ///
    // ezb_zcl_cmd_default_rsp_message_s
    DefaultResp{ info: CommonInfo, header: CommandHeader, cmd_id: u8, err: Option<ZclError> },

    /*** tbd. todo
    /// A callback ID triggered when a ZCL command is received with
    /// Manufacturer-Specific code.
    /// see @ref ezb_zcl_manuf_spec_cmd_message_s.
    EZB_ZCL_CORE_MANUF_SPEC_CMD_CB_ID = 9,
    /// A callback ID triggered when a ZCL Identify IdentifyEffect command is
    /// received. see @ref ezb_zcl_identify_effect_message_s.
    EZB_ZCL_CORE_IDENTIFY_EFFECT_CB_ID = 10,
    /// A callback ID triggered when a ZCL Basic ResetToFactoryDefault command
    /// is received. see @ref ezb_zcl_basic_reset_factory_default_message_s.
    EZB_ZCL_CORE_BASIC_RESET_TO_FACTORY_DEFAULT_CB_ID = 11,
    /// A callback ID triggered when a ZCL ON_OFF OffWithEffect command is
    /// received. see @ref ezb_zcl_on_off_off_with_effect_message_s.
    EZB_ZCL_CORE_ON_OFF_OFF_WITH_EFFECT_CB_ID = 12,
    /// A callback ID triggered when a ZCL Groups AddGroup command is received.
    /// see @ref ezb_zcl_groups_add_group_rsp_message_s.
    EZB_ZCL_CORE_GROUPS_ADD_GROUP_RSP_CB_ID = 13,
    /// A callback ID triggered when a ZCL Groups ViewGroup command is received.
    /// see @ref ezb_zcl_groups_view_group_rsp_message_s.
    EZB_ZCL_CORE_GROUPS_VIEW_GROUP_RSP_CB_ID = 14,
    /// A callback ID triggered when a ZCL Groups GetGroupMembership command is
    /// received.
    /// see @ref ezb_zcl_groups_get_group_membership_rsp_message_s.
    EZB_ZCL_CORE_GROUPS_GET_GROUP_MEMBERSHIP_RSP_CB_ID = 15,
    /// A callback ID triggered when a ZCL Groups RemoveGroup command is
    /// received. see @ref ezb_zcl_groups_remove_group_rsp_message_s.
    EZB_ZCL_CORE_GROUPS_REMOVE_GROUP_RSP_CB_ID = 16,
    /// A callback ID triggered when a ZCL Scenes OperateScene command is
    /// received. see @ref ezb_zcl_scenes_operate_scene_rsp_message_s.
    EZB_ZCL_CORE_SCENES_OPERATE_SCENE_RSP_CB_ID = 17,
    /// A callback ID triggered when a ZCL Scenes ViewScene command is received.
    /// see @ref ezb_zcl_scenes_view_scene_rsp_message_s.
    EZB_ZCL_CORE_SCENES_VIEW_SCENE_RSP_CB_ID = 18,
    /// A callback ID triggered when a ZCL Scenes GetSceneMembership command is
    /// received.
    /// see @ref ezb_zcl_scenes_get_scene_membership_rsp_message_s.
    EZB_ZCL_CORE_SCENES_GET_SCENE_MEMBERSHIP_RSP_CB_ID = 19,
    /// A callback ID triggered when a ZCL Scenes StoreScene command is
    /// received. see @ref ezb_zcl_scenes_store_scene_message_s.
    EZB_ZCL_CORE_SCENES_STORE_SCENE_CB_ID = 20,
    /// A callback ID triggered when a ZCL Scenes RecallScene command is
    /// received. see @ref ezb_zcl_scenes_recall_scene_message_s.
    EZB_ZCL_CORE_SCENES_RECALL_SCENE_CB_ID = 21,
    /// A callback ID triggered when a ZCL DoorLock Lock command is received.
    /// see @ref ezb_zcl_door_lock_lock_door_message_s.
    EZB_ZCL_CORE_DOOR_LOCK_LOCK_DOOR_CB_ID = 22,
    /// A callback ID triggered when a ZCL DoorLock Unlock command is received.
    /// see @ref ezb_zcl_door_lock_unlock_door_message_t.
    EZB_ZCL_CORE_DOOR_LOCK_UNLOCK_DOOR_CB_ID = 23,
    /// A callback ID triggered when a ZCL DoorLock LockResponse command is
    /// received. see @ref ezb_zcl_door_lock_lock_door_rsp_message_s.
    EZB_ZCL_CORE_DOOR_LOCK_LOCK_DOOR_RSP_CB_ID = 24,
    /// A callback ID triggered when a ZCL DoorLock UnlockResponse command is
    /// received. see @ref ezb_zcl_door_lock_unlock_door_rsp_message_t.
    EZB_ZCL_CORE_DOOR_LOCK_UNLOCK_DOOR_RSP_CB_ID = 25,
    /// A callback ID triggered when a ZCL WindowCovering Movement command is
    /// received. see @ref ezb_zcl_window_covering_movement_message_s.
    EZB_ZCL_CORE_WINDOW_COVERING_MOVEMENT_CB_ID = 26,
    /// A callback ID triggered when a ZCL ColorControl ColorModeChange command
    /// is received.
    /// see @ref ezb_zcl_color_control_color_mode_change_message_s.
    EZB_ZCL_CORE_COLOR_CONTROL_COLOR_MODE_CHANGE_CB_ID = 27,
    /// A callback ID triggered when a ZCL IAS ACE Arm command is received.
    /// see @ref ezb_zcl_ias_ace_arm_message_s.
    EZB_ZCL_CORE_IAS_ACE_ARM_CB_ID = 28,
    /// A callback ID triggered when a ZCL IAS ACE Bypass command is received.
    /// see @ref ezb_zcl_ias_ace_bypass_message_s.
    EZB_ZCL_CORE_IAS_ACE_BYPASS_CB_ID = 29,
    /// A callback ID triggered when a ZCL IAS ACE Emergency command is
    /// received. see @ref ezb_zcl_ias_ace_emergency_message_s.
    EZB_ZCL_CORE_IAS_ACE_EMERGENCY_CB_ID = 30,
    /// A callback ID triggered when a ZCL IAS ACE Fire command is received.
    /// see @ref ezb_zcl_ias_ace_emergency_message_s.
    EZB_ZCL_CORE_IAS_ACE_FIRE_CB_ID = 31,
    /// A callback ID triggered when a ZCL IAS ACE Panic command is received.
    /// see @ref ezb_zcl_ias_ace_emergency_message_s.
    EZB_ZCL_CORE_IAS_ACE_PANIC_CB_ID = 32,
    /// A callback ID triggered when a ZCL IAS ACE GetPanelStatus command is
    /// received. see @ref ezb_zcl_ias_ace_get_panel_status_message_s.
    EZB_ZCL_CORE_IAS_ACE_GET_PANEL_STATUS_CB_ID = 33,
    /// A callback ID triggered when a ZCL IAS ACE GetZoneStatus command is
    /// received. see @ref ezb_zcl_ias_ace_get_zone_status_message_s.
    EZB_ZCL_CORE_IAS_ACE_GET_ZONE_STATUS_CB_ID = 34,
    /// A callback ID triggered when a ZCL IAS ACE ArmResponse command is
    /// received. see @ref ezb_zcl_ias_ace_arm_rsp_message_s.
    EZB_ZCL_CORE_IAS_ACE_ARM_RSP_CB_ID = 35,
    /// A callback ID triggered when a ZCL IAS ACE GetZoneIdMapResponse command
    /// is received. see @ref ezb_zcl_ias_ace_get_zone_id_map_rsp_message_s.
    EZB_ZCL_CORE_IAS_ACE_GET_ZONE_ID_MAP_RSP_CB_ID = 36,
    /// A callback ID triggered when a ZCL IAS ACE GetZoneInfoResponse command
    /// is received. see @ref ezb_zcl_ias_ace_get_zone_info_rsp_message_s.
    EZB_ZCL_CORE_IAS_ACE_GET_ZONE_INFO_RSP_CB_ID = 37,
    /// A callback ID triggered when a ZCL IAS ACE ZoneStatusChanged command is
    /// received. see @ref ezb_zcl_ias_ace_zone_status_changed_message_s.
    EZB_ZCL_CORE_IAS_ACE_ZONE_STATUS_CHANGED_CB_ID = 38,
    /// A callback ID triggered when a ZCL IAS ACE PanelStatusChanged command is
    /// received. see @ref ezb_zcl_ias_ace_panel_status_changed_message_s.
    EZB_ZCL_CORE_IAS_ACE_PANEL_STATUS_CHANGED_CB_ID = 39,
    /// A callback ID triggered when a ZCL IAS ACE GetPanelStatusResponse
    /// command is received.
    /// see @ref ezb_zcl_ias_ace_get_panel_status_rsp_message_s.
    EZB_ZCL_CORE_IAS_ACE_GET_PANEL_STATUS_RSP_CB_ID = 40,
    /// A callback ID triggered when a ZCL IAS ACE SetBypassedZoneListResponse
    /// command is received.
    /// see @ref ezb_zcl_ias_ace_set_bypassed_zone_list_rsp_message_s.
    EZB_ZCL_CORE_IAS_ACE_SET_BYPASSED_ZONE_LIST_RSP_CB_ID = 41,
    /// A callback ID triggered when a ZCL IAS ACE BypassResponse command is
    /// received. see @ref ezb_zcl_ias_ace_bypass_rsp_message_s.
    EZB_ZCL_CORE_IAS_ACE_BYPASS_RSP_CB_ID = 42,
    /// A callback ID triggered when a ZCL IAS ACE GetZoneStatusResponse command
    /// is received. see @ref ezb_zcl_ias_ace_get_zone_status_rsp_message_s.
    EZB_ZCL_CORE_IAS_ACE_GET_ZONE_STATUS_RSP_CB_ID = 43,
    /// A callback ID triggered when a ZCL IAS WD StartWarning command is
    /// received. see @ref ezb_zcl_ias_wd_start_warning_message_s.
    EZB_ZCL_CORE_IAS_WD_START_WARNING_CB_ID = 44,
    /// A callback ID triggered when a ZCL IAS WD Squawk command is received.
    /// see @ref ezb_zcl_ias_wd_squawk_message_s.
    EZB_ZCL_CORE_IAS_WD_SQUAWK_CB_ID = 45,
    /// A callback ID triggered when a ZCL IAS Zone InitiateTestMode command is
    /// received. see @ref ezb_zcl_ias_zone_initiate_test_mode_message_s.
    EZB_ZCL_CORE_IAS_ZONE_INIT_TEST_MODE_CB_ID = 46,
    /// A callback ID triggered when a ZCL IAS Zone InitiateNormalMode command
    /// is received.
    /// see @ref ezb_zcl_ias_zone_initiate_normal_mode_message_s.
    EZB_ZCL_CORE_IAS_ZONE_INIT_NORMAL_MODE_CB_ID = 47,
    /// A callback ID triggered when a ZCL IAS Zone EnrollRequest command is
    /// received. see @ref ezb_zcl_ias_zone_enroll_req_message_s.
    EZB_ZCL_CORE_IAS_ZONE_ENROLL_CB_ID = 48,
    /// A callback ID triggered when a ZCL IAS Zone EnrollResponse command is
    /// received. see @ref ezb_zcl_ias_zone_enroll_rsp_message_s.
    EZB_ZCL_CORE_IAS_ZONE_ENROLL_RSP_CB_ID = 49,
    /// A callback ID triggered when a ZCL IAS Zone StatusChangeNotification
    /// command is received.
    /// see @ref ezb_zcl_ias_zone_status_change_notif_message_s.
    EZB_ZCL_CORE_IAS_ZONE_STATUS_CHANGE_NOTIF_CB_ID = 50,
    /// A callback ID triggered when a ZCL Alarms Alarm command is received.
    /// see @ref ezb_zcl_alarms_alarm_cmd_message_s.
    EZB_ZCL_CORE_ALARMS_ALARM_CB_ID = 51,
    /// A callback ID triggered when a ZCL Alarms GetAlarmResponse command is
    /// received. see @ref ezb_zcl_alarms_get_alarm_rsp_message_s.
    EZB_ZCL_CORE_ALARMS_GET_ALARM_RSP_CB_ID = 52,
    /// A callback ID triggered when a ZCL Alarms ResetAlarm command is
    /// received. see @ref ezb_zcl_alarms_reset_alarm_message_s.
    EZB_ZCL_CORE_ALARMS_RESET_ALARM_CB_ID = 53,
    /// A callback ID triggered when a ZCL Alarms ResetAllAlarms command is
    /// received. see @ref ezb_zcl_alarms_reset_all_alarms_message_s.
    EZB_ZCL_CORE_ALARMS_RESET_ALL_ALARMS_CB_ID = 54,
    /// A callback ID triggered when a ZCL Thermostat Setpoint command is
    /// received. see @ref ezb_zcl_thermostat_setpoint_message_s.
    EZB_ZCL_CORE_THERMOSTAT_SETPOINT_CB_ID = 55,
    /// A callback ID triggered when a ZCL Thermostat SetWeeklySchedule command
    /// is received.
    /// see @ref ezb_zcl_thermostat_set_weekly_schedule_message_s.
    EZB_ZCL_CORE_THERMOSTAT_SET_WEEKLY_SCHEDULE_CB_ID = 56,
    /// A callback ID triggered when a ZCL Thermostat GetWeeklyScheduleResponse
    /// command is received.
    /// see @ref ezb_zcl_thermostat_get_weekly_schedule_rsp_message_s.
    EZB_ZCL_CORE_THERMOSTAT_GET_WEEKLY_SCHEDULE_RSP_CB_ID = 57,
    /// A callback ID triggered when a ZCL OTA Upgrade Client Progress command
    /// is received. see @ref ezb_zcl_ota_upgrade_client_progress_message_s.
    EZB_ZCL_CORE_OTA_UPGRADE_CLIENT_PROGRESS_CB_ID = 58,
    /// A callback ID triggered when a ZCL OTA Upgrade QueryNextImageResponse
    /// command is received.
    /// see @ref ezb_zcl_ota_upgrade_query_next_image_rsp_message_s.
    EZB_ZCL_CORE_OTA_UPGRADE_QUERY_NEXT_IMAGE_RSP_CB_ID = 59,
    /// A callback ID triggered when a ZCL OTA Upgrade Server Progress command
    /// is received. see @ref ezb_zcl_ota_upgrade_server_progress_message_s.
    EZB_ZCL_CORE_OTA_UPGRADE_SERVER_PROGRESS_CB_ID = 60,
    /// A callback ID triggered when a ZCL PollControl CheckIn command is
    /// received. see @ref ezb_zcl_poll_control_check_in_message_s.
    EZB_ZCL_CORE_POLL_CONTROL_CHECK_IN_CB_ID = 61,
    /// A callback ID triggered when a ZCL ElectricalMeasurement GetProfileInfo
    /// command is received.
    /// see @ref ezb_zcl_electrical_measurement_get_prof_info_message_s.
    EZB_ZCL_CORE_ELECTRICAL_MEASUREMENT_GET_PROF_INFO_CB_ID = 62,
    /// A callback ID triggered when a ZCL ElectricalMeasurement
    /// GetMeasurementProfile command is received.
    /// see @ref ezb_zcl_electrical_measurement_get_meas_prof_message_s.
    EZB_ZCL_CORE_ELECTRICAL_MEASUREMENT_GET_MEAS_PROF_CB_ID = 63,
    /// A callback ID triggered when a ZCL ElectricalMeasurement
    /// GetProfileInfoResponse command is received.
    /// see @ref ezb_zcl_electrical_measurement_get_prof_info_rsp_message_s.
    EZB_ZCL_CORE_ELECTRICAL_MEASUREMENT_GET_PROF_INFO_RSP_CB_ID = 64,
    /// A callback ID triggered when a ZCL ElectricalMeasurement
    /// GetMeasurementProfileResponse command is received.
    /// GetMeasurementProfileResponse command is received.
    /// see @ref ezb_zcl_electrical_measurement_get_meas_prof_rsp_message_s.
    EZB_ZCL_CORE_ELECTRICAL_MEASUREMENT_GET_MEAS_PROF_RSP_CB_ID = 65,
    /// A callback ID triggered when a ZCL Metering GetProfile command is
    /// received. see @ref ezb_zcl_metering_get_profile_req_message_s.
    EZB_ZCL_CORE_METERING_GET_PROFILE_CB_ID = 66,
    /// A callback ID triggered when a ZCL Metering GetProfileResponse command
    /// is received. see @ref ezb_zcl_metering_get_profile_rsp_message_s.
    EZB_ZCL_CORE_METERING_GET_PROFILE_RSP_CB_ID = 67,
    /// A callback ID triggered when a ZCL Metering RequestFastPollMode command
    /// is received.
    /// see @ref ezb_zcl_metering_request_fast_poll_mode_req_message_s.
    EZB_ZCL_CORE_METERING_REQUEST_FAST_POLL_MODE_CB_ID = 68,
    /// A callback ID triggered when a ZCL Metering RequestFastPollModeResponse
    /// command is received.
    /// see @ref ezb_zcl_metering_request_fast_poll_mode_rsp_message_s.
    EZB_ZCL_CORE_METERING_REQUEST_FAST_POLL_MODE_RSP_CB_ID = 69,
    /// A callback ID triggered when a ZCL Metering GetSnapshot command is
    /// received. see @ref ezb_zcl_metering_get_snapshot_req_message_s.
    EZB_ZCL_CORE_METERING_GET_SNAPSHOT_CB_ID = 70,
    /// A callback ID triggered when a ZCL Metering PublishSnapshot command is
    /// received.
    /// see @ref ezb_zcl_metering_publish_snapshot_command_message_s.
    EZB_ZCL_CORE_METERING_PUBLISH_SNAPSHOT_CB_ID = 71,
    /// A callback ID triggered when a ZCL Metering GetSampledData command is
    /// received. see @ref ezb_zcl_metering_get_sampled_data_req_message_s.
    EZB_ZCL_CORE_METERING_GET_SAMPLED_DATA_CB_ID = 72,
    /// A callback ID triggered when a ZCL Metering GetSampledDataResponse
    /// command is received.
    /// see @ref ezb_zcl_metering_get_sampled_data_rsp_message_s.
    EZB_ZCL_CORE_METERING_GET_SAMPLED_DATA_RSP_CB_ID = 73,
    /// A callback ID triggered when a ZCL Price GetCurrentPrice command is
    /// received. see @ref ezb_zcl_price_get_current_price_message_s.
    EZB_ZCL_CORE_PRICE_GET_CURRENT_PRICE_CB_ID = 74,
    /// A callback ID triggered when a ZCL Price GetScheduledPrices command is
    /// received. see @ref ezb_zcl_price_get_scheduled_prices_message_s.
    EZB_ZCL_CORE_PRICE_GET_SCHEDULED_PRICES_CB_ID = 75,
    /// A callback ID triggered when a ZCL Price GetTierLabels command is
    /// received. see @ref ezb_zcl_price_get_tier_labels_message_s.
    EZB_ZCL_CORE_PRICE_GET_TIER_LABELS_CB_ID = 76,
    /// A callback ID triggered when a ZCL Price PriceAck command is received.
    /// see @ref ezb_zcl_price_price_ack_message_s.
    EZB_ZCL_CORE_PRICE_PRICE_ACK_CB_ID = 77,
    /// A callback ID triggered when a ZCL Price PublishPrice command is
    /// received. see @ref ezb_zcl_price_publish_price_message_s.
    EZB_ZCL_CORE_PRICE_PUBLISH_PRICE_CB_ID = 78,
    /// A callback ID triggered when a ZCL Price PublishTierLabels command is
    /// received. see @ref ezb_zcl_price_publish_tier_labels_message_s.
    EZB_ZCL_CORE_PRICE_PUBLISH_TIER_LABELS_CB_ID = 79,
    /// A callback ID triggered when a ZCL Touchlink EpInfo command is received.
    /// see @ref ezb_zcl_touchlink_ep_info_message_s.
    EZB_ZCL_CORE_TOUCHLINK_EP_INFO_CB_ID = 80,
    /// A callback ID triggered when a ZCL Touchlink GetGroupIdsResponse command
    /// is received. see @ref ezb_zcl_touchlink_get_group_ids_rsp_message_s.
    EZB_ZCL_CORE_TOUCHLINK_GET_GROUP_IDS_RSP_CB_ID = 81,
    /// A callback ID triggered when a ZCL Touchlink GetEndpointListResponse
    /// command is received.
    /// see @ref ezb_zcl_touchlink_get_ep_list_rsp_message_s.
    EZB_ZCL_CORE_TOUCHLINK_GET_ENDPOINT_LIST_RSP_CB_ID = 82,
    ***/

    /// The end of the ZCL core action events.
    // tbd. not sure when this actually happens
    End,
}


impl ZclEvent {
    /**
    * Convert from C level to Rust so that type and message are self-contained.
    */
    // @note 'vp' is 'mut' though we don't change it. 'NotNull' needs it that way, but this also allows us to (later)
    //      write to '.out' field if that ever becomes necessary (that write would need to be synchronous).
    //
    pub(crate) fn parse(e: ezb_zcl_core_action_callback_id_e, vp: *mut ::core::ffi::c_void) -> Option<Self> {
        use ezb_zcl_core_action_callback_id_e::*;

        // In C samples, the pointer is checked against 'null', and the callback returned from if it is.
        let Some(p) = NonNull::new(vp) else {
            log::debug!("Null pointer, no data");
            return None;
        };

        let happy_res = match e {
            EZB_ZCL_CORE_SET_ATTR_VALUE_CB_ID => {
                let msg = get_msg::<ezb_zcl_set_attr_value_message_t>(p);

                Self::SetAttrValue{
                    info: CommonInfo::parse(&msg.info)?,
                    attr: ZclAttr::parse(&msg.in_.attribute)?
                }
            },

            EZB_ZCL_CORE_READ_ATTR_RSP_CB_ID => {
                let msg = get_msg::<ezb_zcl_cmd_read_attr_rsp_message_t>(p);

                Self::ReadAttrResp{
                    info: CommonInfo::parse(&msg.info)?,
                    header: CommandHeader::parse(msg.in_.header)?,
                    variables: ZclAttrResp::parse_list(&msg.in_.variables)?
                }
            },

            EZB_ZCL_CORE_WRITE_ATTR_RSP_CB_ID => {
                let msg = get_msg::<ezb_zcl_cmd_write_attr_rsp_message_t>(p);

                Self::WriteAttrResp{
                    info: CommonInfo::parse(&msg.info)?,
                    header: CommandHeader::parse(msg.in_.header)?,
                    variables: ZclAttrResp::parse_list(&msg.in_.variables)?
                }
            }

            // ...

            EZB_ZCL_CORE_DEFAULT_RSP_CB_ID => {
                let msg = get_msg::<ezb_zcl_cmd_default_rsp_message_t>(p);
                    //typedef struct ezb_zcl_cmd_default_rsp_message_s {
                    //     ezb_zcl_message_info_t info; /*!< Common information about the received response. See @ref ezb_zcl_message_info_s. */
                    //     struct {
                    //         const ezb_zcl_cmd_hdr_t *header;      /*!< ZCL command header information. See @ref ezb_zcl_cmd_hdr_s. */
                    //         uint8_t                  rsp_to_cmd;  /*!< Command ID (0x00-0xFF) to which this is a response. */
                    //         uint8_t                  status_code; /*!< Status code of the response. See @ref ezb_zcl_status_t. */
                    //     } in;                                     /*!< Input data from the received default response. */
                    // } ezb_zcl_cmd_default_rsp_message_t;

                Self::DefaultResp{
                    info: CommonInfo::parse(&msg.info)?,
                    header: CommandHeader::parse(msg.in_.header)?,
                    cmd_id: msg.in_.rsp_to_cmd,
                    err: ZclError::parse(msg.in_.status_code)?
                }
            },

            // ...

            //
            EZB_ZCL_CORE_CB_ID_END => { // 83
                Self::End
            },
            _ => {
                return None;
            }
        };
        Some(happy_res)
    }
}

/* Define 'Display' but use '{:?}' internally.
* - enough for debug needs, and takes care that caller does not need to use '{:?}'
*/
impl fmt::Display for ZclEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/**
* Carrier of the '.info' fields - common to all ZCL Core events.
*/
#[derive(Debug, Clone)]
pub struct CommonInfo {
    #[allow(non_snake_case)]
    ///< Status of the message processing. 'None' for success; 'Some' for errors.
    pub err: Option<ZclError>, // status: ezb_zcl_status_e
    ///< The destination endpoint ID of the ZCL indication.
    pub dst_ep: u8,
    ///< The cluster ID of the ZCL indication.
    pub cluster_id: ClusterId, // u16
    ///< The role of cluster
    pub cluster_role: ClusterRole,
}

impl CommonInfo {
    fn parse(v: &ezb_zcl_message_info_s) -> Option<Self> {
        let ezb_zcl_message_info_s{
            status, dst_ep, cluster_id, cluster_role
        } = *v;

        Some( Self {
            err: ZclError::parse(status)?,
            dst_ep,
            cluster_id: ClusterId::parse(cluster_id)?,
            cluster_role: ClusterRole::parse(cluster_role)?
        })
    }
}


#[cfg(false)]   // #later
/**
* Message for 'ReadAttrResp'
*/
//typedef struct ezb_zcl_cmd_read_attr_rsp_message_s {
//     ezb_zcl_message_info_t info; /*!< Common information about the received message.
//                                           See @ref ezb_zcl_message_info_s. */
//     struct {
//         const ezb_zcl_cmd_hdr_t *header;             /*!< ZCL command header information.
//                                                           See @ref ezb_zcl_cmd_hdr_s. */
//         ezb_zcl_read_attr_rsp_variable_t *variables; /*!< Linked list of attribute response variables.
//                                                            Each variable contains one attribute's read result. */
//     } in;                                            /*!< Input data from the received response. */
//     struct {
//         ezb_zcl_status_t result; /*!< Status of processing in application. Set this to indicate how the application processed
//                                     the response. */
//     } out;                       /*!< Output data to be returned to the ZCL stack. */
// } ezb_zcl_cmd_read_attr_rsp_message_t;
struct ReadAttrRespM {
    r#in_x: ReadAttrRespM_In,
    st: Option<OutStatus>
}

#[cfg(false)]   // #later
#[allow(non_camel_case_types)]
struct ReadAttrRespM_In;

#[cfg(false)]   // #later
impl From<&ezb_zcl_set_attr_value_message_t> for ReadAttrRespM {
    fn from(v: &ezb_zcl_set_attr_value_message_t) -> Self {
        Self {
            in_x: ReadAttrRespM_In,
            st: OutStatus::from_try(v.out.result)
    }
}

/**
* Message for 'WriteAttrResp'
*/
#[cfg(false)]   // #later
//typedef struct ezb_zcl_cmd_write_attr_rsp_message_s {
//     ezb_zcl_message_info_t info; /*!< Common information about the received response. See @ref ezb_zcl_message_info_s. */
//     struct {
//         const ezb_zcl_cmd_hdr_t *header; /*!< ZCL command header information. See @ref ezb_zcl_cmd_hdr_s. */
//         ezb_zcl_write_attr_rsp_variable_t
//             *variables; /*!< Linked list of write response variables. Each variable contains one attribute's write result. */
//     } in;               /*!< Input data from the received response. */
//     struct {
//         ezb_zcl_status_t result; /*!< Status of processing in application. Set this to indicate how the application processed
//                                     the response. */
//     } out;                       /*!< Output data to be returned to the ZCL stack. */
// } ezb_zcl_cmd_write_attr_rsp_message_t;
struct WriteAttrRespM {
    r#in: WriteAttrRespM_In,
}

}

/**
* Message for 'ConfigReportResp'
*/
#[cfg(false)]   // #later
//typedef struct ezb_zcl_cmd_config_report_rsp_message_s {
//     ezb_zcl_message_info_t info; /*!< Common information about the received response. See @ref ezb_zcl_message_info_s. */
//     struct {
//         const ezb_zcl_cmd_hdr_t              *header;    /*!< ZCL command header information. See @ref ezb_zcl_cmd_hdr_s. */
//         ezb_zcl_config_report_rsp_variable_t *variables; /*!< Linked list of configure report response variables. Each variable
//                                                              contains one attribute's configuration result. */
//     } in;                                                /*!< Input data from the received response. */
//     struct {
//         ezb_zcl_status_t result; /*!< Status of processing in application. Set this to indicate how the application processed
//                                     the response. */
//     } out;                       /*!< Output data to be returned to the ZCL stack. */
// } ezb_zcl_cmd_config_report_rsp_message_t;
struct ConfigReportRespM {
    r#in: InHeaderAndVars,
    st: Option<OutStatus>
}

/**
* Re-interpret a void pointer, as a message of a certain type.
*
* @note #safety:
*       'esp_zigbee_lib' (2.0.1) uses '#[repr(packed)]' for the 'ezb_eui64_s' type (IEEE / Extended PAN addresses).
*       You CAN use this function with messages that contain such an address. Just DO NOT USE that resulting reference
*       to it. Instead, value-read the IEEE field with 'std::ptr::read_unaligned()'.
*
*       Alternatively, we can make this function return 'T' (by value), and use '::read_unaligned'.
*/
fn get_msg<'a, T>(vp: core::ptr::NonNull<core::ffi::c_void>) -> &'a T {
    let typed_ptr = vp.as_ptr() as *const T;
    unsafe { &*typed_ptr }
    //or: unsafe { Some(std::ptr::read_unaligned(typed_ptr)) }
}
