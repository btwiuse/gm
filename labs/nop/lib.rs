#![no_std]

#[no_mangle]
unsafe extern "C" fn handle() {
    // without this line test panics
    gstd::msg::load_bytes();
}

#[no_mangle]
unsafe extern "C" fn init() {}

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
