//! contract tests

use crate::*;
use config::*;

#[test]
#[should_panic]
fn mint_twice_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount
    contract.mint(1, 2, 3);
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn mint_batch_twice_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]); // to, token, amount
    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]);
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn balance_of_batch_length_mismatch_panics() {
    let contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 1)])),
            (1, BTreeMap::from([(1, 2)])),
            (2, BTreeMap::from([(1, 3)])),
        ]),
        ..Default::default()
    };

    contract.balance_of_batch(vec![1, 1, 1], vec![0, 1, 2, 3]);
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn transfer_from_non_owner_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(0, BTreeMap::from([(1, 1)]))]),
        ctx: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.safe_transfer_from(1, 42, 0, 1); // from, to, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn transfer_exceeding_balance_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(0, BTreeMap::from([(1, 3)]))]),
        ctx: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.safe_transfer_from(1, 42, 0, 4); // from, to, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn transfer_batch_from_non_owner_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(0, BTreeMap::from([(1, 1)]))]),
        ctx: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.safe_batch_transfer_from(1, 42, vec![0], vec![1]); // from, to, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn transfer_batch_exceeding_balance_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 3)])),
            (1, BTreeMap::from([(1, 4)])),
            (2, BTreeMap::from([(1, 5)])),
        ]),
        ctx: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.safe_batch_transfer_from(1, 42, vec![0, 1, 2], vec![4, 2, 1]); // from, to, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn transfer_batch_length_mismatch_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 1)])),
            (1, BTreeMap::from([(1, 2)])),
            (2, BTreeMap::from([(1, 3)])),
        ]),
        ..Default::default()
    };

    contract.safe_batch_transfer_from(1, 42, vec![0, 1, 2], vec![0, 1, 2, 3]);
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn burn_from_non_owner_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]),
        ctx: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.burn(1, 2, 3); // from, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn burn_exceeding_balance_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]),
        ctx: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.burn(1, 2, 4); // from, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn burn_batch_length_mismatch_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 1)])),
            (1, BTreeMap::from([(1, 2)])),
            (2, BTreeMap::from([(1, 3)])),
        ]),
        ..Default::default()
    };

    contract.burn_batch(1, vec![0, 1, 2], vec![0, 1, 2, 3]);
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn burn_batch_from_non_owner_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 4)])),
            (1, BTreeMap::from([(1, 5)])),
            (2, BTreeMap::from([(1, 6)])),
        ]),
        ctx: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.burn_batch(1, vec![0, 1, 2], vec![5, 5, 5]); // from, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn burn_batch_exceeding_balance_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 4)])),
            (1, BTreeMap::from([(1, 5)])),
            (2, BTreeMap::from([(1, 6)])),
        ]),
        ctx: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.burn_batch(1, vec![0, 1, 2], vec![5, 5, 5]); // from, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn set_approval_for_all_from_non_owner_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        ctx: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };
    contract.set_approval_for_all(1, 42, true); // owner, operator
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn update_token_metadata_from_non_owner_panics() {
    let some_metadata = Some(TokenMetadata {
        name: "nft".to_string(),
        description: "nft for test".to_string(),
        image_uri: "https://gm.dev/nft.png".to_string(),
        json_uri: "https://gm.dev/nft.json".to_string(),
    });

    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]),
        ctx: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.update_token_metadata(2, some_metadata);
    panic!("this line shouldn't appear in cargo test result");
}
