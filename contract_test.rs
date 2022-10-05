//! contract tests

use crate::*;

#[cfg(test)]
#[test]
fn mint_works() {
    let expected = BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]);
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount
    assert_eq!(contract.balances, expected);
}

#[test]
#[should_panic]
fn mint_twice_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount
    contract.mint(1, 2, 3);
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn mint_batch_works() {
    let expected = BTreeMap::from([
        (0, BTreeMap::from([(1, 1)])),
        (1, BTreeMap::from([(1, 2)])),
        (2, BTreeMap::from([(1, 3)])),
    ]);
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]); // to, token, amount
    assert_eq!(contract.balances, expected);
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
fn balance_of_works() {
    let contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 1)])),
            (1, BTreeMap::from([(1, 2)])),
            (2, BTreeMap::from([(1, 3)])),
        ]),
        ..Default::default()
    };

    assert_eq!(contract.balance_of(1, 0), 1); // who, token
    assert_eq!(contract.balance_of(1, 1), 2); // who, token
    assert_eq!(contract.balance_of(1, 2), 3); // who, token
    assert_eq!(contract.balance_of(1, 3), 0); // who, token
}

#[test]
fn balance_of_batch_works() {
    let contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 1)])),
            (1, BTreeMap::from([(1, 2)])),
            (2, BTreeMap::from([(1, 3)])),
        ]),
        ..Default::default()
    };

    assert_eq!(
        contract.balance_of_batch(vec![1, 1, 1, 1], vec![0, 1, 2, 3]),
        vec![1, 2, 3, 0]
    ); // who, token
}

#[test]
fn transfer_works() {
    let expected = BTreeMap::from([(0, BTreeMap::from([(1, 0), (42, 1)]))]);
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(0, BTreeMap::from([(1, 1)]))]),
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.safe_transfer_from(1, 42, 0, 1); // from, to, token, amount
    assert_eq!(contract.balances, expected);
}

#[test]
fn transfer_batch_works() {
    let expected = BTreeMap::from([
        (0, BTreeMap::from([(1, 0), (42, 3)])),
        (1, BTreeMap::from([(1, 2), (42, 2)])),
        (2, BTreeMap::from([(1, 4), (42, 1)])),
    ]);
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 3)])),
            (1, BTreeMap::from([(1, 4)])),
            (2, BTreeMap::from([(1, 5)])),
        ]),
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.safe_batch_transfer_from(1, 42, vec![0, 1, 2], vec![3, 2, 1]); // from, to, token, amount

    assert_eq!(contract.balances, expected);
}

#[test]
#[should_panic]
fn transfer_exceeding_balance_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(0, BTreeMap::from([(1, 3)]))]),
        env: MockConfig {
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
fn transfer_batch_exceeding_balance_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 3)])),
            (1, BTreeMap::from([(1, 4)])),
            (2, BTreeMap::from([(1, 5)])),
        ]),
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.safe_batch_transfer_from(1, 42, vec![0, 1, 2], vec![4, 2, 1]); // from, to, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn burn_works() {
    let expected = BTreeMap::from([(2, BTreeMap::from([(1, 0)]))]);
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]),
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.burn(1, 2, 3); // from, token, amount
    assert_eq!(contract.balances, expected);
}

#[test]
#[should_panic]
fn burn_from_non_owner_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]),
        env: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.burn(1, 2, 3); // from, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn burn_from_approved_works() {
    let expected = BTreeMap::from([(2, BTreeMap::from([(1, 0)]))]);
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        approvals: BTreeMap::from([(1, BTreeMap::from([(42, true)]))]),
        balances: BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]),
        env: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.burn(1, 2, 3); // from, token, amount
    assert_eq!(contract.balances, expected);
}

#[test]
#[should_panic]
fn burn_exceeding_balance_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]),
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.burn(1, 2, 4); // from, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn burn_batch_works() {
    let expected = BTreeMap::from([
        (0, BTreeMap::from([(1, 1)])),
        (1, BTreeMap::from([(1, 2)])),
        (2, BTreeMap::from([(1, 3)])),
    ]);

    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 4)])),
            (1, BTreeMap::from([(1, 5)])),
            (2, BTreeMap::from([(1, 6)])),
        ]),
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.burn_batch(1, vec![0, 1, 2], vec![3, 3, 3]); // from, token, amount
    assert_eq!(contract.balances, expected);
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
        env: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.burn_batch(1, vec![0, 1, 2], vec![5, 5, 5]); // from, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn burn_batch_from_approved_works() {
    let expected = BTreeMap::from([
        (0, BTreeMap::from([(1, 0)])),
        (1, BTreeMap::from([(1, 1)])),
        (2, BTreeMap::from([(1, 2)])),
    ]);
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        approvals: BTreeMap::from([(1, BTreeMap::from([(42, true)]))]),
        balances: BTreeMap::from([
            (0, BTreeMap::from([(1, 4)])),
            (1, BTreeMap::from([(1, 5)])),
            (2, BTreeMap::from([(1, 6)])),
        ]),
        env: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.burn_batch(1, vec![0, 1, 2], vec![4, 4, 4]); // from, token, amount
    assert_eq!(contract.balances, expected);
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
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.burn_batch(1, vec![0, 1, 2], vec![5, 5, 5]); // from, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn default_approval_for_all_is_false() {
    let contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert!(!approved);
}

#[test]
#[should_panic]
fn set_approval_for_all_from_non_owner_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        env: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };
    contract.set_approval_for_all(1, 42, true); // owner, operator
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn is_approved_for_all_works() {
    let contract: Contract<MockConfig> = Contract::<MockConfig> {
        approvals: BTreeMap::from([(1, BTreeMap::from([(42, true)]))]),
        ..Default::default()
    };

    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert!(approved);
}

#[test]
fn set_approval_for_all_from_sender_works() {
    let expected = BTreeMap::from([(1, BTreeMap::from([(42, false)]))]);
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        env: MockConfig {
            sender: 1,
            origin: 0,
        },
        approvals: BTreeMap::from([(1, BTreeMap::from([(42, true)]))]),
        ..Default::default()
    };

    contract.set_approval_for_all(1, 42, false); // owner, operator
    assert_eq!(contract.approvals, expected);
}

#[test]
fn set_approval_for_all_works() {
    let expected = BTreeMap::from([(1, BTreeMap::from([(42, false)]))]);
    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        approvals: BTreeMap::from([(1, BTreeMap::from([(42, true)]))]),
        ..Default::default()
    };

    contract.set_approval_for_all(1, 42, false); // owner, operator
    assert_eq!(contract.approvals, expected);
}

#[test]
fn default_token_metadata_is_none() {
    let contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    let metadata = contract.get_token_metadata(2);
    assert!(metadata.is_none());
}

#[test]
fn update_token_metadata_works() {
    let some_metadata = Some(TokenMetadata {
        name: "nft".to_string(),
        description: "nft for test".to_string(),
        image_uri: "https://gm.dev/nft.png".to_string(),
        json_uri: "https://gm.dev/nft.json".to_string(),
    });

    let expected = BTreeMap::from([(2, some_metadata.clone().unwrap())]);

    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]),
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        ..Default::default()
    };

    contract.update_token_metadata(2, some_metadata.clone());
    assert_eq!(contract.metadata_registry, expected);
}

#[test]
fn remove_update_token_metadata_works() {
    let some_metadata = Some(TokenMetadata {
        name: "nft".to_string(),
        description: "nft for test".to_string(),
        image_uri: "https://gm.dev/nft.png".to_string(),
        json_uri: "https://gm.dev/nft.json".to_string(),
    });

    let expected = BTreeMap::from([]);

    let mut contract: Contract<MockConfig> = Contract::<MockConfig> {
        balances: BTreeMap::from([(2, BTreeMap::from([(1, 3)]))]),
        env: MockConfig {
            sender: 1,
            origin: 1,
        },
        metadata_registry: BTreeMap::from([(2, some_metadata.clone().unwrap())]),
        ..Default::default()
    };

    contract.update_token_metadata(2, None);
    assert_eq!(contract.metadata_registry, expected);
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
        env: MockConfig {
            sender: 42,
            origin: 42,
        },
        ..Default::default()
    };

    contract.update_token_metadata(2, some_metadata.clone());
    panic!("this line shouldn't appear in cargo test result");
}
