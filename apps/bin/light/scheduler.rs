/*
* Application level scheduler.
*
* Design criteria:
*   - callback needs to happen in the _same FreeRTOS_ thread, as the scheduling
*       - this rules out using FreeRTOS timers themselves,
*         leaving:
*           - 'esp_zb_scheduler_alarm()' (by 'esp-zigbee-lib'); it calls in the same RTOS thread
*           - our own check of scheduled tasks, within the Zigbee iteration loop.
*/

/* tbd. EDIT/REVIEW; KEEP ALTERNATIVES and pros/cons?
* Bind the C 'esp_zb_scheduler_alarm' system to Rust
*
* Design:
*   'esp-zigbee-lib' has 'esp_zb_scheduler_alarm()' which _runs the C callback in the same RTOS thread_ as the Zigbee stack.
*   We wish (need) this. (rules out direct FreeRTOS timers, since those would wake up in a separate timer thread)
*
*   'esp-zigblee-lib' scheduling takes only a singe, 'u8' parameter. This restricts our simultaneously possible timers
*   to 255-ish, but that's fine. We map an integer to the wider set of params.
*
*   Note: The higher levels attach a 'node' parameter to the calls. There are no problems because there's EVER ONLY ONE
*       'Node' in the system, with a static lifespan.
*
*   Alternatives:
*       - could craft our own scheduling list as part of the '.roll()' looping
*           pros:
*               + allows us to be more Rust-centric (no mapping of callback to 'u8')
*           cons:
*               - more work
*/

use std::collections::BinaryHeap;
//r? use embassy_time::{Duration, Instant, Timer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use core::cmp::Ordering;
use embassy_futures::select::{select, Either};

use crate::LightController;

// Channel to send tasks over
static SCHED_CHAN: Channel<CriticalSectionRawMutex, ScheduledTask, 4> = Channel::new();

type NodeT = LightController;

/*
* Something the application wants to happen, in the future.
*/
#[derive(Debug)]
struct ScheduledTask {
    time: Instant,
    #[allow(dead_code)] // without which, Rust (1.97) could warn about the field not being used. #weird
    job: fn(&'static NodeT),
}

// 'BinaryHeap' is 'Max-Heap' so reversing the order (nearest time first)
impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool { self.time == other.time }
}
impl Eq for ScheduledTask {}

/*
* Scheduling worker task
*
* Note: Here we are "stuck" with the application level type - or we could do these more generically and only allow
*       access to certain Node/Router/Controller methods, in the callback.
*       i.e. cannot use generics in an Embassy task definition.
*/
#[embassy_executor::task]
pub async fn scheduler_worker(lr: &'static NodeT) {
    let mut store: BinaryHeap<ScheduledTask> = BinaryHeap::new();

    loop {
        let next_timeout = store.peek().map(|t| t.time);

        let res = match next_timeout {
            None => Either::First(SCHED_CHAN.receive().await),
            Some(time) => select(
                SCHED_CHAN.receive(),   // this gives 'Either::First'
                Timer::at(time)         // ..gives 'Either::Second'
            ).await,
        };

        match res {
            // New job -> add to the ordered store
            Either::First(task) => {
                store.push(task);
            }
            // Time is up -> pick next task
            Either::Second(_) => {
                let task = store.pop().expect("store should have stuff");

                assert!(task.time <= Instant::now(), "task picked too soon! {} > {}", task.time, Instant::now());
                (task.job)(lr);
            }
        }
    }
}

pub fn schedule(after: Duration, job: fn(&'static NodeT)) {
    let task = ScheduledTask {
        time: Instant::now() + after,
        job,
    };

    let _ = SCHED_CHAN.try_send(task).map_err(|e| {
        log::error!("Writing to channel failed: {:?}", e);
    });
}
