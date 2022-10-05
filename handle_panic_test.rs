//! contract tests

use crate::*;

#[cfg(test)]
use gtest::{Program, System};

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

    let mint_msg = Action::Mint {
        to: ActorId::from(42),
        token: 0,
        amount: 1,
    };

    program.send(42, mint_msg.clone());
    let res = program.send(42, mint_msg);

    assert!(res.main_failed());
}

#[test]
fn burn_empty_panics() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    program.send(42, Init::default());

    let res = program.send(
        42,
        Action::Burn {
            from: ActorId::from(42),
            token: 0,
            amount: 1,
        },
    );

    assert!(res.main_failed());
}
