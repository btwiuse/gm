#![no_std]

use gstd::{debug, msg, prelude::*, ActorId};

static mut OWNER: Option<ActorId> = None;

#[no_mangle]
unsafe extern "C" fn handle() {
    let id: ActorId = msg::source();
    debug!("id: {:?}", id);
    let is_owner = Some(id) == OWNER;
    debug!("is_owner: {:?}", is_owner);
    match is_owner {
        true => msg::reply_bytes([1], 0).expect("Failed to reply"),
        false => msg::reply_bytes([0], 0).expect("Failed to reply"),
    };
}

#[no_mangle]
unsafe extern "C" fn init() {
    let id: ActorId = msg::source();
    OWNER = Some(id);
    debug!("init(OWNER = {:?})", OWNER);
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
