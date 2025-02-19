use crate::codec::Query;
use crate::codec::State;
use crate::state::STATE;
use gstd::{msg, prelude::*, ActorId};

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: Query = msg::load().expect("failed to decode input argument");
    let id: ActorId = msg::source();
    // PAYLOADS.push(input);
    let id = STATE.last().unwrap_or(&id);
    let encoded = match query {
        Query::Last => State::AccountId(*id).encode(),
        Query::All => State::All(STATE.clone()).encode(),
    };
    gstd::util::to_leak_ptr(encoded)
}
