//! contract implementation

use crate::*;
use config::GearConfig;
use io::*;

/// ERC1155GearExt interface
impl IERC1155GearExt for Contract<GearConfig> {
    fn emit_update_token_metadata_event(&self, token: u128, metadata: Option<TokenMetadata>) {
        gstd::msg::reply(Event::UpdateTokenMetadata { token, metadata }, 0)
            .expect("Failed to reply Event::UpdateTokenMetadata");
    }
    fn emit_whoami_event(&self) {
        gstd::msg::reply(
            Event::Whoami {
                sender: self.sender(),
                origin: self.origin(),
            },
            0,
        )
        .expect("Failed to reply Event::Whoami");
    }
    fn emit_transfer_single_event(
        &self,
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        token: u128,
        amount: u128,
    ) {
        gstd::msg::reply(
            Event::TransferSingle {
                operator,
                from,
                to,
                token,
                amount,
            },
            0,
        )
        .expect("Failed to reply Event::TransferSingle");
    }
    fn emit_transfer_batch_event(
        &self,
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        token: Vec<u128>,
        amount: Vec<u128>,
    ) {
        gstd::msg::reply(
            Event::TransferBatch {
                operator,
                from,
                to,
                token,
                amount,
            },
            0,
        )
        .expect("Failed to reply Event::TransferBatch");
    }
    fn emit_approval_for_all_event(&self, owner: ActorId, operator: ActorId, approved: bool) {
        gstd::msg::reply(
            Event::ApprovedForAll {
                owner,
                operator,
                approved,
            },
            0,
        )
        .expect("Failed to reply Event::ApprovedForAll");
    }
    fn emit_uri_event(&self, value: String, token: u128) {
        gstd::msg::reply(Event::URI { value, token }, 0).expect("Failed to reply Event::URI");
    }
}
