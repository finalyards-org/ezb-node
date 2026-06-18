use embassy_sync::channel::{
    DynamicReceiver
};
use embassy_time::{
    Duration
};

use ezb_node::{
    AppSignal,
    BdbMode,
    BdbStatus,
    Config,
    Node,
    ZclEvent,
    ZclError,
};

//? use crate::{scheduler::schedule};

const ONE_SEC: Duration = Duration::from_millis(1000);

pub(crate) struct LightSwitchRouter
where Self: Node {
    // can have internally-mutable state here
}

impl Node for LightSwitchRouter {}

impl LightSwitchRouter {
    pub fn new(c: &'static Config) -> Result<Self, ezb_node::Error> {
        const AUTO_START: bool = false;
        // Note: This might disappear, see comment of 'Node::init()'.

        let () = <Self as Node>::init(c, AUTO_START)?;
        Ok(Self {})
    }

    /**
    * Behaviour of this particular node.
    *
    * @note: The closures provided to 'Node::run' are run in this application task.
    */
    pub async fn run(mut self) -> ! {
        <Self as Node>::run(self,
  |this, sig| Box::pin(this.on_app_signal(sig)),
  |this, ev| { unimplemented!() }
        ).await
    }

    /**
    * Run the received signal through our logic match.
    */
    async fn on_app_signal(&self, sig: AppSignal) {
        use AppSignal::*;

        match sig {
            ZdoSignalSkipStartup => {
                log::info!("Initialize Zigbee stack");
                self.start_top_level_commissioning(BdbMode::Initialization);
            },

            BdbSignalDeviceFirstStart(BdbStatus::Success) | BdbSignalDeviceReboot(BdbStatus::Success) => {
                // Could do delayed hw initialization, here

                let is_new = self.is_factory_new();
                log::info!("Device started up in {} mode.", if is_new {"factory-reset"} else {"commissioned"});
                if is_new {
                    self.start_top_level_commissioning(BdbMode::NetworkSteering);   // router: steering
                } else {
                    log::info!("Device rebooted");
                }
            },
            BdbSignalDeviceFirstStart(st) | BdbSignalDeviceReboot(st) => {
                log::warn!("{} failed with status = {}; retrying...", sig, st);

                embassy_time::Timer::after_secs(1).await;
                self.start_top_level_commissioning(BdbMode::Initialization);
            },

            BdbSignalSteering(BdbStatus::Success) => {
                log::info!("Joined network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                    self.get_extended_panid(),
                    self.get_panid(),
                    self.get_current_channel(),
                    self.get_short_address()
                );
                zdo_find_ha_color_dimmable_light_device();
            },
            BdbSignalSteering(st) => {
                log::info!("Failed to join network: {}, st = {}", sig, st);

                embassy_time::Timer::after_secs(1).await;
                self.start_top_level_commissioning(BdbMode::NetworkSteering);
            },

            ZdoSignalLeave { leave_type_X } => {    // tbd. make a proper enum (drop '_X')
                log::info!("Left network successfully with type: {}", leave_type_X);
            },

            NwkSignalPermitJoinStatus { is_opened_for } => {
                let pan_id = self.get_panid();
                match is_opened_for {
                    Some(dur) =>
                        log::info!("Network {} is open for {} seconds", pan_id, dur.as_secs()),
                    None =>
                        log::info!("Network {} closed, devices joining not allowed.", pan_id)
                };
            },

            _ => {
                log::info!("Zigbee APP signal: {sig}");
            },
        }
    }
}


//static ezb_err_t zdo_find_ha_color_dimmable_light_device(void)
// {
//     ezb_err_t ret            = EZB_ERR_FAIL;
//     uint16_t  cluster_list[] = {EZB_ZCL_CLUSTER_ID_ON_OFF, EZB_ZCL_CLUSTER_ID_LEVEL, EZB_ZCL_CLUSTER_ID_COLOR_CONTROL};
//
//     ezb_zdo_match_desc_req_t req = {
//         .dst_nwk_addr = 0xFFFD,
//         .field =
//             {
//                 .nwk_addr_of_interest = 0xFFFD,
//                 .profile_id           = EZB_AF_HA_PROFILE_ID,
//                 .num_in_clusters      = sizeof(cluster_list) / sizeof(cluster_list[0]),
//                 .num_out_clusters     = 0,
//                 .cluster_list         = cluster_list,
//             },
//         .cb       = zdo_find_ha_color_dimmable_light_device_result,
//         .user_ctx = NULL,
//     };
//     ret = ezb_zdo_match_desc_req(&req);
//     if (ret == EZB_ERR_NONE) {
//         ESP_LOGI(TAG, "Attempt to find HA color dimmable light device");
//     } else {
//         ESP_LOGE(TAG, "Failed to find HA color dimmable light device with error(0x%04x)", ret);
//     }
//     return ret;
// }
fn zdo_find_ha_color_dimmable_light_device() -> Option<ZclError> {

    let req = {
        //         .dst_nwk_addr = 0xFFFD,
        //         .field =
        //             {
        //                 .nwk_addr_of_interest = 0xFFFD,
        //                 .profile_id           = EZB_AF_HA_PROFILE_ID,
        //                 .num_in_clusters      = sizeof(cluster_list) / sizeof(cluster_list[0]),
        //                 .num_out_clusters     = 0,
        //                 .cluster_list         = cluster_list,
        //             },
        //         .cb       = zdo_find_ha_color_dimmable_light_device_result,      // HA, callback!! we're in trouble here, are we???
        //         .user_ctx = NULL,
    };

    let ret = match ezb_zdo_match_desc_req(&req);
    match ret {
        None => {
            log::info!("Attempt to find HA color dimmable light device");
        },
        Some(err) => {
            log::error!("Failed to find HA color dimmable light device: {}", err);
        }
    }
    ret
}
