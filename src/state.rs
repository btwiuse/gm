//! contract state

use crate::*;

/// contract state is populated during initialization
pub static mut STATE: Option<Contract<GearConfig>> = None;
