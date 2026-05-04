/**
* Presents the Zigbee node configuration that _eventually_ would come from a TOML file.
*
* For now, we provide a generator for it.
*/
use alloc::collections::BTreeMap;

use esp_zb::{node::ChannelMask, Config, EndpointConfig};

pub(crate) fn my_config() -> Config {

    let ep_10 = EndpointConfig::ColorDimmableLightEPC {

    };

    let c = Config {
        primary_channel_mask: ChannelMask::from([13]),  // as in C sample
        secondary_channel_mask: ChannelMask::ALL,

        storage_partition_name: "zb_storage".into(),

        manufacturer_name: "Your name".into(),
        model_identifier: "Your model".into(),

        endpoints: BTreeMap::from([(10, ep_10)])
    };

    c
}
