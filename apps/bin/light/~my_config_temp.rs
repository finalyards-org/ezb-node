/**
* Presents the Zigbee node configuration that _eventually_ would come from a TOML file.
*
* For now, we provide a generator for it.
*/
use alloc::collections::BTreeMap;

use esp_zb::{
    node::ChannelMask,
    Config,
    EndpointConfig,
    NodeType,
    BaseConfig,
};

// Note: Eventually this will get read from a TOML file.
//
pub(crate) fn my_config() -> Config {

    let channel_masks = [ChannelMask::from([13]), ChannelMask::ALL]; // same as in C example

    let storage_partition_name= "zb_storage";

    let install_code_policy = false;
    let max_children = 10;

    let ep_10 = EndpointConfig::ColorDimmableLightEPC {};

    let bc = BaseConfig{
        manufacturer_name: "Your name".into(),
        model_identifier: "Your model".into(),
    };

    Config {
        channel_masks,
        storage_partition_name,

        node: NodeType::CoordinatorConfig{
            install_code_policy,
            max_children
        },

        endpoints: BTreeMap::from([(10, (ep_10, bc))])
    }
}
