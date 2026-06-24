#![cfg(feature = "match_color_dimmable_light")]

use ezb_node_raw::{
    ezb_zcl_cluster_cmd_ctrl_s,
    ezb_zcl_color_control_move_to_color_cmd_payload_t,
    ezb_zcl_color_control_move_to_color_cmd_req,
    ezb_zcl_color_control_move_to_color_cmd_s,
    ezb_zcl_level_move_to_level_cmd_payload_t,
    ezb_zcl_level_move_to_level_with_on_off_cmd_req,
    ezb_zcl_level_move_to_level_with_on_off_cmd_t,
    ezb_err_e,
};

use crate::{
    utils::c_zeroed,
    AddrMode,
};

use super::{
    Accessor,
    AccessorCtx,
};

const TRANSITION_TIME: core::time::Duration = core::time::Duration::from_secs(1);

// Note: If there are more and more of the accessors, make the pattern into a macro.

/**
* Access to a Zigbee node (of color-dimmable-light profile) that we've bounded with.
*/
#[derive(Copy, Clone)]
pub struct AccessColorDimmableLight {
    ctx: AccessorCtx,
}

impl AccessColorDimmableLight {
    pub(super) fn new(ctx: AccessorCtx) -> Self {
        Self { ctx }
    }

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

        let err = self.ctx.guarded(|_node| {
            unsafe { ezb_zcl_color_control_move_to_color_cmd_req(&req) }
        });
        if err != 0 {
            match ezb_err_e::parse(err) {
                None => {
                    log::error!("Unexpected 'esp_zigbee_lib' error (ignored): {:?}", err);
                },
                Some(e) => {
                    // Note: Study how much we get these, and what's the right way to inform (or not) the application
                    //      about it. #later
                    //
                    //      If needing to pass errors to the application, do an enum and only carry those values we've
                    //      actually met, and whose circumstances are understood.
                    //
                    log::error!("'esp_zigbee_lib' error (ignored): {:?}", e);
                }
            }
        }
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
                src_ep: self.get().src_ep,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            },
            payload: ezb_zcl_level_move_to_level_cmd_payload_t {
                level,
                transition_time: ((TRANSITION_TIME.as_millis() +50) / 100) as u16,
                ..c_zeroed()    // like in C 'color_dimmer_switch' example
            }
        };

        let err = self.ctx.guarded(|_node| {
            unsafe { ezb_zcl_level_move_to_level_with_on_off_cmd_req(&req) }
        });
        if err != 0 {
            match ezb_err_e::parse(err) {
                None => {
                    log::error!("Unexpected 'esp_zigbee_lib' error (ignored): {:?}", err);
                },
                Some(e) => {
                    // Note: Study how much we get these, and what's the right way to inform (or not) the application
                    //      about it. #later
                    //
                    log::error!("'esp_zigbee_lib' error (ignored): {:?}", e);
                }
            }
        }
    }
}

impl Accessor for AccessColorDimmableLight {
    fn get(&self) -> &AccessorCtx {
        &self.ctx
    }
}
