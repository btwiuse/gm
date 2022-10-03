//! contract state query

use crate::*;

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: Query = msg::load().expect("failed to decode input argument");
    let id: ActorId = msg::source();
    // PAYLOADS.push(input);
    // let id = STATE.last().unwrap_or(&id);
    let state = STATE.as_ref().unwrap();
    let encoded = match query {
        /*
        Query::Last => State::AccountId(*id).encode(),
        Query::All => State::All(STATE.clone()).encode(),
        */
        Query::Name => State::Name(state.name.clone()),
        Query::Symbol => State::Symbol(state.symbol.clone()),
        Query::BaseUri => State::BaseUri(state.base_uri.clone()),
        Query::TokenMetadata(token) => State::TokenMetadata(0, None),
        Query::IsApprovedForAll { who, operator } => State::IsApprovedForAll(true),
        Query::BalanceOf(who, token) => State::BalanceOf(who, token.clone(), token),
        Query::BalanceOfBatch(who, token) => State::BalanceOfBatch(who, token.clone(), token),
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}
