#![cfg(feature = "match_color_dimmable_light")]

use ezb_node_raw::ezb_zcl_cluster_id_e;

use futures_util::Stream;

use super::{
    AccessColorDimmableLight
};

use crate::{
    Node,
    ShortAddr,
};

use super::MatchingContext;

pub enum MatchEvent {
    Success(dyn AccessColorDimmableLight),
    MatchError,
}

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
    fn start_matching_color_dimmable_lights(&self, src_ep: u8) -> impl Stream<Item = MatchEvent> {

        let mc = MatchingContext::new_pinned(Self::IN_CLUSTERS, Self::OUT_CLUSTERS);

        mc.start_matching(&self, src_ep)
    }
}
