//! contract transaction handlers

use crate::*;

#[no_mangle]
unsafe extern "C" fn handle() {
    let id: ActorId = msg::source();
    debug!("id: {:?}", id);
    // msg::reply_bytes(Output::Payload(id).encode(), 0).expect("Failed to reply");
    // STATE.push(id)
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
            state.set_approval_for_all(operator, approved);
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
        Input::UpdateTokenMetadata {
            token,
            metadata,
        } => {
            // state.update_token_metadata()
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use gtest::{Program, System};

    #[test]
    fn it_works() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "Let's start");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        assert!(res.log().len() == 1);
        let addr = res.log()[0].payload();
        assert!(addr.starts_with(&[42u8]));
        assert!(addr.ends_with(&[0u8; 31]));
        let who = ActorId::from_slice(addr);
        assert!(who.is_ok());

        let res = program.send_bytes(69, "Gear");
        assert!(res.log().len() == 1);
        let addr = res.log()[0].payload();
        assert!(addr.starts_with(&[69u8]));
        assert!(addr.ends_with(&[0u8; 31]));
        let who = ActorId::from_slice(addr);
        assert!(who.is_ok());
    }
}
