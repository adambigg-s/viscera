use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;

use crate::callback_event;
use crate::callback_frame;
use crate::callback_init;
use crate::state::State;

pub extern "C" fn ffi_cb_init(user_data: *mut c_void) {
    let state: &mut State;
    unsafe {
        state = &mut *(user_data as *mut State);
    }

    callback_init(user_data, state);
}

pub extern "C" fn ffi_cb_event(raw_event: *const sap::Event, user_data: *mut c_void) {
    let event: &sap::Event;
    let state: &mut State;
    unsafe {
        event = &*raw_event;
        state = &mut *(user_data as *mut State);
    }

    callback_event(event, state);
}

pub extern "C" fn ffi_cb_frame(user_data: *mut c_void) {
    let state: &mut State;
    unsafe {
        state = &mut *(user_data as *mut State);
    }

    callback_frame(state);
}

#[allow(unused_must_use)]
#[allow(clippy::from_raw_with_void_ptr)]
pub extern "C" fn ffi_cb_cleanup(user_data: *mut c_void) {
    gfx::shutdown();
    unsafe {
        Box::from_raw(user_data);
    }
}
