#![no_std]

use gstd::{debug, msg, prelude::*};

#[no_mangle]
unsafe extern "C" fn handle() {
    let payload = String::from_utf8(msg::load_bytes()).expect("Invalid handle message");
    debug!("handle(): {}", payload);
}

#[no_mangle]
unsafe extern "C" fn init() {
    let payload = String::from_utf8(msg::load_bytes()).expect("Invalid init message");
    debug!("init(): {}", payload);
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

        let res = program.send_bytes(42, "Let's start");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        assert!(res.log().is_empty());

        let res = program.send_bytes(69, "Gear");
        assert!(res.log().is_empty());
    }
}
