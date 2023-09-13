#![no_std]

//! contract associated types configuration
//!
//! GearConfig is used in formal code, see state.rs.
//!
//! For testing purpose, MockConfig is preferred, see contract_panic_test.rs and contract_test.rs

use ::parity_scale_codec::{Decode, Encode};
use ::scale_info::TypeInfo;
use gstd::{prelude::*, ActorId};
use interface::*;

/// GearConfig implements IConfig for gear environment
#[derive(Default, Clone, Copy, Encode, Decode, TypeInfo)]
pub struct GearConfig;

impl IConfig for GearConfig {
    type AccountId = ActorId;
    type Balance = u128;
    type TokenId = u128;
    type Text = String;
    fn sender(&self) -> Self::AccountId {
        gstd::msg::source()
    }
}

/// GearConfig implements IConfig for testing environment
#[derive(Default, Clone, Copy, PartialOrd, Eq, PartialEq)]
pub struct MockConfig {
    pub sender: u8,
}

impl MockConfig {
    pub fn set_sender(&mut self, sender: u8) {
        self.sender = sender;
    }
}

impl IConfig for MockConfig {
    type AccountId = u8;
    type Balance = u32;
    type TokenId = u8;
    type Text = String;
    fn sender(&self) -> Self::AccountId {
        self.sender
    }
}
