//! contract tests

use crate::*;

#[cfg(test)]
use gtest::{Program, System};

#[test]
fn mint_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = Init {
        name: "gm".to_string(),
        symbol: "GM".to_string(),
        base_uri: "https://gm.dev/{}".to_string(),
    };
    let _res = program.send(42, init_msg);

    let mint_msg = Input::Mint {
        to: ActorId::from(42),
        token: 0,
        amount: 1,
    };

    let res = program.send(42, mint_msg.clone());
    assert!(!res.log().is_empty());
    let res = program.send(42, mint_msg);
    assert!(!res.log().is_empty());
    /*
    let name_query = Query::Name;
    let res = program.send(42, name_query);
    assert!(!res.log().is_empty());
    */

    /*
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
    */
}
