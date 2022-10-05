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
fn mint_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    let expected = Event::TransferSingle {
        operator: ActorId::from(42),
        from: ActorId::zero(),
        to: ActorId::from(42),
        token: 0,
        amount: 1,
    };

    let res = program.send(
        42,
        Action::Mint {
            to: ActorId::from(42),
            token: 0,
            amount: 1,
        },
    );

    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}

#[test]
fn mint_batch_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);
    init_program(&program);

    let expected = Event::TransferBatch {
        operator: ActorId::from(42),
        from: ActorId::zero(),
        to: ActorId::from(42),
        token: vec![0, 1, 2, 3],
        amount: vec![1, 2, 3, 4],
    };

    let res = program.send(
        42,
        Action::MintBatch {
            to: ActorId::from(42),
            token: vec![0, 1, 2, 3],
            amount: vec![1, 2, 3, 4],
        },
    );

    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}

#[test]
fn transfer_works() {
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
        42,
        Action::TransferFrom {
            from: ActorId::from(42),
            to: ActorId::from(1),
            token: 0,
            amount: 1,
        },
    );

    let expected = Event::TransferSingle {
        operator: ActorId::from(42),
        from: ActorId::from(42),
        to: ActorId::from(1),
        token: 0,
        amount: 1,
    };

    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}

#[test]
fn transfer_batch_works() {
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
            amount: vec![1, 2, 3, 4],
        },
    );

    let expected = Event::TransferBatch {
        operator: ActorId::from(42),
        from: ActorId::from(42),
        to: ActorId::from(1),
        token: vec![0, 1, 2, 3],
        amount: vec![1, 2, 3, 4],
    };

    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}

#[test]
fn burn_works() {
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
        42,
        Action::Burn {
            from: ActorId::from(42),
            token: 0,
            amount: 1,
        },
    );

    let expected = Event::TransferSingle {
        operator: ActorId::from(42),
        from: ActorId::from(42),
        to: ActorId::zero(),
        token: 0,
        amount: 1,
    };

    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}

#[test]
fn burn_batch_works() {
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
            amount: vec![1, 1, 1, 1],
        },
    );

    let expected = Event::TransferBatch {
        operator: ActorId::from(42),
        from: ActorId::from(42),
        to: ActorId::zero(),
        token: vec![0, 1, 2, 3],
        amount: vec![1, 1, 1, 1],
    };

    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}

#[test]
fn update_token_metadata_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    program.send(42, Init::default());
    init_program(&program);

    program.send(
        42,
        Action::Mint {
            to: ActorId::from(42),
            token: 0,
            amount: 1,
        },
    );

    let some_metadata = Some(TokenMetadata {
        name: "nft".to_string(),
        description: "nft for test".to_string(),
        image_uri: "https://gm.dev/nft.png".to_string(),
        json_uri: "https://gm.dev/nft.json".to_string(),
    });

    let res = program.send(
        42,
        Action::UpdateTokenMetadata {
            token: 0,
            metadata: some_metadata.clone(),
        },
    );

    let expected = Event::UpdateTokenMetadata {
        token: 0,
        metadata: some_metadata,
    };

    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}

#[test]
fn set_approval_for_all_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    program.send(42, Init::default());
    init_program(&program);

    let res = program.send(
        42,
        Action::SetApprovalForAll {
            operator: ActorId::from(1),
            approved: true,
        },
    );

    let expected = Event::ApprovedForAll {
        owner: ActorId::from(42),
        operator: ActorId::from(1),
        approved: true,
    };

    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}
