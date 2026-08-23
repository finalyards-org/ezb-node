use std::sync::OnceLock;

use ezb_node_config::{
    ChannelMask,
    Config,
    NodeType::*,
};

use ezb_node_raw::{
    esp_zigbee_config_t,
    esp_zigbee_device_config_t,
    esp_zigbee_platform_config_t,
    ezb_nwk_device_type_t::*,
};

static BAKED: OnceLock<esp_zigbee_config_t> = OnceLock::new();
    // 'esp_zigbee_config_t' is the struct itself (not a pointer)

/**
* A view to a 'Config' struct used in initializing 'Node'.
*
* Covers TOML sections '[network]', '[platform]' and '[node]'.
*/
pub(crate) struct ConfigAccess(&'static Config);

impl ConfigAccess {

    pub(crate) fn expand(&self) -> (&'static esp_zigbee_config_t, [ChannelMask;2]) {
        let channels = [self.0.primary_channels, self.0.secondary_channels];
        let storage_partition_name = self.0.storage_partition_name.as_ref();
        let nt = self.0.node;

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
        let platform_cfg = esp_zigbee_platform_config_t::for_native_mode(
            storage_partition_name
        );

        let cc = esp_zigbee_config_t {
            device_config: dev_cfg,
            platform_config: platform_cfg,
        };

        // Since the config is only one (we know this, it's not in the types), we can simply provide the first
        // value ever created. (We can also panic if coming here a second time; should not happen).
        //
        assert!(BAKED.get().is_none(), "Internal: initializing the node twice.");

        let cc = BAKED.get_or_init(|| cc);
        (cc, channels)
    }
}

// This allows an app to provide a static '&Config' where the needing party only needs a view.
impl From<&'static Config> for ConfigAccess {
    fn from(c: &'static Config) -> Self {
        Self(c)
    }
}
