
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
    NodeApi,
    ZclEvent,
    ZclError,
};
use ezb_node::config::DeviceType;

#[path = ".conf.rs"]
mod cfg;

use crate::button_task::{
    receive as btn_receive,
    ButtonEvent
};

const ONE_SEC: Duration = Duration::from_millis(1000);

pub struct SwitchRouter {
    //src_ep: u8,  // The endpoint number associated with 'HA::color_dimmer_switch'
    count: u8,
}

impl SwitchRouter {
    pub fn new() -> Result<Self, ezb_node::Error> {
        let c = cfg::get();
        let () = <Self as Node>::init(c)?;

        // tbd. eventually, (we could) make the 'config' generate a const entry (not using 'BTreeMap', for example).
        //      The constness allows us to read the endpoint index corresponding to a certain device type,
        //      into a const. Then feed it to the 'impl MatchColorDimmableLights', below.
        //
        // Discover the index of 'HA::color_dimmer_switch' end point
        //
        #[cfg(false)]
        let src_ep = c.endpoints .iter()
            .find_map(|(&k,&v)| {
                if v == DeviceType::ColorDimmerSwitch {
                    Some(k)
                } else {
                    None
                }
            })
            .unwrap("My own endpoint is missing!");

        Ok(Self { count: 0 /*, src_ep*/ })
    }
}

impl Node for SwitchRouter {
    /**
    * Run the received signal through our logic match.
    */
    async fn on_app_signal(&mut self, sig: AppSignal) {
        use AppSignal::*;

        // Used so much it warrants us to make it a closure.
        let enter_commission_mode = |mode: BdbMode| {
            self.start_top_level_commissioning(mode)
                .expect("Unable to commit the top level commissioning state");  // tbd. does this occur?
        };

        match sig {
            ZdoSignalSkipStartup => {
                log::info!("Initialize Zigbee stack");
                enter_commission_mode(BdbMode::Initialization);
            },

            BdbSignalDeviceFirstStart(BdbStatus::Success) | BdbSignalDeviceReboot(BdbStatus::Success) => {
                // Could do delayed hw initialization, here

                let is_new = self.is_factory_new();
                log::info!("Device started up in {} mode.", if is_new {"factory-reset"} else {"commissioned"});
                if is_new {
                    enter_commission_mode(BdbMode::NetworkSteering);   // router: steering (coordinator: formation)!!!
                } else {
                    log::info!("Device reboot");
                }
            },
            BdbSignalDeviceFirstStart(st) | BdbSignalDeviceReboot(st) => {
                log::warn!("{} failed with status = {}; retrying...", sig, st);

                embassy_time::Timer::after_secs(1).await;
                enter_commission_mode(BdbMode::Initialization);
            },

            // Note: 'Steering' for router ('Formation' for coordinator)!
            BdbSignalSteering(BdbStatus::Success) => {
                log::info!("Joined network successfully: Extended PAN ID: {}, PAN ID: {}, Channel:{}, Short Address: {:#06x}",
                    self.get_extended_panid(),
                    self.get_panid(),
                    self.get_current_channel(),
                    self.get_short_address()
                );

                // #HACK. Once we have the 'Config' as 'const', this can be deduced.
                const SRC_EP: u8 = 1;   // MUST MATCH what's in TOML

                // Searching the remote counterpart.
                let first_light: AccessColorDimmableLight<_> = {
                    let matcher = Matcher<AccessColorDimmableLight::<SRC_EP>>::new();

                    let match_stream = matcher.start();
                    pin_mut!(match_stream); // pins to stack

                    loop {
                        match match_stream.next().await {
                            Some(MatchEvent::Found(light)) => {
                                log::info!("Found color dimmable light device: 0x{:04X}:{}", light.dst_addr, light.dst_ep);
                                break light;    // start using it
                            },
                            Some(MatchEvent::Error(err)) => {
                                log::error!("Failed during color dimmable light search (from remote device 0x{:04X}): {}", xxx_dst_addr, err);
                                // keep on searching
                            },
                            None => {
                                log::error!("Unexpected end of binding stream!");
                                    // tbd. does this simply mean timeout?
                            }
                        }
                    }
                    // 'match_stream' drops; search phase is over.
                };

                {
                    log::info!("Binding with {}:{}...", first_light.addr, first_light.ep);

                    // 'first_light.bind() .await' or something else
                    todo!()
                    //...

                }

                // tbd. move to application task???  ('first_light' as a member?)
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
            // Note: 'Steering' for router ('Formation' for coordinator)
            BdbSignalSteering(st) => {
                log::info!("Failed to join network: {}, st = {}", sig, st);  // differs, router vs. coordinator

                embassy_time::Timer::after_secs(1).await;
                enter_commission_mode(BdbMode::NetworkSteering);    // router: Steering (coordinator: Formation)
            },

            ZdoSignalLeave { leave_type_X } => {
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

    fn on_zcl_event(&mut self, zcl_event: ZclEvent) {
        todo!()
    }
} //

/**
* Steer the remote light, similar to the C example.
*/
fn handle_button_press(light: &AccessColorDimmableLight, count_ref: &mut u8) {
    let count = *count_ref;
    *count_ref += 1;

    if count % 2 == 0 {
        // Even -> steer the level
        // (In the C code, levels raise 32 -> 64 -> 96 -> ...)
        let next_level = ((count / 2) % 4 + 1) * 32;
        log::info!("Set the level of HA dimmable light to: {}", next_level);

        light.set_level(next_level);
    } else {
        // Odd -> steer color
        // (From C code):
        let color_x = 0x0400 * ((count as u16 / 2) % 2 + 1);
        let color_y = 0x0400 * ((count as u16 / 2) % 2 + 1);
        log::info!("Set the color of HA dimmable light: {}, {}", color_x, color_y);

        light.set_color_xy(color_x, color_y);
    }
}
