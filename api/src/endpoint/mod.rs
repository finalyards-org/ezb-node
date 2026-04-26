/*
* Endpoint.
*
* 
*/

/*
* A Router, Controller and/or EndDevice.
*
* Node type specific methods and constants are defined each in their own source file.
* General (all nodes) are under 'Node' trait.
*
* Design:
*   The C API implements all these as _global_ functions and constants. We are more specific,
*   which should help e.g. in IDE auto-completion (and generally not using weird stuff when it
*   does not make sense!).
*
* Note:
*   C code supports both native and RCP (radio co-processor) implementations. We only native.
*/
use embassy_time::{Timer, Duration};

use bitflags::bitflags;
use log;

use esp_zb_raw::{esp_zb_cfg_t, esp_zb_init, esp_zb_start, esp_zb_stack_main_loop_iteration, esp_zb_app_signal_t, esp_zb_get_pan_id, esp_zb_get_current_channel, esp_zb_bdb_start_top_level_commissioning, esp_zb_bdb_commissioning_mode_t, esp_zb_get_extended_pan_id, esp_zb_bdb_is_factory_new, esp_zb_get_short_address};

use esp_idf_sys::EspError;

use crate::{Error, IeeeAddr, Signal};

mod router;
pub use crate::node::router::Router;

#[allow(non_upper_case_globals)]
static mut G_nwk_cfg: Option<esp_zb_cfg_t> = None;
//
// note: could use 'OnceLock' but it's 'std'. Also this works.

pub trait Node {
/**
