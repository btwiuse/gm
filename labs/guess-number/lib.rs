#![no_std]

use gstd::{debug, msg, prelude::*};

static mut SECRET: u8 = 0;

#[no_mangle]
unsafe extern "C" fn handle() {
    let payload = msg::load_bytes();
    debug!("handle() {:?}", payload);
    let number: u8 = payload.first().copied().unwrap_or(0);
    debug!("handle(number = {:?})", number);
    let result = match number {
        n if n > SECRET => "-",
        n if n < SECRET => "+",
        _ => "=",
    };
    debug!("handle(result = {:?})", result);
    msg::reply_bytes(result, 0).expect("Failed to reply");
    // msg::reply_bytes(compare, 0).expect("Failed to reply");
    /*
    if number.is_none() || SECRET.is_none() {
        msg::reply_bytes([0xFF], 0).expect("Failed to reply");
    } else {
    }
    let secret = SECRET.unwrap();
    let number = *number.unwrap();

    debug!("handle(): {}", number);
    if number > secret {
        msg::reply_bytes("-", 0).expect("Failed to reply");
    }
    if number < secret {
        msg::reply_bytes("+", 0).expect("Failed to reply");
    }
    if number == secret {
        msg::reply_bytes("=", 0).expect("Failed to reply");
    }
    */
}

#[no_mangle]
unsafe extern "C" fn init() {
    let payload = msg::load_bytes();
    debug!("init() {:?}", payload);
    if let Some(number) = payload.first() {
        SECRET = *number;
        debug!("init(SECRET = {})", number);
    }
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

        let res = program.send_bytes(42, "X");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "A");
        assert!(!res.log().is_empty());

        let log = Log::builder().source(1).dest(42).payload_bytes("+");
        assert!(res.contains(&log));

        let res = program.send_bytes(69, "Z");
        assert!(!res.log().is_empty());

        let log = Log::builder().source(1).dest(69).payload_bytes("-");
        assert!(res.contains(&log));

        let res = program.send_bytes(123, "X");
        assert!(!res.log().is_empty());

        let log = Log::builder().source(1).dest(123).payload_bytes("=");
        assert!(res.contains(&log));
    }

    #[test]
    fn it_works_empty() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(42, "");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "");
        assert!(!res.log().is_empty());
    }
}
