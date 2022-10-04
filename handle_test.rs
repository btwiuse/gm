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

    let expected = Event::TransferSingle {
        operator: ActorId::from(42),
        from: ActorId::zero(),
        to: ActorId::from(42),
        token: 0,
        amount: 1,
    };

    let res = program.send(42, mint_msg.clone());
    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}

#[test]
fn mint_twice_panics() {
    use super::*;
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

    program.send(42, mint_msg.clone());
    program.send(42, mint_msg.clone()); // should panic
                                        // TODO how to assert panic in tests?
                                        // panic!("this line shouldn't appear in cargo test result");
                                        //
                                        // TODO why this panics?
                                        // debug!("this line shouldn't appear in cargo test result");
}

#[test]
fn mint_batch_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = Init {
        name: "gm".to_string(),
        symbol: "GM".to_string(),
        base_uri: "https://gm.dev/{}".to_string(),
    };
    let _res = program.send(42, init_msg);

    let mint_msg = Input::MintBatch {
        to: ActorId::from(42),
        token: vec![0, 1, 2, 3],
        amount: vec![1, 2, 3, 4],
    };

    let expected = Event::TransferBatch {
        operator: ActorId::from(42),
        from: ActorId::zero(),
        to: ActorId::from(42),
        token: vec![0, 1, 2, 3],
        amount: vec![1, 2, 3, 4],
    };

    let res = program.send(42, mint_msg.clone());
    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}
