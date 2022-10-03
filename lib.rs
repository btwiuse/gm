//! high level trait definitions and abstractions,
//! also acts as prelude for the entire crate

#![feature(trait_alias)]
#![no_std]

use gstd::{debug, exec, msg, prelude::*, ActorId};

pub mod codec;
pub mod config;
pub mod contract;
pub mod handle;
pub mod init;
pub mod metadata;
pub mod query;
pub mod state;

mod contract_test;
mod handle_test;
mod init_test;
mod query_test;

pub use crate::codec::{Event, Init, InitOk, Input, Query, State, TokenMetadata};
pub use config::{GearConfig, TestConfig};
pub use contract::Contract;
pub use state::STATE;

/// Ownable interface definition
// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/access/Ownable.sol
pub trait IOwnable<T: IConfig> {
    fn owner(&self) -> T::AccountId;
    fn is_owner(&self, who: &T::AccountId) -> bool;
}

/// ERC1155 interface check extension
pub trait IERC1155Check<T: IConfig> {
    fn check_transfer_from(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        from: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    );
    fn check_batch_transfer_from(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        from: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    );
    fn check_mint(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    );
    fn check_mint_batch(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    );
    fn check_set_approval_for_all(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        owner: T::AccountId,
        operator: T::AccountId,
        approved: bool,
    );
    fn check_burn(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        from: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    );
    fn check_burn_batch(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        from: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    );
    fn check_update_token_metadata(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        token: T::TokenId,
        metadata: Option<TokenMetadata>,
    );
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
}

/// ERC1155 interface extension
pub trait IERC1155Ext<T: IConfig> {
    fn name(&self) -> T::Text;
    fn symbol(&self) -> T::Text;
    fn burn(&mut self, from: T::AccountId, token: T::TokenId, amount: T::AccountBalance);
    fn burn_batch(
        &mut self,
        from: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    );
    fn mint(&mut self, to: T::AccountId, token: T::TokenId, amount: T::AccountBalance);
    fn mint_batch(
        &mut self,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    );
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
pub trait IERC1155<T: IConfig> {
    fn balance_of(&self, who: T::AccountId, token: T::TokenId) -> T::AccountBalance;
    fn balance_of_batch(
        &self,
        who: Vec<T::AccountId>,
        token: Vec<T::TokenId>,
    ) -> Vec<T::AccountBalance>;
    fn safe_transfer_from(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    );
    fn safe_batch_transfer_from(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    );
    fn set_approval_for_all(&mut self, owner: T::AccountId, operator: T::AccountId, approved: bool);
    fn is_approved_for_all(&self, owner: T::AccountId, operator: T::AccountId) -> bool;
}

/// configuration trait that abstracts contract implementation from concrete types
///
/// making the contract testable without gear dependencies
pub trait IConfig {
    type AccountId: IAccountId;
    type AccountBalance: IAccountBalance;
    type Text: IText;
    type TokenDecimal: ITokenDecimal;
    type TokenId: ITokenId;
}

/// token id trait alias
pub trait ITokenId = Eq + Copy + Clone + core::hash::Hash + Ord + fmt::Debug;

/// account id trait alias
///
/// a method for returning the zero address is required.
pub trait IAccountId = zero::IZero + Eq + Copy + Clone + core::hash::Hash + Ord + fmt::Debug;

/// account balance trait alias
///
/// any unsigned integer type that is at least u32 should work.
pub trait IAccountBalance = num_traits::Zero
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

/// token decimal trait alias
///
/// any unsigned integer type that is at least u8 should work.
pub trait ITokenDecimal = num_traits::Zero
    + num_traits::One
    + num_traits::CheckedAdd
    + num_traits::CheckedSub
    + num_traits::SaturatingAdd
    + num_traits::SaturatingSub
    + num_traits::sign::Unsigned
    + fmt::Debug
    + Copy
    + Clone
    + Default;

/// text trait
///
/// default value should be an empty string
pub trait IText: From<String> + ToString + Clone + fmt::Debug {
    fn default() -> Self;
}

impl IText for String {
    fn default() -> Self {
        return String::from("");
    }
}

/// define IZero trait required by IAccountId and implement it for ActorId and u8
/// for gear and testing environments respectively
mod zero {
    use crate::*;

    /// IZero is a trait with methods for obtaining / comparing with the zero address
    pub trait IZero {
        fn zero() -> Self;
        fn is_zero(&self) -> bool;
    }

    impl IZero for ActorId {
        fn zero() -> Self {
            ActorId::zero()
        }
        fn is_zero(&self) -> bool {
            *self == Self::zero()
        }
    }

    // u8 is used as AccountId in contract_test.rs
    impl IZero for u8 {
        fn zero() -> Self {
            0u8
        }
        fn is_zero(&self) -> bool {
            *self == 0u8
        }
    }
}
