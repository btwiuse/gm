//! contract I/O types for initialization, transactions, state queries and events

use crate::*;
use ::codec::{Decode, Encode};
use ::scale_info::TypeInfo;

/// contract I/O types for initialization
pub mod init {
    use super::*;
    #[derive(Debug, TypeInfo, Encode, Decode)]
    pub struct Init {
        pub name: String,
        pub symbol: String,
        pub base_uri: String,
    }
}

/// contract I/O types for transactions
pub mod transaction {
    use super::*;
    #[derive(Debug, TypeInfo, Decode, Encode)]
    pub enum Input {
        // Payload(String),
        /*
        BalanceOf {
            who: ActorId,
            token: u128,
        },
        BalanceOfBatch {
            who: Vec<ActorId>,
            token: Vec<u128>,
        },
        */
        TransferFrom {
            from: ActorId,
            to: ActorId,
            token: u128,
            amount: u128,
        },
        BatchTransferFrom {
            from: ActorId,
            to: ActorId,
            token: Vec<u128>,
            amount: Vec<u128>,
        },
        SetApprovalForAll {
            operator: ActorId,
            approved: bool,
        },
        Mint {
            to: ActorId,
            token: u128,
            amount: u128,
        },
        MintBatch {
            to: ActorId,
            token: Vec<u128>,
            amount: Vec<u128>,
        },
        Burn {
            from: ActorId,
            token: u128,
            amount: u128,
        },
        BurnBatch {
            from: ActorId,
            token: Vec<u128>,
            amount: Vec<u128>,
        },
        UpdateTokenMetadata {
            token: u128,
            metadata: Option<TokenMetadata>,
        }
    }

    #[derive(Debug, TypeInfo, Encode, Decode)]
    pub enum Output {
        Payload(ActorId),
    }
}

/// contract I/O types for state queries
pub mod query {
    use super::*;
    #[derive(Debug, TypeInfo, Decode, Encode)]
    pub enum Query {
        Name,
        Symbol,
        BaseUri,
        BalanceOf(ActorId, u128),
        BalanceOfBatch(Vec<ActorId>, Vec<u128>),
        /*
        Metadata(TokenId),
        TokensForOwner(ActorId),
        TokensIDsForOwner(ActorId),
        Supply(TokenId),
        OwnerOf(TokenId),
        */
        IsApprovedForAll { who: ActorId, operator: bool },
        TokenMetadata(u128),
    }

    #[derive(Debug, TypeInfo, Encode, Decode)]
    pub enum State {
        // AccountId(ActorId),
        Name(String),
        Symbol(String),
        BaseUri(String),
        BalanceOf(ActorId, u128, u128),
        BalanceOfBatch(Vec<ActorId>, Vec<u128>, Vec<u128>),
        /*
        Balance(ActorId),
        URI(String),
        Metadata(TokenMetadata),
        Tokens(Vec<>),
        Supply(Vec<>),
        OwnerOf(Vec<>),
        */
        IsApprovedForAll(bool),
        TokenMetadata(u128, Option<TokenMetadata>)
    }
}

/// contract I/O types for events
pub mod event {
    use super::*;
    #[derive(Debug, TypeInfo, Encode, Decode)]
    pub enum Event {
        TransferSingle {
            operator: ActorId,
            from: ActorId,
            to: ActorId,
            token: u128,
            amount: u128,
        },
        TransferBatch {
            operator: ActorId,
            from: ActorId,
            to: ActorId,
            token: Vec<u128>,
            amount: Vec<u128>,
        },
        ApprovalForAll {
            who: ActorId,
            operator: ActorId,
            approved: bool,
        },
    }
}

/// token metadata
#[derive(Debug, TypeInfo, Encode, Decode)]
pub struct TokenMetadata {
    pub name: String,
    pub description: String,
    pub image_uri: String,
    pub json_uri: String,
}

pub use self::event::*;
pub use self::init::*;
pub use self::query::*;
pub use self::transaction::*;
