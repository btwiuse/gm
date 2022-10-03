//! contract tests

use crate::*;

#[cfg(test)]
#[test]
fn mint_works() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount
    assert_eq!(contract.balance_of(1, 2), 3); // who, token

    contract.mint(1, 2, 3);
    assert_eq!(contract.balance_of(1, 2), 6);
}

#[test]
fn mint_batch_works() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]); // to, token, amount
    assert_eq!(contract.balance_of(1, 0), 1); // who, token
    assert_eq!(contract.balance_of(1, 1), 2); // who, token
    assert_eq!(contract.balance_of(1, 2), 3); // who, token

    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]);
    assert_eq!(contract.balance_of(1, 0), 2); // who, token
    assert_eq!(contract.balance_of(1, 1), 4); // who, token
    assert_eq!(contract.balance_of(1, 2), 6); // who, token
}

#[test]
fn balance_of_works() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    assert_eq!(contract.balance_of(1, 2), 0); // who, token

    contract.mint(1, 2, 3); // to, token, amount
    assert_eq!(contract.balance_of(1, 2), 3); // who, token

    contract.mint(1, 2, 3);
    assert_eq!(contract.balance_of(1, 2), 6);
}

#[test]
fn balance_of_batch_works() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    assert_eq!(
        contract.balance_of_batch(vec![1, 1, 1], vec![0, 1, 2]),
        vec![0, 0, 0]
    ); // who, token

    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]); // to, token, amount
    assert_eq!(
        contract.balance_of_batch(vec![1, 1, 1], vec![0, 1, 2]),
        vec![1, 2, 3]
    ); // who, token

    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]); // to, token, amount
    assert_eq!(
        contract.balance_of_batch(vec![1, 1, 1], vec![0, 1, 2]),
        vec![2, 4, 6]
    ); // who, token
}

#[test]
fn transfer_works() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount

    contract.safe_transfer_from(1, 42, 2, 3); // from, to, token, amount
    assert_eq!(contract.balance_of(1, 2), 0);
    assert_eq!(contract.balance_of(42, 2), 3);
}

#[test]
fn burn_works() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount

    contract.burn(1, 2, 3); // from, token, amount
    assert_eq!(contract.balance_of(1, 2), 0);
}

#[test]
fn default_approval_for_all_is_false() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount

    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert_eq!(approved, false);
}

#[test]
fn set_approval_for_all_works() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount

    contract.set_approval_for_all(1, 42, true); // owner, operator
    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert_eq!(approved, true);

    contract.set_approval_for_all(1, 42, false); // owner, operator
    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert_eq!(approved, false);
}

#[test]
fn default_token_metadata_is_none() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount

    let metadata = contract.get_token_metadata(2);
    assert!(metadata.is_none());
}

#[test]
fn update_token_metadata_works() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount

    let some_metadata = Some(TokenMetadata {
        name: "nft".to_string(),
        description: "nft for test".to_string(),
        image_uri: "https://gm.dev/nft.png".to_string(),
        json_uri: "https://gm.dev/nft.json".to_string(),
    });

    contract.update_token_metadata(2, some_metadata.clone());
    let metadata = contract.get_token_metadata(2);
    assert_eq!(metadata, some_metadata);

    contract.update_token_metadata(2, None);
    let metadata = contract.get_token_metadata(2);
    assert_eq!(metadata, None);
}
