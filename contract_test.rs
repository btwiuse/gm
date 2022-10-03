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
fn approval_works() {
    let mut contract: Contract<TestConfig> = Contract::<TestConfig>::default();
    contract.mint(1, 2, 3); // to, token, amount

    contract.burn(1, 2, 3); // from, token, amount
    assert_eq!(contract.balance_of(1, 2), 0);
}
