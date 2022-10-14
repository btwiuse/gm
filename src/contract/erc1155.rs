//! contract implementation

use crate::*;

/// ERC1155 interface
impl<T: IConfig> IERC1155<T> for Contract<T> {
    fn balance_of(&self, who: T::AccountId, token: T::TokenId) -> T::Balance {
        *self
            .balances
            .get(&token)
            .and_then(|kv| kv.get(&who))
            .unwrap_or(&T::Balance::zero())
    }
    fn balance_of_batch(&self, who: Vec<T::AccountId>, token: Vec<T::TokenId>) -> Vec<T::Balance> {
        self.check_balance_of_batch(who.clone(), token.clone());
        token
            .iter()
            .zip(who)
            .map(|(token, account)| self.balance_of(account, *token))
            .collect()
    }
    fn safe_transfer_from(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::Balance,
    ) {
        self.check_transfer_from(from, to, token, amount);
        self.balances.entry(token).and_modify(|kv| {
            kv.entry(from)
                .and_modify(|v| *v = v.saturating_sub(&amount));
            kv.entry(to)
                .and_modify(|v| *v = v.saturating_add(&amount))
                .or_insert(amount);
        });
    }
    fn safe_batch_transfer_from(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::Balance>,
    ) {
        self.check_batch_transfer_from(from, to, token.clone(), amount.clone());
        token
            .iter()
            .enumerate()
            .for_each(|(i, tk)| self.safe_transfer_from(from, to, *tk, amount[i]))
    }
    fn set_approval_for_all(
        &mut self,
        owner: T::AccountId,
        operator: T::AccountId,
        approved: bool,
    ) {
        self.check_set_approval_for_all(owner, operator, approved);
        self.approvals
            .entry(owner)
            .and_modify(|kv| {
                kv.entry(operator).and_modify(|v| *v = approved);
            })
            .or_insert({
                let mut btm = BTreeMap::new();
                btm.insert(operator, approved);
                btm
            });
    }
    fn is_approved_for_all(&self, owner: T::AccountId, operator: T::AccountId) -> bool {
        *self
            .approvals
            .get(&owner)
            .and_then(|kv| kv.get(&operator))
            .unwrap_or(&false)
    }
}
