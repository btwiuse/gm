//! contract implementation

use crate::*;

/// ERC1155Check interface
impl<T: IConfig> IERC1155Check<T> for Contract<T> {
    fn check_transfer_from(
        &self,
        from: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    ) {
        if amount.is_zero() {
            panic!("check failed: cannot transfer 0 amount")
        }
        if from != self.sender() && !self.is_approved_for_all(from, self.sender()) {
            panic!("check failed: needs approval")
        }
        if self.balance_of(from, token) < amount {
            panic!("check failed: insufficient balance")
        }
        if from == to {
            panic!("check failed: self transfer not permitted")
        }
        if to.is_zero() {
            panic!("check failed: transfer to black hole not permitted")
        }
    }
    fn check_batch_transfer_from(
        &self,
        from: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        if token.len() != amount.len() {
            panic!("check failed: token and amount length mismatch")
        }
        for (tk, am) in token.iter().zip(amount) {
            self.check_transfer_from(from, to, *tk, am)
        }
    }
    fn check_mint(&self, to: T::AccountId, token: T::TokenId, amount: T::AccountBalance) {
        if to.is_zero() {
            panic!("check failed: cannot mint to black hole address")
        }
        if amount.is_zero() {
            panic!("check failed: cannot mint 0 amount")
        }
        if self.balances.contains_key(&token) {
            panic!("check failed: cannot mint twice")
        }
    }
    fn check_mint_batch(
        &self,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        if token.len() != amount.len() {
            panic!("check failed: token and amount length mismatch")
        }
        for (tk, am) in token.iter().zip(amount) {
            self.check_mint(to, *tk, am)
        }
    }
    fn check_balance_of_batch(&self, who: Vec<T::AccountId>, token: Vec<T::TokenId>) {
        if who.len() != token.len() {
            panic!("check failed: token and account length mismatch")
        }
    }
    fn check_set_approval_for_all(
        &self,
        owner: T::AccountId,
        _operator: T::AccountId,
        _approved: bool,
    ) {
        if owner != self.sender() {
            panic!("check failed: sender is not account owner")
        }
    }
    fn check_burn(&self, from: T::AccountId, token: T::TokenId, amount: T::AccountBalance) {
        if amount.is_zero() {
            panic!("check failed: cannot burn 0 amount")
        }
        if from != self.sender() && !self.is_approved_for_all(from, self.sender()) {
            panic!("check failed: needs approval")
        }
        if self.balance_of(from, token) < amount {
            panic!("check failed: insufficient balance")
        }
    }
    fn check_burn_batch(
        &self,
        from: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        if token.len() != amount.len() {
            panic!("check failed: token and amount length mismatch")
        }
        for (tk, am) in token.iter().zip(amount) {
            self.check_burn(from, *tk, am)
        }
    }
    // allow owner of token to update metadata
    fn check_update_token_metadata(&self, token: T::TokenId, _metadata: Option<TokenMetadata>) {
        if !self.balances.contains_key(&token) {
            panic!("check failed: no such token")
        }
        if self.balance_of(self.sender(), token).is_zero() {
            panic!("check failed: not token owner")
        }
    }
}
