#![cfg(feature = "remote_dt_color_dimmable_light")]

use embassy_time::Duration;

use ezb_node_raw::{
    ezb_zcl_cluster_cmd_ctrl_s,
    ezb_zcl_color_control_move_to_color_cmd_payload_t,
    ezb_zcl_color_control_move_to_color_cmd_req,
    ezb_zcl_color_control_move_to_color_cmd_s,
    ezb_zcl_level_move_to_level_cmd_payload_t,
    ezb_zcl_level_move_to_level_with_on_off_cmd_req,
    ezb_zcl_level_move_to_level_with_on_off_cmd_t,
    ezb_zcl_cluster_id_e::{self,
        EZB_ZCL_CLUSTER_ID_ON_OFF,
        EZB_ZCL_CLUSTER_ID_LEVEL,
        EZB_ZCL_CLUSTER_ID_COLOR_CONTROL,
    }
};

use crate::{
    utils::c_zeroed,
    AddrMode,
    EzbError,
};

use super::{
    Accessor,
};

const TRANSITION_TIME: Duration = Duration::from_secs(1);

/**
* Access to a Zigbee node (of color-dimmable-light profile) that we've bounded with.
*/
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct AccessColorDimmableLight<const SRC_EP: u8> {}

impl<const SRC_EP: u8> Accessor for AccessColorDimmableLight<SRC_EP> {
    const SRC_EP: u8 = SRC_EP;

    const IN_CLUSTERS: &[ezb_zcl_cluster_id_e] = &[
        EZB_ZCL_CLUSTER_ID_ON_OFF,
        EZB_ZCL_CLUSTER_ID_LEVEL,
        EZB_ZCL_CLUSTER_ID_COLOR_CONTROL,
    ];
    const OUT_CLUSTERS: &[ezb_zcl_cluster_id_e] = &[];

    fn new() -> Self {
        Self {}
    }
}

/// Additional methods for accessing a 'color-dimmable-light' end point.
///
impl<const SRC_EP: u8> AccessColorDimmableLight<SRC_EP> {
    /**
    * Send a request to set color x,y.
    *
    * @note The request is sent immediately to the Zigbee task. Confirmation of receiving can be listened to, on the
    *       Zigbee stream. We can consider making a system that both sends, and picks the response on the stream,
    *       as an async function.
    */
    pub fn set_color_xy(&self, color_x: u16, color_y: u16) {

        let req = ezb_zcl_color_control_move_to_color_cmd_s {
            cmd_ctrl: ezb_zcl_cluster_cmd_ctrl_s {
                dst_addr: AddrMode::None .into(),
                src_ep: Self::SRC_EP,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            },
            payload: ezb_zcl_color_control_move_to_color_cmd_payload_t {
                color_x,
                color_y,
                transition_time: ((TRANSITION_TIME.as_millis() +50) / 100) as u16,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            }
        };

        EzbError::wrap( Self::guarded(|| {
            unsafe { ezb_zcl_color_control_move_to_color_cmd_req(&req) }
        })).unwrap_or_else(|e| {

            // Study how much we get these, and what's the right way to inform (or not) the application
            // about it. tbd.
            //
            // If needing to pass errors to the application, do an enum and only carry those values we've
            // actually met, and whose circumstances are understood.
            //
            log::error!("'esp_zigbee_lib' error (ignored): {}", e);
        });
    }

    /**
    * Send a request to set color level.
    *
    * @note The request is sent immediately to the Zigbee task. Confirmation may come on the Zigbee stream.
    *       (see suggestion at comment of 'set_color_xy')
    */
    pub fn set_level(&self, level: u8) {
        let req = ezb_zcl_level_move_to_level_with_on_off_cmd_t {
            cmd_ctrl: ezb_zcl_cluster_cmd_ctrl_s {
                dst_addr: AddrMode::None .into(),
                src_ep: Self::SRC_EP,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            },
            payload: ezb_zcl_level_move_to_level_cmd_payload_t {
                level,
                transition_time: ((TRANSITION_TIME.as_millis() +50) / 100) as u16,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            }
        };

        EzbError::wrap( Self::guarded(|| {
            unsafe { ezb_zcl_level_move_to_level_with_on_off_cmd_req(&req) }
        })).unwrap_or_else(|e| {
            // Note: Study how much we get these, and what's the right way to inform (or not) the application
            //      about it. #later
            //
            log::error!("'esp_zigbee_lib' error (ignored): {:?}", e);
        });
    }
}
