use ezb_node_config::{
    ChannelMask,
    Config,
    NodeType::*,
};

#[cfg(feature = "coordinator")]
use ezb_node_config::PreConfiguredDevice;

use ezb_node_raw::{
    esp_zigbee_config_t,
    esp_zigbee_device_config_t,
    esp_zigbee_platform_config_t,
    ezb_bdb_set_primary_channel_set,
    ezb_bdb_set_secondary_channel_set,
    ezb_secur_ic_type_e::EZB_SECUR_IC_TYPE_128,
    esp_zigbee_init,
    ezb_secur_ic_add,
    ezb_secur_ic_type_e,
    ezb_aps_secur_enable_distributed_security,
};

use crate::{
    EzbError,
    EspError,
    EzbEspError,
};

/**
* An object to bridge the TOML sections '[network]' and '[node]' with 'esp_zigbee_lib' APIs.
*/
pub(crate) struct ConfigAccess<const PARTITION_NAME: &'static str = "nvs">{
    /// Data structure needed for 'ezb_init()'
    c_config: esp_zigbee_config_t,
    channel_masks: [ChannelMask; 2],

    #[cfg(feature = "coordinator")]
    pre_configurers: Vec<PreConfigurer>,
}

impl<const PARTITION_NAME: &'static str> ConfigAccess<PARTITION_NAME> {

    /// Form a *data structure* that leans towards the 'esp_zigbee_lib' usage, but *does not run
    /// any initializations*, yet.
    ///
    fn new(cfg: &'static Config) -> Self {
        let Config {
            primary_channels,
            secondary_channels,
            node,
            .. } = cfg;

        #[cfg(feature = "coordinator")]
        let pre_configured_devices: &'static _;     // to be picked up, just for coordinator

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
        /// The 'dev_cfg_t' is a C-level configuration structure, but still just that - a data structure.
        ///
        let dev_cfg_t = match node {
            // '.pre_configured_devices' is taken later (stage 2)
            #[cfg(feature = "coordinator")]
            CoordinatorConfig { max_children, pre_configured_devices: pcds } => {
                pre_configured_devices = pcds;  // put aside
                let max_children = *max_children;

                // true: require actual install codes
                // false: allow known install codes (e.g. 'Zigbee09Alliance')
                //
                let install_code_policy = ! pre_configured_devices.is_empty();

                esp_zigbee_device_config_t::for_coordinator(install_code_policy, max_children)
            },
            #[cfg(feature = "router")]
            RouterConfig { install_code_policy, max_children } => {
                let install_code_policy = *install_code_policy;
                let max_children = *max_children;

                esp_zigbee_device_config_t::for_router(install_code_policy, max_children)
            },
            #[cfg(false)]
            EndDeviceConfig { install_code_policy, max_children } => {
                let install_code_policy = *install_code_policy;
                let max_children = *max_children;

                todo!()
                //esp_zigbee_device_config_t::for_end_device(ed_timeout, keep_alive.as_millis())
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
        /// A data structure, one used by C level.
        ///
        let platform_cfg = {
            // Create a CStr version of the partition name
            let cs = std::ffi::CString::new(PARTITION_NAME).unwrap();
            let leaked_cs: &'static std::ffi::CStr = Box::leak(cs.into_boxed_c_str());

            esp_zigbee_platform_config_t::for_native_mode(leaked_cs)
        };

        /// Data we retain for the later '.init()'
        ///
        let c_config = esp_zigbee_config_t {
            device_config: dev_cfg_t,
            platform_config: platform_cfg,
        };

        Self {
            c_config,
            channel_masks: [*primary_channels, *secondary_channels],

            #[cfg(feature = "coordinator")]
            pre_configurers: pre_configured_devices.iter().map(|entry: &'static _| {
                PreConfigurer::from(entry)
            }).collect()
        }
    }

    /// Initialize the node, in C library.
    ///
    //ESP_ERROR_CHECK(esp_zigbee_init(&config));
    //
    // esp_zigbee_setup_commissioning:
    //      ezb_aps_secur_enable_distributed_security(false);
    //      ESP_ERROR_CHECK(ezb_bdb_set_primary_channel_set(ESP_ZIGBEE_PRIMARY_CHANNEL_MASK));
    //      ESP_ERROR_CHECK(ezb_bdb_set_secondary_channel_set(ESP_ZIGBEE_SECONDARY_CHANNEL_MASK));
    //      ESP_ERROR_CHECK(ezb_app_signal_add_handler(esp_zigbee_app_signal_handler));
    //
    pub(crate) fn init(self) -> Result<(), EzbEspError> {
        // Note: Leaking ensures that the C-level data structure remains valid throughout the
        // process lifespan.
        //
        let x = Box::leak(Box::new(self.c_config));

        EzbError::wrap( unsafe {
            esp_zigbee_init(x)
        })?;

        // We only support centralized trust, i.e. the coordinator (not a router) decides, which
        // devices are let to enter.
        {
            let () = unsafe {
                ezb_aps_secur_enable_distributed_security(false)
            };
        }

        // Set the channels
        set_channels(self.channel_masks)?;

        #[cfg(feature = "coordinator")]
        // Init the additional clusters
        {
            for pre_configurer in self.pre_configurers {
                pre_configurer._call()?
            }
        }

        Ok(())
    }
}

// This allows an app to provide a static '&Config' where the needing party only needs a view.
impl<const PN: &'static str> From<&'static Config> for ConfigAccess<PN> {
    fn from(c: &'static Config) -> Self {
        Self::new(c)
    }
}

/**
* Set the channels that the node uses for commissioning.
*
* Coordinator: selects the channel for the network, from these (considering noise levels).
* others:   use these channels to find and join the network
*
* @note The possible failure code is 'EZB_ERR_INVALID_ARG' but the values have been pruned in
*       'config' so we really should not get any errors.
*/
//ESP_ERROR_CHECK(ezb_bdb_set_primary_channel_set(ESP_ZIGBEE_PRIMARY_CHANNEL_MASK));
//ESP_ERROR_CHECK(ezb_bdb_set_secondary_channel_set(ESP_ZIGBEE_SECONDARY_CHANNEL_MASK));
//
fn set_channels(channel_masks: [ChannelMask;2]) -> Result<(), EzbError> {

    log::trace!("ezb_bdb_set_primary_channel_set()");   // TEMP

    EzbError::wrap( unsafe {
        ezb_bdb_set_primary_channel_set(channel_masks[0].bits())
    })?;

    EzbError::wrap( unsafe {
        ezb_bdb_set_secondary_channel_set(channel_masks[1].bits())
    })?;

    Ok(())
}

#[cfg(feature = "coordinator")]
struct PreConfigurer {
    mac_address: &'static [u8;8],     // 64-bit
    install_code: &'static [u8;18],
}

#[cfg(feature = "coordinator")]
impl From<&'static PreConfiguredDevice> for PreConfigurer {
    fn from(o: &'static PreConfiguredDevice) -> Self {
        let PreConfiguredDevice { mac_address, install_code } = o;
        Self { mac_address, install_code }
    }
}

#[cfg(feature = "coordinator")]
impl PreConfigurer {
    const IC_TYPE: ezb_secur_ic_type_e = EZB_SECUR_IC_TYPE_128;

    /// Apply the pre-configured devices (if any); coordinator only.
    ///
    /// This needs to be done after endpoint creation but before the stack is started (source: google.ai).
    ///
    // Rust note: nightly allows making a struct 'impl Fn<(...,)>', but it becomes verbose because also 'FnMut' and 'FnOnce'
    //          need to be defined.
    //
    pub(crate) fn _call(self) -> Result<(),EzbError>{

        // These ptr's are valid forever, and safe to pass to C land; they are based on static stores.
        //
        let mac_address = self.mac_address.as_ptr() .cast();
        let install_code = self.install_code.as_ptr();

        //      #esp_zigbee_lib: It would be great if you document all the cases where pointers are
        //          used longer than just during the call of the function.

        EzbError::wrap( unsafe {
            ezb_secur_ic_add(mac_address, Self::IC_TYPE as u8, install_code)
        })?;

        Ok(())
    }
}
