//! contract implementation

use crate::*;

use ::parity_scale_codec::{Decode, Encode};
use ::scale_info::TypeInfo;

mod contract_panic_test;
mod contract_test;

mod erc1155;
mod erc1155_check;
mod erc1155_ext;
mod erc1155_gear_ext;
mod erc1155_metadata_uri;
mod token_metadata_registry;

/// Contract struct
#[derive(Default, Clone, Encode, Decode, TypeInfo)]
pub struct Contract<T: IConfig> {
    pub ctx: T,
    pub owner: T::AccountId,
    pub name: T::Text,
    pub symbol: T::Text,
    pub base_uri: T::Text,
    pub balances: BTreeMap<T::TokenId, BTreeMap<T::AccountId, T::Balance>>,
    pub approvals: BTreeMap<T::AccountId, BTreeMap<T::AccountId, bool>>,
    pub metadata_registry: BTreeMap<T::TokenId, TokenMetadata>,
}

/// constructor method
impl<T: IConfig> Contract<T> {
    pub fn new(owner: &T::AccountId) -> Self {
        Self {
            owner: *owner,
            ..Self::default()
        }
    }
    pub fn sender(&self) -> T::AccountId {
        self.ctx.sender()
    }
    pub fn origin(&self) -> T::AccountId {
        self.ctx.origin()
    }
}
