#![no_std]

use gstd::{debug, msg, prelude::*, ActorId};

gstd::metadata! {
    title: "nonce",
    handle:
        input: Vec<u8>,
        output: u8,
}

static mut STATE: BTreeMap<ActorId, u8> = BTreeMap::new();

#[no_mangle]
unsafe extern "C" fn handle() {
    let id: ActorId = msg::source();
    debug!("handle(): {:?}", id);

    let nonce = STATE.get(&id).cloned().unwrap_or(0) + 1;

    STATE.insert(id, nonce);

    debug!("{:?}: {}", id, nonce);
    msg::reply_bytes([nonce], 0).expect("Failed to reply");
}

#[no_mangle]
unsafe extern "C" fn init() {
    let payload = String::from_utf8(msg::load_bytes()).expect("Invalid init message");
    debug!("init(): {}", payload);
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use gtest::{Program, System};

    #[test]
    fn it_works() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "init");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        assert_eq!(res.log().len(), 1);
        assert_eq!(res.log()[0].payload(), &[1]);

        let res = program.send_bytes(69, "Gear");
        assert_eq!(res.log().len(), 1);
        assert_eq!(res.log()[0].payload(), &[1]);

        let res = program.send_bytes(69, "Gear");
        assert_eq!(res.log().len(), 1);
        assert_eq!(res.log()[0].payload(), &[2]);
    }
}
