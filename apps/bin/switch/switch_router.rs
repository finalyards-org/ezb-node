use embassy_sync::channel::{
    DynamicReceiver
};
use embassy_time::{
    Duration
};

use ezb_node::{
    AccessColorDimmableLight,
    AppSignal,
    BdbMode,
    BdbStatus,
    Config,
    MatchColorDimmableLights,
    MatchEvent,
    Node,
    ZclEvent,
    ZclError,
};

const ONE_SEC: Duration = Duration::from_millis(1000);

pub(crate) struct LightSwitchRouter
where Self: Node {
    src_ep: u8  // needed for Zigbee
}

impl Node for LightSwitchRouter {}

impl MatchColorDimmableLights for LightSwitchRouter {}

impl LightSwitchRouter {
    pub fn new(c: &'static Config) -> Result<Self, ezb_node::Error> {
        const AUTO_START: bool = false;
        // Note: This might disappear, see comment of 'Node::init()'.

        let () = <Self as Node>::init(c, AUTO_START)?;

        // #hack: How does this go: 'Config' has many endpoints; we need one for the Zigbee comms...
        const SRC_EP: u8 = 1;

        Ok(Self { src_ep: SRC_EP })
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

                let match_stream = self.start_matching_color_dimmable_lights(self.src_ep);

                while let Some(ev) = match_stream.recv().await {
                    match ev {
                        MatchEvent::Bound(light) => {
                            let short_addr = light.get().short_addr;
                            let ep_id = light.get().dst_ep_id;
                            log::info!("Bound with color dimmable light device: 0x{:04X}:{}", short_addr, ep_id);

                            // Use it
                            light.set_level(128) .await;   // tbd. use in the same way as C example

                            // Keeps on searching....
                            // tbd. C sample only takes the first, and runs with it
                            todo!()
                        },
                        MatchEvent::Error(err) => {
                            log::error!("Failed to bind color dimmable light device: {}", err);
                            break;
                        }
                    }
                }
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
