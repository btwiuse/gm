#![feature(trait_alias)]
#![no_std]

use gstd::{debug, msg, prelude::*, ActorId};

mod config;
mod contract;

use config::Config;
use config::Ledger;
use config::Ownable;
use contract::Contract;

#[derive(Clone, Copy)]
struct CONFIG;

impl Config for CONFIG {
    type AccountId = ActorId;
    type Balance = u8;
}

static mut SELF: Option<Contract<CONFIG>> = None;

#[no_mangle]
unsafe extern "C" fn handle() {
    let id: ActorId = msg::source();
    SELF.balance_incr(&id);
    debug!(
        "id: {:?}, balance: {:?}, is_owner: {}",
        id,
        SELF.balance_of(&id),
        SELF.is_owner(&id)
    );
    match SELF.is_owner(&id) {
        true => msg::reply_bytes([SELF.balance_of(&id), 1], 0).expect("Failed to reply"),
        false => msg::reply_bytes([SELF.balance_of(&id), 0], 0).expect("Failed to reply"),
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
        assert!(res.log()[0].payload().starts_with(&[1, 1]));

        let res = program.send_bytes(69, "Gear");
        assert!(res.log().len() == 1);
        assert!(res.log()[0].payload().starts_with(&[1, 0]));

        let res = program.send_bytes(69, "Gear");
        assert!(res.log().len() == 1);
        assert!(res.log()[0].payload().starts_with(&[2, 0]));
    }

    #[test]
    fn it_works_too() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "init");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        assert!(res.log().len() == 1);
        assert!(res.log()[0].payload().starts_with(&[1, 1]));

        let res = program.send_bytes(42, "Hello");
        assert!(res.log().len() == 1);
        assert!(res.log()[0].payload().starts_with(&[2, 1]));

        let res = program.send_bytes(69, "Gear");
        assert!(res.log().len() == 1);
        assert!(res.log()[0].payload().starts_with(&[1, 0]));

        let res = program.send_bytes(69, "Gear");
        assert!(res.log().len() == 1);
        assert!(res.log()[0].payload().starts_with(&[2, 0]));
    }
}
