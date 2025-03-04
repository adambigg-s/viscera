use std::ffi::c_void;

use sokol::app as sap;
use sokol::gfx;

use crate::State;

pub extern "C" fn ffi_cb_init(user_data: *mut c_void) {
    let state: &mut State;
    unsafe {
        state = &mut *(user_data as *mut State);
    }

    state.callback_init(user_data);
}

pub extern "C" fn ffi_cb_event(raw_event: *const sap::Event, user_data: *mut c_void) {
    let event: &sap::Event;
    let state: &mut State;
    unsafe {
        event = &*raw_event;
        state = &mut *(user_data as *mut State);
    }

    state.callback_event(event);
}

pub extern "C" fn ffi_cb_frame(user_data: *mut c_void) {
    let state: &mut State;
    unsafe {
        state = &mut *(user_data as *mut State);
    }

    state.callback_frame();
}

#[allow(unused_must_use)]
#[allow(clippy::from_raw_with_void_ptr)]
pub extern "C" fn ffi_cb_cleanup(user_data: *mut c_void) {
    gfx::shutdown();
    unsafe {
        if !user_data.is_null() {
            Box::from_raw(user_data);
        }
    }
}
