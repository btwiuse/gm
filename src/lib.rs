//! high level trait definitions and abstractions,
//! also acts as prelude for the entire crate

#![feature(trait_alias)]
#![no_std]

use gstd::{prelude::*, ActorId};

pub mod codec;
pub mod config;
pub mod contract;
pub mod handle;
pub mod init;
pub mod metadata;
pub mod query;
pub mod state;

pub use crate::codec::{Action, Event, Init, InitOk, Query, State, TokenMetadata};
pub use config::{GearConfig, MockConfig};
pub use contract::Contract;
pub use state::STATE;

/// ERC1155 interface check extension
pub trait IERC1155Check<T: IConfig> {
    fn check_transfer_from(
        &self,
        from: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::Balance,
    );
    fn check_batch_transfer_from(
        &self,
        from: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::Balance>,
    );
    fn check_balance_of_batch(&self, who: Vec<T::AccountId>, token: Vec<T::TokenId>);
    fn check_mint(&self, to: T::AccountId, token: T::TokenId, amount: T::Balance);
    fn check_mint_batch(&self, to: T::AccountId, token: Vec<T::TokenId>, amount: Vec<T::Balance>);
    fn check_set_approval_for_all(
        &self,
        owner: T::AccountId,
        operator: T::AccountId,
        approved: bool,
    );
    fn check_burn(&self, from: T::AccountId, token: T::TokenId, amount: T::Balance);
    fn check_burn_batch(&self, from: T::AccountId, token: Vec<T::TokenId>, amount: Vec<T::Balance>);
    fn check_update_token_metadata(&self, token: T::TokenId, metadata: Option<TokenMetadata>);
}

/// ERC1155 interface gear extension
pub trait IERC1155GearExt {
    fn emit_transfer_single_event(
        &self,
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        token: u128,
        amount: u128,
    );
    fn emit_transfer_batch_event(
        &self,
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        token: Vec<u128>,
        amount: Vec<u128>,
    );
    fn emit_approval_for_all_event(&self, owner: ActorId, spender: ActorId, approved: bool);
    fn emit_uri_event(&self, value: String, token: u128);
    /// whoami is a utility method for emitting an event containing sender and origin of the current tx
    fn emit_whoami_event(&self);
    fn emit_update_token_metadata_event(&self, token: u128, metadata: Option<TokenMetadata>);
}

/// ERC1155 interface extension
pub trait IERC1155Ext<T: IConfig>: IERC1155<T> {
    fn name(&self) -> T::Text;
    fn symbol(&self) -> T::Text;
    fn burn(&mut self, from: T::AccountId, token: T::TokenId, amount: T::Balance);
    fn burn_batch(&mut self, from: T::AccountId, token: Vec<T::TokenId>, amount: Vec<T::Balance>);
    fn mint(&mut self, to: T::AccountId, token: T::TokenId, amount: T::Balance);
    fn mint_batch(&mut self, to: T::AccountId, token: Vec<T::TokenId>, amount: Vec<T::Balance>);
}

/// ERC1155MetadataURI interface definition
// https://eips.ethereum.org/EIPS/eip-1155#metadata
pub trait IERC1155MetadataURI<T: IConfig> {
    fn uri(&self, token: T::TokenId) -> T::Text;
}

pub trait ITokenMetadataRegistry<T: IConfig> {
    fn get_token_metadata(&self, token: T::TokenId) -> Option<TokenMetadata>;
    fn update_token_metadata(&mut self, token: T::TokenId, metadata: Option<TokenMetadata>);
}

/// ERC1155 interface definition
// https://eips.ethereum.org/EIPS/eip-1155
// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC1155/ERC1155.sol
// https://github.com/paritytech/ink/blob/master/examples/erc1155/lib.rs
pub trait IERC1155<T: IConfig>: IERC1155Check<T> {
    fn balance_of(&self, who: T::AccountId, token: T::TokenId) -> T::Balance;
    fn balance_of_batch(&self, who: Vec<T::AccountId>, token: Vec<T::TokenId>) -> Vec<T::Balance>;
    fn safe_transfer_from(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::Balance,
    );
    fn safe_batch_transfer_from(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::Balance>,
    );
    fn set_approval_for_all(&mut self, owner: T::AccountId, operator: T::AccountId, approved: bool);
    fn is_approved_for_all(&self, owner: T::AccountId, operator: T::AccountId) -> bool;
}

/// configuration trait that abstracts contract implementation from concrete types
///
/// making the contract testable without gear dependencies
pub trait IConfig: Default {
    type AccountId: IAccountId;
    type Balance: IBalance;
    type Text: IText;
    type TokenId: ITokenId;
    fn origin(&self) -> Self::AccountId;
    fn sender(&self) -> Self::AccountId;
}

/// token id trait alias
pub trait ITokenId = Eq + Copy + Clone + core::hash::Hash + Ord + fmt::Debug + Default;

/// account id trait alias
///
/// a method for returning the zero address is required.
pub trait IAccountId = Eq + Copy + Clone + core::hash::Hash + Ord + fmt::Debug + Default;

/// account balance trait alias
///
/// any unsigned integer type that is at least u32 should work.
pub trait IBalance = num_traits::Zero
    + num_traits::One
    + num_traits::CheckedAdd
    + num_traits::CheckedSub
    + num_traits::SaturatingAdd
    + num_traits::SaturatingSub
    + num_traits::sign::Unsigned
    + fmt::Debug
    + Copy
    + Clone
    + PartialOrd
    + Default
    + From<u16>
    + From<u32>;

/// text trait
///
/// default value should be an empty string
pub trait IText = From<&'static str> + Clone + fmt::Debug + Default;
