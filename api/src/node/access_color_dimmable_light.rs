#![cfg(feature = "bind_color_dimmable_light")]

use ezb_node_raw::ezb_zcl_cluster_id_e;
use crate::Node;
use crate::node::groupcast_matcher::GroupcastMatcher;

/**
* Access to a Zigbee node (of specific profile).
*/
pub struct AccessColorDimmableLightX<'a> {
    // 'Node' is a singleton, so we don't really need this field to access it.
    node: &'a dyn Node,
}

impl<'a> AccessColorDimmableLightX<'a> {
    /// The clusters we demand from the other end, within matching.
    const IN_CLUSTERS: [ezb_zcl_cluster_id_e;3] = [
        ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_ON_OFF,
        ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_LEVEL,
        ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_COLOR_CONTROL,
    ];
    const OUT_CLUSTERS: [ezb_zcl_cluster_id_e;0] = [];

    pub fn set_color(&self, red: u8, green: u8, blue: u8) {
        unimplemented!();
        unsafe {
        }
    }

    pub fn set_level(&self, level: u8) {
        unimplemented!()
    }
}

/*
* Extension to node methods.
*/
#[cfg(feature = "bind_color_dimmable_light")]
pub trait AccessColorDimmableLights where Self: Node {

    fn start_matching_color_dimmable_lights(&self) -> AccessColorDimmableLightX {

        let matcher = GroupcastMatcher::new();

        matcher.find_and_bind() .await;
    }
}
