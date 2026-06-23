#![cfg(feature = "match_color_dimmable_light")]

use embassy_sync::channel::DynamicReceiver;

use ezb_node_raw::ezb_zcl_cluster_id_e;

use crate::Node;

use super::{
    AccessColorDimmableLight,
    MatchError,
    MatchSuccess,
    MatchingContext,
};

/**
* Extension to node, allowing finding, binding and accessing nodes with a certain profile.
*/
#[cfg(feature = "match_color_dimmable_light")]
pub trait MatchColorDimmableLights: Node {
    // The clusters we demand from the other end
    const IN_CLUSTERS: &[ezb_zcl_cluster_id_e] = &[
        ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_ON_OFF,
        ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_LEVEL,
        ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_COLOR_CONTROL,
    ];
    const OUT_CLUSTERS: &[ezb_zcl_cluster_id_e] = &[];

    /**
    * Start matching; pass matches over as profile-specific access types.
    */
    fn start_matching_color_dimmable_lights(&self, src_ep: u8) -> DynamicReceiver<AccessColorDimmableLight> {

        let mc = MatchingContext::new_pinned(Self::IN_CLUSTERS, Self::OUT_CLUSTERS);

        mc.start_matching(src_ep, |match_res| {
            match match_res {
                MatchSuccess { short_addr, eps } => {

                },

                MatchError::ZdpError(err) => {

                },

                MatchError::Error(err) => {

                }
            }
            AccessColorDimmableLight{ src_ep, f: }
        })
    }
}
