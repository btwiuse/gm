#![no_std]

use gstd::{debug, msg, prelude::*, ActorId};

static mut SELF: Contract = Contract::default();

pub struct Contract {
    owner: ActorId,
}

impl Contract {
    const fn default() -> Self {
        Contract {
            owner: ActorId::zero(),
        }
    }
}

impl Ownable for Contract {
    fn owner(&self) -> ActorId {
        self.owner
    }
    fn is_owner(&self, who: &ActorId) -> bool {
        *who == self.owner
    }
}

pub trait Ownable {
    fn owner(&self) -> ActorId;
    fn is_owner(&self, who: &ActorId) -> bool;
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let id: ActorId = msg::source();
    debug!("id: {:?}", id);
    match SELF.is_owner(&id) {
        true => msg::reply_bytes([1], 0).expect("Failed to reply"),
        false => msg::reply_bytes([0], 0).expect("Failed to reply"),
    };
}

#[no_mangle]
unsafe extern "C" fn init() {
    let id: ActorId = msg::source();
    SELF.owner = id;
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
