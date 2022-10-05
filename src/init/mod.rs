//! contract state initialization / constructor

use crate::*;

mod init_test;

#[no_mangle]
/// # Safety
pub unsafe extern "C" fn init() {
    let Init {
        name,
        symbol,
        base_uri,
    } = gstd::msg::load().expect("Invalid init message");
    let id: ActorId = gstd::msg::source();
    STATE = Some(Contract::<GearConfig>::new(&id));
    let state = STATE.as_mut().expect("failed to get contract state");
    state.name = name;
    state.symbol = symbol;
    state.base_uri = base_uri;
    gstd::msg::reply(InitOk, 0).expect("Failed to reply InitOk");
}
