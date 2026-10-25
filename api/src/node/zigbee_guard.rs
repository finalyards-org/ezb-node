use std::sync::atomic::{AtomicBool, Ordering};

/// All 'Node' methods are outside of the 'esp_zigbee_lib' callbacks, since they operate in another
/// RTOS thread and received the information via a channel. Thus, they need locks when accessing the
/// 'esp_zigbee_lib' API.
///
#[must_use = "Please store the guard in a variable, e.g. '_guard = ...'"]
struct ZigbeeGuard;

// Just a safety check to see if anyone tries to guard repeatedly.
static GUARDED: AtomicBool = AtomicBool::new(false);

impl ZigbeeGuard {
    fn acquire() -> Self {
        let claimed = GUARDED.swap(true, Ordering::Relaxed);
        assert!(!claimed, "INTERNAL: Guard re-entered"); // if we coded correctly, a double claim won't happen. Application cannot cause it.

        unsafe {
            let got_it = esp_zigbee_lock_acquire(99999);    // tbd. change to a tick count matching, say, 9s

            // Note: 'got_it' being 'false' is technically possible: it means our huge window timed out.
            if !got_it { panic!("Timed out waiting for Zigbee lock") };
        };
        Self
    }
}

impl Drop for ZigbeeGuard {
    fn drop(&mut self) {
        let claimed= GUARDED.swap(false, Ordering::Relaxed);
        assert!(claimed, "INTERNAL: Guard already dropped!");

        unsafe {
            esp_zigbee_lock_release();
        }
    }
}
