//! contract state

use crate::*;

/// contract state is populated during initialization
pub static mut STATE: Option<Contract<GearConfig>> = None;

#[no_mangle]
extern "C" fn state() {
    let state = unsafe { STATE.as_ref().expect("failed to get contract state") };
    gstd::msg::reply(state.clone(), 0).expect("Failed to share state");
}
