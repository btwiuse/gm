//! contract transaction handlers

use crate::*;

#[no_mangle]
unsafe extern "C" fn handle() {
    let signer: ActorId = gstd::msg::source();
    let state = STATE.as_mut().expect("Could not get state");
    let input: Input = gstd::msg::load().expect("Could not load msg");
    match input {
        Input::TransferFrom {
            from,
            to,
            token,
            amount,
        } => {
            state.safe_transfer_from(from, to, token, amount);
            state.emit_transfer_single_event(signer, from, to, token, amount);
        }
        Input::BatchTransferFrom {
            from,
            to,
            token,
            amount,
        } => {
            state.safe_batch_transfer_from(from, to, token.clone(), amount.clone());
            state.emit_transfer_batch_event(signer, from, to, token, amount);
        }
        Input::SetApprovalForAll { operator, approved } => {
            state.set_approval_for_all(signer, operator, approved);
            state.emit_approval_for_all_event(signer, operator, approved);
        }
        Input::Mint { to, token, amount } => {
            state.mint(to, token, amount);
            state.emit_transfer_single_event(signer, ActorId::zero(), to, token, amount);
        }
        Input::MintBatch { to, token, amount } => {
            state.mint_batch(to, token.clone(), amount.clone());
            state.emit_transfer_batch_event(signer, ActorId::zero(), to, token, amount);
        }
        Input::Burn {
            from,
            token,
            amount,
        } => {
            state.burn(from, token, amount);
            state.emit_transfer_single_event(signer, from, ActorId::zero(), token, amount);
        }
        Input::BurnBatch {
            from,
            token,
            amount,
        } => {
            state.burn_batch(from, token.clone(), amount.clone());
            state.emit_transfer_batch_event(signer, from, ActorId::zero(), token, amount);
        }
        Input::UpdateTokenMetadata { token, metadata } => {
            state.update_token_metadata(token, metadata);
        }
    }
}
