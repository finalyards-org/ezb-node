
use std::sync::atomic::{AtomicU8, Ordering};

use embassy_sync::channel::{
    DynamicReceiver
};
use embassy_time::{
    Duration
};
use futures_util::{
    pin_mut,
    StreamExt
};

use ezb_node::{
    Accessor,
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

use crate::button_task::{
    receive as btn_receive,
    ButtonEvent
};

const ONE_SEC: Duration = Duration::from_millis(1000);

// #hack: How does this go: 'Config' has many endpoints; we need one for the Zigbee comms...
const SRC_EP: u8 = 1;

pub(crate) struct LightSwitchRouter
where Self: Node {
    src_ep: u8,  // #later this will not be needed once Accessors are based on endpoint
    count: AtomicU8,      // internally mutable state for steering the remote light
}

impl Node for LightSwitchRouter {}

impl MatchColorDimmableLights for LightSwitchRouter {}

impl LightSwitchRouter {
    pub fn new(cfg: &'static Config) -> Result<Self, ezb_node::Error> {
        const AUTO_START: bool = false;
        // Note: This might disappear, see comment of 'Node::init()'.

        let () = <Self as Node>::init(cfg, AUTO_START)?;

        Ok(Self { src_ep: SRC_EP, count: AtomicU8::new(0) })
    }

    /**
    * Behaviour of this particular node.
    *
    * @note: The closures provided to 'Node::run' are run in this application task.
    */
    pub async fn run(mut self) -> ! {
        <Self as Node>::run(self,
            |this: &'static Self, sig| Box::pin(this.on_app_signal(sig)),
            |this: &'static Self, ev| { unimplemented!() }
        ).await
    }

    /**
    * Run the received signal through our logic match.
    */
    async fn on_app_signal(&'static self, sig: AppSignal) {
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
                pin_mut!(match_stream); // pins to stack

                let first_light = loop {
                    match match_stream.next().await {
                        Some(MatchEvent::Bound(light)) => {
                            log::info!("Bound with color dimmable light device: 0x{:04X}:{}", light.get().dst_addr, light.get().dst_ep);
                            break light;    // start using it!
                        },
                        Some(MatchEvent::Error(err)) => {
                            log::error!("Failed during color dimmable light search (from remote device): {}", err);
                            // keep on searching
                        },
                        None => {
                            log::error!("Unexpected end of binding stream!");   // what's this?
                        }
                    }
                };

                // 'match_stream' drops; search phase is over.

                log::info!("Listening to button presses...");
                loop {
                    match btn_receive().await {
                        ButtonEvent::Pressed => {
                            handle_button_press(&first_light, &self.count);
                        },
                        ButtonEvent::Depressed => {}
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

/**
* Steer the remove light, similar to the C example.
*/
// Note: state of 'LightSwitchRouter' needs to be internally mutable, because the structure is passed around as shared pointers.
//
fn handle_button_press(light: &AccessColorDimmableLight, count_ref: &AtomicU8) {
    let count = count_ref.fetch_add(1, Ordering::Relaxed);  // increment, return the previous value

    if count % 2 == 0 {
        // Even -> steer the level
        // (In the C code, levels raise 32 -> 64 -> 96 -> ...)
        let next_level = ((count / 2) % 4 + 1) * 32;
        log::info!("Move the level of HA dimmable light to {}", next_level);

        light.set_level(next_level);
    } else {
        // Odd -> steer color
        // (From C code):
        let color_x = 0x0400 * ((count as u16 / 2) % 2 + 1);
        let color_y = 0x0400 * ((count as u16 / 2) % 2 + 1);

        log::info!("Move the color of HA dimmable light");
        light.set_color_xy(color_x, color_y);
    }
}
