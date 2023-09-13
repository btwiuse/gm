//! contract state

use crate::*;

/// contract state is populated during initialization
pub static mut STATE: Option<Contract<GearConfig>> = None;

#[no_mangle]
extern "C" fn state() {
    let query: Query = gstd::msg::load().expect("failed to decode input argument");
    let state = unsafe { STATE.as_ref().expect("failed to get contract state") };
    let reply = match query {
        Query::Name => State::Name(state.name()),
        Query::Symbol => State::Symbol(state.symbol()),
        Query::BaseUri => State::BaseUri(state.base_uri.clone()),
        Query::TokenMetadata(token) => {
            let metadata = state.get_token_metadata(token);
            State::TokenMetadata(metadata)
        }
        Query::IsApprovedForAll { owner, operator } => {
            let approved = state.is_approved_for_all(owner, operator);
            State::IsApprovedForAll(approved)
        }
        Query::BalanceOf(who, token) => {
            let balance = state.balance_of(who, token);
            State::BalanceOf(balance)
        }
        Query::BalanceOfBatch(who, token) => {
            let balance = state.balance_of_batch(who, token);
            State::BalanceOfBatch(balance)
        }
    };
    gstd::msg::reply(reply, 0).expect("Failed to share state");
}
