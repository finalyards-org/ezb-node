use ezb_node_raw::{
    ezb_zcl_cluster_id_e,
    ezb_zcl_cluster_id_t,
};

// For now, enough to pass the values we actually are using in applications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::FromRepr, strum::Display)]
#[repr(u16)]    // we know this by the C library
pub enum ClusterId {
    Basic = ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_BASIC as _,
    PowerConfig = ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_POWER_CONFIG as _,
    //...
    Diagnostics = ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_DIAGNOSTICS as _,
    #[cfg(feature = "touchlink")]
    TouchlinkCommissioning = ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_TOUCHLINK_COMMISSIONING,
}

impl ClusterId {
    pub(crate) fn parse(v: ezb_zcl_cluster_id_t /*u16*/) -> Option<Self> {

        // We have two ranges of checks:
        //  - 'esp_zigbee_lib': check against completely unknown values
        //  - us: check against cluster id's 'ezb_node' does not yet support
        //
        ClusterId::from_repr(v).or_else(|| {
            if ezb_zcl_cluster_id_e::parse(v).is_some() {
                log::error!("ZCL cluster ID not supported (please add it): {}", v);
            } else {
                log::error!("Invalid ZCL cluster ID (not recognized by esp_zigbee_lib): {}", v);
            }
            None
        })
    }
}
