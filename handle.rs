//! contract transaction handlers

use crate::*;

#[no_mangle]
unsafe extern "C" fn handle() {
    let id: ActorId = msg::source();
    debug!("id: {:?}", id);
    let state = STATE.as_mut().expect("Could not get state");
    let input: Input = msg::load().expect("Could not load msg");
    match input {
        Input::TransferFrom {
            from,
            to,
            token,
            amount,
        } => {
            state.safe_transfer_from(from, to, token, amount);
            state.emit_transfer_single_event(id, from, to, token, amount);
        }
        Input::BatchTransferFrom {
            from,
            to,
            token,
            amount,
        } => {
            state.safe_batch_transfer_from(from, to, token.clone(), amount.clone());
            state.emit_transfer_batch_event(id, from, to, token, amount);
        }
        Input::SetApprovalForAll { operator, approved } => {
            state.set_approval_for_all(id, operator, approved);
            state.emit_approval_for_all_event(id, operator, approved);
        }
        Input::Mint { to, token, amount } => {
            state.mint(to, token, amount);
            state.emit_transfer_single_event(id, ActorId::zero(), to, token, amount);
        }
        Input::MintBatch { to, token, amount } => {
            state.mint_batch(to, token.clone(), amount.clone());
            state.emit_transfer_batch_event(id, ActorId::zero(), to, token, amount);
        }
        Input::Burn {
            from,
            token,
            amount,
        } => {
            state.burn(from, token, amount);
            state.emit_transfer_single_event(id, from, ActorId::zero(), token, amount);
        }
        Input::BurnBatch {
            from,
            token,
            amount,
        } => {
            state.burn_batch(from, token.clone(), amount.clone());
            state.emit_transfer_batch_event(id, from, ActorId::zero(), token, amount);
        }
        Input::UpdateTokenMetadata { token, metadata } => {
            state.update_token_metadata(token, metadata);
        }
    }
}
