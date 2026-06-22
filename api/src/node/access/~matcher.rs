use ezb_node_raw::ezb_zcl_cluster_id_e;

use crate::{
    Node
};

use super::{
    Access,
};

/*
* Extension of 'Node' that allows matching for a certain kinds of other nodes.
*/
pub(super) trait Matcher<T: Access>: Node<'static> {
    const IN_CLUSTERS: &[ezb_zcl_cluster_id_e];
    const OUT_CLUSTERS: &[ezb_zcl_cluster_id_e];

    fn start_matching
}
