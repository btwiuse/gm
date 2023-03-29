#![no_std]

use config::GearConfig;
use gmeta::{metawasm, Metadata};
use gstd::prelude::*;
use interface::*;
use io::TokenMetadata;
use metadata::ProgramMetadata;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn name(state: State) -> String {
        state.name()
    }
    pub fn symbol(state: State) -> String {
        state.symbol()
    }
    pub fn base_uri(state: State) -> String {
        state.base_uri
    }
    pub fn token_metadata(
        state: State,
        token: <GearConfig as IConfig>::TokenId,
    ) -> Option<TokenMetadata> {
        state.get_token_metadata(token)
    }
    pub fn is_approved_for_all(
        state: State,
        owner: <GearConfig as IConfig>::AccountId,
        operator: <GearConfig as IConfig>::AccountId,
    ) -> bool {
        state.is_approved_for_all(owner, operator)
    }
    pub fn balance_of(
        state: State,
        who: <GearConfig as IConfig>::AccountId,
        token: <GearConfig as IConfig>::TokenId,
    ) -> <GearConfig as IConfig>::Balance {
        state.balance_of(who, token)
    }
    pub fn balance_of_batch(
        state: State,
        who: Vec<<GearConfig as IConfig>::AccountId>,
        token: Vec<<GearConfig as IConfig>::TokenId>,
    ) -> Vec<<GearConfig as IConfig>::Balance> {
        state.balance_of_batch(who, token)
    }
}
