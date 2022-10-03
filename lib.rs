//! high level trait definitions and abstractions,
//! also acts as prelude for the entire crate

#![feature(trait_alias)]
#![no_std]

use gstd::{debug, msg, prelude::*, ActorId};
use primitive_types::U256;

pub mod codec;
pub mod config;
pub mod contract;
pub mod init;
pub mod metadata;
pub mod query;
pub mod state;
pub mod transaction;

pub use crate::codec::{Event, Init, Input, Output, Query, State};
pub use config::{GearConfig, TestConfig};
pub use contract::Contract;
pub use state::STATE;

/// Ownable interface definition
// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/access/Ownable.sol
pub trait IOwnable<T: IConfig> {
    fn owner(&self) -> T::AccountId;
    fn is_owner(&self, who: &T::AccountId) -> bool;
}

/// Ledger interface definition
pub trait ILedger<T: IConfig> {
    fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance;
    fn balance_incr(&mut self, who: &T::AccountId);
}

/// ERC20 interface definition
// https://github.com/paritytech/ink/blob/master/examples/erc20/lib.rs
// https://github.com/ethereum/EIPs/blob/master/EIPS/eip-20.md
pub trait IERC20<T: IConfig> {
    fn symbol(&self) -> T::Text;
    fn name(&self) -> T::Text;
    fn decimals(&self) -> T::TokenDecimal;
    fn total_issuance(&self) -> T::AccountBalance;
    fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance;
    fn transfer(&mut self, to: &T::AccountId, amount: T::AccountBalance);
    fn transfer_from(&mut self, from: &T::AccountId, to: &T::AccountId, amount: T::AccountBalance);
    fn allowance(&self, owner: &T::AccountId, spender: &T::AccountId) -> T::AccountBalance;
    fn emit_transfer_event(from: &T::AccountId, to: &T::AccountId, amount: &T::AccountBalance);
    fn emit_approval_event(
        owner: &T::AccountId,
        spender: &T::AccountId,
        amount: &T::AccountBalance,
    );
    fn burn(&mut self);
    fn mint(&mut self);
}

/// ERC721 interface definition
// https://eips.ethereum.org/EIPS/eip-721
pub trait IERC721<T: IConfig> {
    fn symbol(&self) -> T::Text;
    fn name(&self) -> T::Text;
    fn decimals(&self) -> T::TokenDecimal;
    fn total_issuance(&self) -> T::AccountBalance;
    fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance;
    fn owner_of(&self, token: &T::TokenId) -> T::AccountId;
    fn safe_transfer_from(&mut self, from: &T::AccountId, to: &T::AccountId, token: &T::TokenId);
    fn transfer_from(&mut self, from: &T::AccountId, to: &T::AccountId, token: &T::TokenId);
    fn approve(&self, who: &T::AccountId, token: &T::TokenId);
    fn get_approved(&self, token: &T::TokenId) -> T::AccountId;
    fn emit_transfer_event(from: &T::AccountId, to: &T::AccountId, token: &T::TokenId);
    fn emit_approval_event(owner: &T::AccountId, spender: &T::AccountId, token: &T::TokenId);
    fn burn(&mut self, token: &T::TokenId);
    fn mint(&mut self, to: &T::AccountId, token: &T::TokenId);
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
    // fn decimals(&self) -> T::TokenDecimal;
    // fn total_issuance(&self) -> T::AccountBalance;
    /// fn approve(&self, who: &T::AccountId, token: &T::TokenId);
    // fn get_approved(&self, token: &T::TokenId) -> T::AccountId;
    // fn emit_transfer_single_event( &self, operator: &T::AccountId, from: &T::AccountId, to: &T::AccountId, token: &T::TokenId, amount: &T::AccountBalance,);
    // fn emit_transfer_batch_event( operator: &T::AccountId, from: &T::AccountId, to: &T::AccountId, token: &[T::TokenId], amount: &[T::AccountBalance],);
    // fn emit_approval_for_all_event(owner: &T::AccountId, spender: &T::AccountId, approved: bool);
    // fn emit_uri_event(value: &T::Text, token: &T::TokenId);
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
    /*
    fn transfer_from(
        &mut self,
        from: &T::AccountId,
        to: &T::AccountId,
        token: &T::TokenId,
        amount: &T::AccountBalance,
    );
    */
    fn set_approval_for_all(&mut self, operator: T::AccountId, approved: bool);
    fn is_approved_for_all(&mut self, owner: T::AccountId, operator: T::AccountId) -> bool;
}

/// configuration trait that abstracts contract implementation from concrete types
pub trait IConfig {
    type AccountId: IAccountId;
    type AccountBalance: IAccountBalance;
    type Text: IText;
    type TokenDecimal: ITokenDecimal;
    type TokenId: ITokenId;
}

/// token id trait alias
pub trait ITokenId = Eq + Copy + Clone + core::hash::Hash + Ord;

/// account id trait alias
///
/// a method for returning the zero address is required.
pub trait IAccountId = zero::IZero + Eq + Copy + Clone + core::hash::Hash + Ord;

/// account balance trait alias
///
/// any unsigned integer type that is at least u32 should work.
pub trait IAccountBalance = num_traits::Zero
    + num_traits::One
    + num_traits::CheckedAdd
    + num_traits::CheckedSub
    + num_traits::sign::Unsigned
    + Copy
    + Clone
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
    + num_traits::sign::Unsigned
    + Copy
    + Clone
    + Default;

/// text trait
///
/// default value should be an empty string
pub trait IText: From<String> + ToString + Clone {
    fn default() -> Self;
}

impl IText for String {
    fn default() -> Self {
        return String::from("");
    }
}

/// define IZero trait and implement it for ActorId
mod zero {
    use crate::*;
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

    impl IZero for u8 {
        fn zero() -> Self {
            0u8
        }
        fn is_zero(&self) -> bool {
            *self == IZero::zero()
        }
    }
}
