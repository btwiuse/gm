#![no_std]

use codec::Decode;
use codec::Encode;
use gstd::{debug, msg, prelude::*};
use scale_info::TypeInfo;

static mut PAYLOADS: Vec<String> = vec![];

gstd::metadata! {
    title: "counter-states",
    init:
        input: String,
    handle:
        input: String,
        output: String,
    state:
        input: StateQuery,
        output: StateReply,
}

#[derive(Debug, TypeInfo, Decode)]
pub enum StateQuery {
    All,
    Len,
    StrLen(String),
    Nth(String, u8),
}

#[derive(Debug, TypeInfo, Encode)]
pub enum StateReply {
    All(Vec<String>),
    Len(u32),
    Char(u8),
}

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: StateQuery = msg::load().expect("failed to decode input argument");
    // PAYLOADS.push(input);
    let encoded = match query {
        StateQuery::All => StateReply::All(PAYLOADS.clone()).encode(),
        StateQuery::Len => StateReply::Len(PAYLOADS.len() as u32).encode(),
        StateQuery::StrLen(s) => StateReply::Len(s.len() as u32).encode(),
        StateQuery::Nth(s, n) => {
            StateReply::Char(s.chars().nth(n as usize).unwrap_or('?') as u8).encode()
        }
    };
    gstd::util::to_leak_ptr(encoded)
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let payload = String::from_utf8(msg::load_bytes()).expect("Invalid handle message");
    debug!("handle(): {}", payload);

    msg::reply_bytes(format!("{}", PAYLOADS.len()), 0).expect("Failed to reply");
    PAYLOADS.push(payload);

    debug!("got {:?} payloads so far:", PAYLOADS.len());
    PAYLOADS
        .iter()
        .enumerate()
        .for_each(|(i, x)| debug!("PAYLOADS[{}] = {}", i, x))
}

#[no_mangle]
unsafe extern "C" fn init() {
    let payload = String::from_utf8(msg::load_bytes()).expect("Invalid init message");
    debug!("init(): {}", payload);
    PAYLOADS.push(payload);
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
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        assert!(!res.log().is_empty());

        let log = Log::builder().source(1).dest(42).payload_bytes("0");
        assert!(res.contains(&log));

        let res = program.send_bytes(69, "Gear");
        assert!(!res.log().is_empty());

        let log = Log::builder().source(1).dest(69).payload_bytes("1");
        assert!(res.contains(&log));
    }

    #[test]
    fn it_works_too() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "Let's start");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        assert!(!res.log().is_empty());

        let log = Log::builder().source(1).dest(42).payload_bytes("0");
        assert!(res.contains(&log));

        let res = program.send_bytes(69, "Gear");
        assert!(!res.log().is_empty());

        let log = Log::builder().source(1).dest(69).payload_bytes("1");
        assert!(res.contains(&log));
    }
}
