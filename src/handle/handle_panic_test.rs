//! contract tests

use crate::*;

#[cfg(test)]
use gtest::{Program, System};

#[cfg(test)]
pub fn init_program(prog: &Program) {
    prog.send(
        42,
        Init {
            name: "gm".to_string(),
            symbol: "GM".to_string(),
            base_uri: "https://gm.dev/{}".to_string(),
        },
    );
}

#[test]
fn mint_twice_panics() {
    use super::*;
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    let _ = program.send(
        42,
        Action::Mint {
            to: ActorId::from(42),
            token: 0,
            amount: 1,
        },
    );

    let res = program.send(
        42,
        Action::Mint {
            to: ActorId::from(42),
            token: 0,
            amount: 1,
        },
    );

    assert!(res.main_failed());
}

#[test]
fn mint_zero_panics() {
    use super::*;
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    let res = program.send(
        42,
        Action::Mint {
            to: ActorId::from(42),
            token: 0,
            amount: 0,
        },
    );

    assert!(res.main_failed());
}

#[test]
fn burn_exceeding_balance_panics() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

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

#[test]
fn burn_zero_panics() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    let res = program.send(
        42,
        Action::Burn {
            from: ActorId::from(42),
            token: 0,
            amount: 0,
        },
    );

    assert!(res.main_failed());
}

#[test]
fn transfer_zero_panics() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    let res = program.send(
        42,
        Action::TransferFrom {
            from: ActorId::from(42),
            to: ActorId::from(1),
            token: 0,
            amount: 0,
        },
    );

    assert!(res.main_failed());
}

#[test]
fn transfer_exceeding_balance_panics() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    let res = program.send(
        42,
        Action::TransferFrom {
            from: ActorId::from(42),
            to: ActorId::from(1),
            token: 0,
            amount: 1,
        },
    );

    assert!(res.main_failed());
}

#[test]
fn transfer_from_non_owner_panics() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    program.send(
        42,
        Action::Mint {
            to: ActorId::from(42),
            token: 0,
            amount: 1,
        },
    );

    let res = program.send(
        69,
        Action::TransferFrom {
            from: ActorId::from(42),
            to: ActorId::from(69),
            token: 0,
            amount: 1,
        },
    );

    assert!(res.main_failed());
}

#[test]
fn transfer_batch_length_mismatch_panics() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    program.send(
        42,
        Action::MintBatch {
            to: ActorId::from(42),
            token: vec![0, 1, 2, 3],
            amount: vec![1, 2, 3, 4],
        },
    );

    let res = program.send(
        42,
        Action::BatchTransferFrom {
            from: ActorId::from(42),
            to: ActorId::from(1),
            token: vec![0, 1, 2, 3],
            amount: vec![1, 2, 3],
        },
    );

    assert!(res.main_failed());
}

#[test]
fn burn_batch_length_mismatch_panics() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    program.send(
        42,
        Action::MintBatch {
            to: ActorId::from(42),
            token: vec![0, 1, 2, 3],
            amount: vec![1, 2, 3, 4],
        },
    );

    let res = program.send(
        42,
        Action::BurnBatch {
            from: ActorId::from(42),
            token: vec![0, 1, 2, 3],
            amount: vec![1, 1, 1],
        },
    );

    assert!(res.main_failed());
}

#[test]
fn transfer_batch_length_mismatch_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    program.send(
        42,
        Action::MintBatch {
            to: ActorId::from(42),
            token: vec![0, 1, 2, 3],
            amount: vec![1, 2, 3, 4],
        },
    );

    let res = program.send(
        42,
        Action::BatchTransferFrom {
            from: ActorId::from(42),
            to: ActorId::from(1),
            token: vec![0, 1, 2, 3],
            amount: vec![1, 2, 3],
        },
    );

    assert!(res.main_failed());
}
