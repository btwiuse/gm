//! contract implementation

use crate::*;

mod contract_panic_test;
mod contract_test;

mod erc1155;
mod erc1155_check;
mod erc1155_ext;
mod erc1155_gear_ext;
mod erc1155_metadata_uri;
mod token_metadata_registry;

/// Contract struct
pub struct Contract<T: IConfig> {
    pub env: T,
    pub owner: T::AccountId,
    pub name: T::Text,
    pub symbol: T::Text,
    pub base_uri: T::Text,
    pub balances: BTreeMap<T::TokenId, BTreeMap<T::AccountId, T::AccountBalance>>,
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
        self.env.sender()
    }
    pub fn origin(&self) -> T::AccountId {
        self.env.origin()
    }
}

impl<T: IConfig> Default for Contract<T> {
    fn default() -> Self {
        Self {
            env: T::default(),
            owner: T::AccountId::zero(),
            name: T::Text::default(),
            symbol: T::Text::default(),
            base_uri: T::Text::default(),
            balances: BTreeMap::<T::TokenId, BTreeMap<T::AccountId, T::AccountBalance>>::default(),
            approvals: BTreeMap::<T::AccountId, BTreeMap<T::AccountId, bool>>::default(),
            metadata_registry: BTreeMap::<T::TokenId, TokenMetadata>::default(),
        }
    }
}
