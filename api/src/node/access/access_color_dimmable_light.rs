#![cfg(feature = "bind_color_dimmable_light")]

use ezb_node_raw::{
    ezb_zcl_cluster_id_e,
    ezb_zcl_color_control_move_to_color_cmd_payload_t,
    ezb_zcl_color_control_move_to_color_cmd_s,
    ezb_zcl_cluster_cmd_ctrl_s,
    ezb_address_s,
    ezb_zcl_color_control_move_to_color_cmd_req,
};

use crate::{
    Node
};
use crate::node::ZigbeeGuard;
use super::{
    //Access,
    //groupcast_matcher::GroupcastMatcher,
};

const TRANSITION_TIME: core::time::Duration = core::time::Duration::from_secs(1);

/**
* Access to a Zigbee node (of color-dimmable-light profile) that we've bounded with.
*/
pub(super) struct AccessColorDimmableLight : AccessTools {
    //node: &'static dyn Node,
    src_ep: u8,
}

impl AccessColorDimmableLight {

    pub fn set_color_xy(&self, color_x: u16, color_y: u16) {

        let req = ezb_zcl_color_control_move_to_color_cmd_s {
            cmd_ctrl: ezb_zcl_cluster_cmd_ctrl_s {
                dst_addr: ezb_address_s::NONE .into(),
                src_ep: self.src_ep,
                ..Default::default()    // fill with 0 like in C 'color_dimmer_switch' example
            },
            payload: ezb_zcl_color_control_move_to_color_cmd_payload_t {
                color_x,
                color_y,
                transition_time: ((TRANSITION_TIME.as_millis() +50) / 100) as u16,
                ..Default::default()    // fill with 0 like in C 'color_dimmer_switch' example
            }
        };

        self.guarded(|| {
            unsafe { ezb_zcl_color_control_move_to_color_cmd_req(req) }
        });
    }

    pub fn set_level(&self, level: u8) {
        unimplemented!()
    }
}
