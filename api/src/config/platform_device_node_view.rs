use super::{
    Config,
    NodeType::*,
};

use crate::raw::{
    esp_zigbee_config_t,
    esp_zigbee_device_config_t,
    esp_zigbee_platform_config_t,
    ezb_nwk_device_type_t::*,
};

use crate::node::{
    ChannelMask
};

/**
* A view to a 'Config' struct used in initializing 'Node'.
*/
pub(crate) struct PlatformDeviceNodeView<'a>(&'a Config);

impl<'a> PlatformDeviceNodeView<'a> {

    pub(crate) fn expand(&self) -> (esp_zigbee_config_t, [ChannelMask;2]) {

        // | TOML     | struct                       |
        // |----------|------------------------------|
        // | network  | channel_masks                |
        // | platform | esp_zigbee_platform_config_t |
        // | node     | esp_zigbee_device_config_t   |
        //
        let nt = self.0.node;
        let channel_masks = self.0.channel_masks;

        //typedef struct esp_zigbee_device_config_s {
        //     ezb_nwk_device_type_t device_type;          /*!< The nwk device type, @ref ezb_nwk_device_type_t */
        //     bool install_code_policy;                   /*!< Allow install code security policy or not */
        //     union {
        //         struct esp_zigbee_zczr_config_s zczr_config; /*!< The Zigbee zc/zr device configuration */
        //         struct esp_zigbee_zed_config_s  zed_config;  /*!< The Zigbee zed device configuration */
        //     };
        // } esp_zigbee_device_config_t;
        //
        //struct esp_zigbee_zczr_config_s {
        //     uint8_t max_children; /*!< Max number of the children */
        // };
        //
        let dev_cfg = match nt {
            #[cfg(feature = "coordinator")]
            CoordinatorConfig { install_code_policy, max_children } => {
                esp_zigbee_device_config_t::for_zczr(EZB_NWK_DEVICE_TYPE_COORDINATOR, install_code_policy, max_children)
            },
            #[cfg(feature = "router")]
            RouterConfig { install_code_policy, max_children } => {
                esp_zigbee_device_config_t::for_zczr(EZB_NWK_DEVICE_TYPE_ROUTER, install_code_policy, max_children)
            },
            #[cfg(false)]
            EndDeviceConfig { } => {
                todo!()
                //esp_zigbee_device_config_t::for_zed(ed_timeout, keep_alive.as_millis())
            }
        };

        //typedef struct esp_zigbee_platform_config_s {
        //     const char *storage_partition_name;     /*!< The name of the storage partition */
        //     esp_zigbee_radio_config_t radio_config; /*!< The radio configuration */
        // } esp_zigbee_platform_config_t;
        //
        //typedef struct esp_zigbee_radio_config_s {
        //     esp_zigbee_radio_mode_t  radio_mode;            /*!< The radio mode */
        //     union {
        //         esp_zigbee_uart_config_t radio_uart_config; /*!< The uart configuration to RCP */
        //     };
        // } esp_zigbee_radio_config_t;
        //
        let storage_partition_name = self.0.storage_partition_name.as_ref();

        let platform_cfg = esp_zigbee_platform_config_t::for_native_mode(
            storage_partition_name
        );

        let cc = esp_zigbee_config_t {
            device_config: dev_cfg,
            platform_config: platform_cfg,
        };

        (cc, channel_masks)
    }
}
