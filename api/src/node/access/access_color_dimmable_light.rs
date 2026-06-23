#![cfg(feature = "match_color_dimmable_light")]

use ezb_node_raw::{
    ezb_zcl_cluster_cmd_ctrl_s,
    ezb_zcl_color_control_move_to_color_cmd_payload_t,
    ezb_zcl_color_control_move_to_color_cmd_req,
    ezb_zcl_color_control_move_to_color_cmd_s,
    ezb_zcl_level_move_to_level_cmd_payload_t,
    ezb_zcl_level_move_to_level_with_on_off_cmd_req,
    ezb_zcl_level_move_to_level_with_on_off_cmd_t,
};

use crate::{
    utils::c_zeroed,
    AddrMode,
};

use super::{
    Accessor
};

const TRANSITION_TIME: core::time::Duration = core::time::Duration::from_secs(1);

/**
* Access to a Zigbee node (of color-dimmable-light profile) that we've bounded with.
*/
pub(super) trait AccessColorDimmableLight : Accessor {

    fn set_color_xy(&self, color_x: u16, color_y: u16) {

        let req = ezb_zcl_color_control_move_to_color_cmd_s {
            cmd_ctrl: ezb_zcl_cluster_cmd_ctrl_s {
                dst_addr: AddrMode::None .into(),
                src_ep: self.get().src_ep,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            },
            payload: ezb_zcl_color_control_move_to_color_cmd_payload_t {
                color_x,
                color_y,
                transition_time: ((TRANSITION_TIME.as_millis() +50) / 100) as u16,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            }
        };

        self.guarded(|&_node| {
            unsafe { ezb_zcl_color_control_move_to_color_cmd_req(&req) }
        });
    }

    fn set_level(&self, level: u8) {
        let req = ezb_zcl_level_move_to_level_with_on_off_cmd_t {
            cmd_ctrl: ezb_zcl_cluster_cmd_ctrl_s {
                dst_addr: AddrMode::None .into(),
                src_ep: self.get().src_ep,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            },
            payload: ezb_zcl_level_move_to_level_cmd_payload_t {
                level,
                transition_time: ((TRANSITION_TIME.as_millis() +50) / 100) as u16,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            }
        };

        self.guarded(|&_node| {
            unsafe { ezb_zcl_level_move_to_level_with_on_off_cmd_req(&req) }
        });
    }
}
