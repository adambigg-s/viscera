mod ffis;
mod shaders;
mod state;
mod temp_util_loc;

use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;
use sokol::log;

use ffis::*;
use state::State;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 1200;
#[rustfmt::skip]
const BACKGROUND: gfx::Color = gfx::Color { r: 0.15, g: 0.2, b: 0.3, a: 1. };

fn main() {
    let state = State::new();
    let state = Box::new(state);
    let user_data = Box::into_raw(state) as *mut c_void;

    sap::run(&sap::Desc {
        user_data,
        init_userdata_cb: Some(ffi_cb_init),
        event_userdata_cb: Some(ffi_cb_event),
        frame_userdata_cb: Some(ffi_cb_frame),
        cleanup_userdata_cb: Some(ffi_cb_cleanup),
        width: WIDTH,
        height: HEIGHT,
        fullscreen: false,
        window_title: c"compassion collective".as_ptr(),
        logger: sap::Logger {
            func: Some(log::slog_func),
            user_data,
        },
        icon: sap::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
