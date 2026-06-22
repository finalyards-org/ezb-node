
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Receiver,
};

use ezb_node_raw::{
    ezb_zcl_cluster_id_e
};

use crate::{
    Node
};

use super::{
    AccessColorDimmableLight,
    GroupcastMatcher,
};

/**
* Extension to node, allowing finding, binding and accessing other nodes with a certain profile.
*/
#[cfg(feature = "match_color_dimmable_light")]
pub trait MatchColorDimmableLights<'a, const N: usize>: Node<'static> {
    // The clusters we demand from the other end
    const IN_CLUSTERS: &[ezb_zcl_cluster_id_e] = &[
        ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_ON_OFF,
        ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_LEVEL,
        ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_COLOR_CONTROL,
    ];
    const OUT_CLUSTERS: &[ezb_zcl_cluster_id_e] = &[];

    fn start_matching_color_dimmable_lights(&self, src_ep: u8) -> Receiver<'a, CriticalSectionRawMutex, AccessColorDimmableLight, N> {

        let m = GroupcastMatcher::new_pinned(Self::IN_CLUSTERS, Self::OUT_CLUSTERS);

        m.start_matching(src_ep, |()| {
            AccessColorDimmableLight::new()
        })
    }
}
