use gstd::{prelude::*, ActorId};

pub type State = Vec<ActorId>;

pub static mut STATE: State = vec![];
