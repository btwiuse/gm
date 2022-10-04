//! contract transaction handlers

use crate::*;

#[no_mangle]
unsafe extern "C" fn handle() {
    let sender: ActorId = gstd::msg::source();
    let state = STATE.as_mut().expect("Could not get state");
    let action: Action = gstd::msg::load().expect("Could not load msg");
    match action {
        Action::TransferFrom {
            from,
            to,
            token,
            amount,
        } => {
            state.safe_transfer_from(from, to, token, amount);
            state.emit_transfer_single_event(sender, from, to, token, amount);
        }
        Action::BatchTransferFrom {
            from,
            to,
            token,
            amount,
        } => {
            state.safe_batch_transfer_from(from, to, token.clone(), amount.clone());
            state.emit_transfer_batch_event(sender, from, to, token, amount);
        }
        Action::SetApprovalForAll { operator, approved } => {
            state.set_approval_for_all(sender, operator, approved);
            state.emit_approval_for_all_event(sender, operator, approved);
        }
        Action::Mint { to, token, amount } => {
            state.mint(to, token, amount);
            state.emit_transfer_single_event(sender, ActorId::zero(), to, token, amount);
        }
        Action::MintBatch { to, token, amount } => {
            state.mint_batch(to, token.clone(), amount.clone());
            state.emit_transfer_batch_event(sender, ActorId::zero(), to, token, amount);
        }
        Action::Burn {
            from,
            token,
            amount,
        } => {
            state.burn(from, token, amount);
            state.emit_transfer_single_event(sender, from, ActorId::zero(), token, amount);
        }
        Action::BurnBatch {
            from,
            token,
            amount,
        } => {
            state.burn_batch(from, token.clone(), amount.clone());
            state.emit_transfer_batch_event(sender, from, ActorId::zero(), token, amount);
        }
        Action::UpdateTokenMetadata { token, metadata } => {
            state.update_token_metadata(token, metadata.clone());
            state.emit_update_token_metadata_event(token, metadata);
        }
        Action::Whoami => state.emit_whoami_event(),
    }
}
