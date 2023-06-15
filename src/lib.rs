//! high level trait definitions and abstractions,
//! also acts as prelude for the entire crate

#![feature(trait_alias)]
#![no_std]

use gstd::{prelude::*, ActorId};

use config::*;
use contract::*;
use interface::*;
use io::*;

pub mod handle;
pub mod init;
pub mod state;

pub use state::STATE;
