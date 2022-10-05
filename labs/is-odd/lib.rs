#![no_std]

use codec::Encode;
use gstd::msg;
use gstd::prelude::*;

#[no_mangle]
unsafe extern "C" fn handle() {
    // without this line test panics
    gstd::msg::load_bytes();
}

gstd::metadata! {
    title: "is_odd?",
    state:
        input: u32,
        output: bool,
}

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let input: u32 = msg::load().expect("failed to decode input argument");
    let encoded = (input & 1 != 0).encode();
    /*
    match person {
        None => WALLETS.encode(),
        Some(lookup_id) => WALLETS
            .iter()
            .filter(|w| w.id == lookup_id)
            .cloned()
            .collect::<Vec<Wallet>>()
            .encode(),
    };
    */
    gstd::util::to_leak_ptr(encoded)
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
