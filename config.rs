//! contract associated types configuration
//!
//! GearConfig is used in formal code, see state.rs.
//!
//! For testing purpose, MockConfig is preferred, see contract_test.rs

use crate::*;

/// GearConfig implements IConfig for gear environment
#[derive(Default, Clone, Copy)]
pub struct GearConfig;

impl IConfig for GearConfig {
    type AccountId = ActorId;
    type AccountBalance = u128;
    type TokenDecimal = u8;
    type TokenId = u128;
    type Text = String;
    fn origin(&self) -> Self::AccountId {
        gstd::exec::origin()
    }
    fn sender(&self) -> Self::AccountId {
        gstd::msg::source()
    }
}

/// GearConfig implements IConfig for testing environment
#[derive(Default, Clone, Copy, PartialOrd, Eq, PartialEq)]
pub struct MockConfig {
    pub origin: u8,
    pub sender: u8,
}

impl MockConfig {
    pub fn set_sender(&mut self, sender: u8) {
        self.sender = sender;
    }
    pub fn set_origin(&mut self, origin: u8) {
        self.origin = origin;
    }
    pub fn set_sender_origin(&mut self, sender: u8, origin: u8) {
        self.set_sender(sender);
        self.set_origin(origin);
    }
}

impl IConfig for MockConfig {
    type AccountId = u8;
    type AccountBalance = u32;
    type TokenDecimal = u8;
    type TokenId = u8;
    type Text = String;
    fn origin(&self) -> Self::AccountId {
        self.origin
    }
    fn sender(&self) -> Self::AccountId {
        self.sender
    }
}
