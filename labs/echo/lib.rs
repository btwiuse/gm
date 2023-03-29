#![no_std]

use gstd::{debug, msg, prelude::*};

#[no_mangle]
unsafe extern "C" fn handle() {
    let payload = String::from_utf8(msg::load_bytes().unwrap()).expect("Invalid handle message");
    debug!("handle(): {}", payload);
    msg::reply_bytes(payload, 0).expect("Failed to reply");
}

#[no_mangle]
unsafe extern "C" fn init() {
    let payload = String::from_utf8(msg::load_bytes().unwrap()).expect("Invalid init message");
    debug!("init(): {}", payload);
}

#[cfg(test)]
mod tests {
    extern crate std;

    use gtest::{Log, Program, System};

    #[test]
    fn it_works() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "Let's start");
        // assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        assert!(!res.log().is_empty());

        let log = Log::builder().source(42).dest(1).payload_bytes("Hello");
        // assert!(res.contains(&log));

        let res = program.send_bytes(69, "Gear");
        assert!(!res.log().is_empty());

        let log = Log::builder().source(1).dest(69).payload_bytes("Gear");
        assert!(res.contains(&log));
    }
}
