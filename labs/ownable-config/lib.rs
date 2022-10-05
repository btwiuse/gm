#![feature(trait_alias)]
#![no_std]

use gstd::{debug, msg, prelude::*, ActorId};

mod config;
mod contract;

use config::Config;
use config::Ownable;
use contract::Contract;

#[derive(Clone, Copy)]
struct CONFIG;

impl Config for CONFIG {
    type AccountId = ActorId;
}

static mut SELF: Option<Contract<CONFIG>> = None;

#[no_mangle]
unsafe extern "C" fn handle() {
    let id: ActorId = msg::source();
    debug!("id: {:?}", id);
    match SELF.unwrap().is_owner(&id) {
        true => msg::reply_bytes([1], 0).expect("Failed to reply"),
        false => msg::reply_bytes([0], 0).expect("Failed to reply"),
    };
}

#[no_mangle]
unsafe extern "C" fn init() {
    let id: ActorId = msg::source();
    // SELF.owner = id;
    SELF = Some(Contract::<CONFIG>::new(&id));
    debug!("init(OWNER = {:?})", id);
}

#[cfg(test)]
mod tests {
    extern crate std;

    use gtest::{Program, System};

    #[test]
    fn it_works() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "init");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        assert!(res.log().len() == 1);
        assert!(res.log()[0].payload().starts_with(&[1]));

        let res = program.send_bytes(69, "Gear");
        assert!(res.log().len() == 1);
        assert!(res.log()[0].payload().starts_with(&[0]));
    }
}
