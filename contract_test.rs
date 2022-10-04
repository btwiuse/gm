//! contract tests

use crate::*;

#[cfg(test)]
#[test]
fn mint_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint(1, 2, 3); // to, token, amount
    assert_eq!(contract.balance_of(1, 2), 3); // who, token
}

#[test]
#[should_panic]
fn mint_twice_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint(1, 2, 3); // to, token, amount
    contract.mint(1, 2, 3);
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn mint_batch_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]); // to, token, amount
    assert_eq!(contract.balance_of(1, 0), 1); // who, token
    assert_eq!(contract.balance_of(1, 1), 2); // who, token
    assert_eq!(contract.balance_of(1, 2), 3); // who, token
}

#[test]
#[should_panic]
fn mint_batch_twice_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]); // to, token, amount
    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]);
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn balance_of_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);

    assert_eq!(contract.balance_of(1, 2), 0); // who, token

    contract.mint(1, 2, 3); // to, token, amount
    assert_eq!(contract.balance_of(1, 2), 3); // who, token
}

#[test]
fn balance_of_batch_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);

    assert_eq!(
        contract.balance_of_batch(vec![1, 1, 1], vec![0, 1, 2]),
        vec![0, 0, 0]
    ); // who, token

    contract.mint_batch(1, vec![0, 1, 2], vec![1, 2, 3]); // to, token, amount
    assert_eq!(
        contract.balance_of_batch(vec![1, 1, 1], vec![0, 1, 2]),
        vec![1, 2, 3]
    ); // who, token
}

#[test]
fn transfer_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint(1, 2, 3); // to, token, amount

    contract.safe_transfer_from(1, 42, 2, 3); // from, to, token, amount
    assert_eq!(contract.balance_of(1, 2), 0);
    assert_eq!(contract.balance_of(42, 2), 3);
}

#[test]
fn transfer_batch_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint_batch(1, vec![0, 1, 2], vec![3, 4, 5]); // to, token, amount

    contract.safe_batch_transfer_from(1, 42, vec![0, 1, 2], vec![3, 2, 1]); // from, to, token, amount
    assert_eq!(contract.balance_of(1, 0), 0);
    assert_eq!(contract.balance_of(1, 1), 2);
    assert_eq!(contract.balance_of(1, 2), 4);
    assert_eq!(contract.balance_of(42, 0), 3);
    assert_eq!(contract.balance_of(42, 1), 2);
    assert_eq!(contract.balance_of(42, 2), 1);
}

#[test]
#[should_panic]
fn transfer_exceeding_balance_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint(1, 0, 3); // to, token, amount

    contract.safe_transfer_from(1, 42, 0, 4); // from, to, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
#[should_panic]
fn transfer_batch_exceeding_balance_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint_batch(1, vec![0, 1, 2], vec![3, 4, 5]); // to, token, amount

    contract.safe_batch_transfer_from(1, 42, vec![0, 1, 2], vec![4, 2, 1]); // from, to, token, amount
    panic!("this line shouldn't appear in cargo test result");
}

#[test]
fn burn_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint(1, 2, 3); // to, token, amount

    contract.burn(1, 2, 3); // from, token, amount
    assert_eq!(contract.balance_of(1, 2), 0);
}

#[test]
fn default_approval_for_all_is_false() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint(1, 2, 3); // to, token, amount

    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert_eq!(approved, false);
}

#[test]
#[should_panic]
fn set_approval_for_all_from_non_owner_panics() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(42, 42);
    contract.set_approval_for_all(1, 42, true); // owner, operator
}

#[test]
fn set_approval_for_all_from_origin_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(0, 1);

    contract.set_approval_for_all(1, 42, true); // owner, operator
    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert_eq!(approved, true);

    contract.set_approval_for_all(1, 42, false); // owner, operator
    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert_eq!(approved, false);
}

#[test]
fn set_approval_for_all_from_sender_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 0);

    contract.set_approval_for_all(1, 42, true); // owner, operator
    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert_eq!(approved, true);

    contract.set_approval_for_all(1, 42, false); // owner, operator
    let approved = contract.is_approved_for_all(1, 42); // owner, operator
    assert_eq!(approved, false);
}

#[test]
fn set_approval_for_all_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
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
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
    contract.mint(1, 2, 3); // to, token, amount

    let metadata = contract.get_token_metadata(2);
    assert!(metadata.is_none());
}

#[test]
fn update_token_metadata_works() {
    let mut contract: Contract<MockConfig> = Contract::<MockConfig>::default();
    contract.env.set_sender_origin(1, 1);
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
