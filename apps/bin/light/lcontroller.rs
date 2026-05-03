/*
*
*/
use std::collections::BTreeMap;

use esp_zb::{
    node::{Node, Router, ChannelMask, NodeConfig},
    IsOpenedForSecs,
    Signal,
    utils::PascalString,
};
use esp_zb::node::{BasicConfig, CommissioningModesMask, EndpointConfig, EndpointType, ColorDimmableLightConfig};

use embassy_time::{Duration};

use crate::{AppError, scheduler::schedule};

const ONE_SEC: Duration = Duration::from_millis(1000);

// Note: These vanish to 'TOML' once done.
const CHANNEL_MASK: ChannelMask = ChannelMask::PRIMARY_CHANNELS;
const MANUFACTURER_NAME: String = "Your name".into();
const MODEL_IDENTIFIER: String = "Your model".into();

pub(crate) struct LightController where Self: Router {  // tbd. !!! make 'Controller'; match 2.0

}

//? const COLOR_DIMMABLE_LIGHT_ENDPOINT: u8 = 10;

impl LightController {
    pub(crate) fn new() -> Result<Self, AppError> {

        // tbd. Most/all things inside here could be gathered to TOML
        //
        //  <<
        //      [network]
        //      channel_mask: [11,15,20,25],    // primary channels
        //
        //      [node]
        //      max_children: 10,
        //
        //      # default for endpoints
        //      manufacturer_name: ...
        //      model_identifier: ...
        //
        //      [node.endpoints.10]
        //      type: "COLOR_DIMMABLE_LIGHT"
        //
        //  <<
        //
        let ep_10 = EndpointConfig(
            EndpointType::ColorDimmableLight( ColorDimmableLightConfig::default() ),
            BasicConfig {
                manufacturer_name: PascalString::from(MANUFACTURER_NAME),
                model_identifier: PascalString::from(MODEL_IDENTIFIER),
            }
        );

        let cfg: NodeConfig = {
            NodeConfig {
                channel_mask: Some(CHANNEL_MASK),
                max_children: None,     // use default
                endpoints: {
                    let mut m = BTreeMap::new();
                        //
                        m.insert(10, ep_10);
                    m
                }
            }
        };

        <Self as Router>::init(cfg)?;
        let me = Self{};

        Ok(me)
    }
}

impl Router for LightController {
    type Error = AppError;

    fn on_app_signal(&self, sig: Signal) /*? -> Result<(), AppError>*/ {
        on_app_signal(self, sig)
    }
}

impl Node for LightController {}

/**
*/
fn on_app_signal(rtr: &LightController, sig: Signal) /*? -> Result<(), AppError>*/ {
    use Signal::*;

    match sig {
        ZdoSignalSkipStartup => {
            log::info!("Initialize Zigbee stack");

            rtr.start_top_level_commissioning(CommissioningModesMask::empty()); // BDB_MODE_INITIALIZATION
        },

        BdbSignalDeviceFirstStart{ success: true } | BdbSignalDeviceReboot{ success: true } => {
            // Could do delayed hw initialization, here

            let is_virgin = rtr.is_factory_new();
            log::info!("Device started up in {} mode.", if is_virgin {"factory-reset"} else {"commissioned"});
            if is_virgin {
                log::info!("Start network steering");
                rtr.start_top_level_commissioning(CommissioningModesMask::NETWORK_STEERING);
            } else {
                log::info!("Device rebooted");
            }
        },
        BdbSignalDeviceFirstStart{ success: false } | BdbSignalDeviceReboot{ success: false } => {
            log::warn!("{} failed, retrying", sig);

            schedule( ONE_SEC, |node| {
                node.start_top_level_commissioning(CommissioningModesMask::empty());   // BDB_MODE_INITIALIZATION
            });
        },

        BdbSignalSteering{ success: true } => {
            log::info!("Joined network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                rtr.get_extended_pan_id(), rtr.get_pan_id(), rtr.get_current_channel(), rtr.get_short_address());
        },
        BdbSignalSteering{ success: false } => {
            log::info!("Network steering was not successful: {}", sig);
            schedule( ONE_SEC, |node| {
                node.start_top_level_commissioning(CommissioningModesMask::NETWORK_STEERING);
            });
        }

        NwkSignalPermitJoinStatus{ isOpened } => {
            let pan_id = rtr.get_pan_id();

            match isOpened {
                Some(IsOpenedForSecs(secs)) =>
                    log::info!("Network{} is open for {} seconds", pan_id, secs),
                None =>
                    log::info!("Network{} closed, devices joining not allowed.", pan_id)
            };
        },
        _ => {
            log::debug!("ZDO signal: {sig}");
        },
    }
    //Ok(())
}
