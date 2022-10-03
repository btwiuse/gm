//! contract associated types configuration
//!
//! GearConfig is used in formal code. For testing purpose, the TestConfig is preferred.
//!
//! In TestConfig, the account id type is set to u8 in order to simplify writing account id literals in tests

use crate::*;

/// GearConfig implements IConfig for gear environment
#[derive(Clone, Copy)]
pub struct GearConfig;

impl IConfig for GearConfig {
    type AccountId = ActorId;
    type AccountBalance = u128;
    type TokenDecimal = u8;
    type TokenId = u128;
    type Text = String;
}

/// GearConfig implements IConfig for testing environment
#[derive(Clone, Copy)]
pub struct TestConfig;

impl IConfig for TestConfig {
    type AccountId = u8;
    type AccountBalance = u32;
    type TokenDecimal = u8;
    type TokenId = u8;
    type Text = String;
}
