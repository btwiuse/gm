//! contract I/O types for initialization, transactions, state queries and events

use crate::*;
use ::codec::{Decode, Encode};
use ::scale_info::TypeInfo;

/// contract I/O types for initialization
pub mod init {
    use super::*;
    #[derive(Debug, TypeInfo, Encode, Decode, PartialEq, Clone)]
    pub struct Init {
        pub name: String,
        pub symbol: String,
        pub base_uri: String,
    }
    #[derive(Debug, TypeInfo, Encode, Decode, PartialEq, Clone)]
    pub struct InitOk;
}

/// contract I/O types for transactions and events
pub mod transaction {
    use super::*;
    #[derive(Debug, TypeInfo, Decode, Encode, PartialEq, Clone)]
    pub enum Input {
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
        },
    }
    #[derive(Debug, TypeInfo, Encode, Decode, PartialEq, Clone)]
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
            owner: ActorId,
            operator: ActorId,
            approved: bool,
        },
        URI {
            value: String,
            token: u128,
        },
    }
}

/// contract I/O types for state queries and replies
pub mod query {
    use super::*;
    #[derive(Debug, TypeInfo, Decode, Encode, PartialEq, Clone)]
    pub enum Query {
        Name,
        Symbol,
        BaseUri,
        BalanceOf(ActorId, u128),
        BalanceOfBatch(Vec<ActorId>, Vec<u128>),
        IsApprovedForAll { owner: ActorId, operator: ActorId },
        TokenMetadata(u128),
    }

    #[derive(Debug, TypeInfo, Encode, Decode, PartialEq, Clone)]
    pub enum State {
        Name(String),
        Symbol(String),
        BaseUri(String),
        BalanceOf(u128),
        BalanceOfBatch(Vec<u128>),
        IsApprovedForAll(bool),
        TokenMetadata(Option<TokenMetadata>),
    }
}

/// token metadata
#[derive(Debug, TypeInfo, Encode, Decode, PartialEq, Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub description: String,
    pub image_uri: String,
    pub json_uri: String,
}

pub use self::init::*;
pub use self::query::*;
pub use self::transaction::*;
