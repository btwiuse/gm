#![no_std]

use config::GearConfig;
use contract::Contract;
use gmeta::{InOut, Metadata};
use io::*;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<Init, InitOk>;
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Contract<GearConfig>;
}
