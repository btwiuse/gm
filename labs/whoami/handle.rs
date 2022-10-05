use crate::codec::Output;
use crate::state::STATE;
use gstd::{debug, msg, prelude::*, ActorId};

#[no_mangle]
unsafe extern "C" fn handle() {
    let id: ActorId = msg::source();
    debug!("id: {:?}", id);
    msg::reply_bytes(Output::Payload(id).encode(), 0).expect("Failed to reply");
    STATE.push(id)
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

        let res = program.send_bytes(42, "Let's start");
        assert!(res.log().is_empty());

        let res = program.send_bytes(42, "Hello");
        assert!(res.log().len() == 1);
        let addr = res.log()[0].payload();
        assert!(addr.starts_with(&[42u8]));
        assert!(addr.ends_with(&[0u8; 31]));
        let who = ActorId::from_slice(addr);
        assert!(who.is_ok());

        let res = program.send_bytes(69, "Gear");
        assert!(res.log().len() == 1);
        let addr = res.log()[0].payload();
        assert!(addr.starts_with(&[69u8]));
        assert!(addr.ends_with(&[0u8; 31]));
        let who = ActorId::from_slice(addr);
        assert!(who.is_ok());
    }
}
