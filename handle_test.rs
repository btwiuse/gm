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

    let mint_msg = Action::Mint {
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

    let res = program.send(42, mint_msg);
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

    let mint_msg = Action::Mint {
        to: ActorId::from(42),
        token: 0,
        amount: 1,
    };

    program.send(42, mint_msg.clone());
    // should panic:
    program.send(42, mint_msg);

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

    let mint_msg = Action::MintBatch {
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

    let res = program.send(42, mint_msg);
    assert_eq!(res.log().len(), 1);
    assert_eq!(res.log()[0].payload(), expected.encode());
}

#[test]
fn burn_empty_panics() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    program.send(42, Init::default());

    let _res = program.send(
        42,
        Action::Burn {
            from: ActorId::from(42),
            token: 0,
            amount: 1,
        },
    );
    // assert!(res.log().is_empty());
}

#[test]
fn burn_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    program.send(42, Init::default());

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
fn update_token_metadata_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    program.send(42, Init::default());

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
