//! high level trait definitions and abstractions,
//! also acts as prelude for the entire crate

#![feature(trait_alias)]
#![no_std]

use gstd::{prelude::*, ActorId};

use io::*;

// pub mod codec;
// pub mod config;
// pub mod contract;
pub mod handle;
pub mod init;
// pub mod metadata;
// pub mod query;
pub mod state;

// pub use crate::codec::{Action, Event, Init, InitOk, Query, State, TokenMetadata};
// pub use config::{GearConfig, MockConfig};
// pub use contract::Contract;
pub use state::STATE;

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    gstd::msg::reply(metahash, 0).expect("Failed to share metahash");
}
