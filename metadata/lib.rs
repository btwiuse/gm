#![no_std]

use gmeta::{InOut, Metadata};
use io::*;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<Init, InitOk>;
    type Handle = InOut<Action, Event>;
    type State = InOut<Query, State>;
    type Others = ();
    type Reply = ();
    type Signal = ();
}
