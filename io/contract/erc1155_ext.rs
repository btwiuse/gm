//! contract implementation

use crate::*;

/// ERC1155Ext interface
impl<T: IConfig> IERC1155Ext<T> for Contract<T> {
    fn name(&self) -> T::Text {
        self.name.clone()
    }
    fn symbol(&self) -> T::Text {
        self.symbol.clone()
    }
    fn burn(&mut self, from: T::AccountId, token: T::TokenId, amount: T::Balance) {
        self.check_burn(from, token, amount);
        self.balances.entry(token).and_modify(|kv| {
            kv.entry(from)
                .and_modify(|v| *v = v.saturating_sub(&amount));
        });
    }
    fn burn_batch(&mut self, from: T::AccountId, token: Vec<T::TokenId>, amount: Vec<T::Balance>) {
        self.check_burn_batch(from, token.clone(), amount.clone());
        token
            .iter()
            .enumerate()
            .for_each(|(i, tk)| self.burn(from, *tk, amount[i]))
    }
    fn mint(&mut self, to: T::AccountId, token: T::TokenId, amount: T::Balance) {
        self.check_mint(to, token, amount);
        self.balances
            .entry(token)
            .and_modify(|kv| {
                kv.entry(to)
                    .and_modify(|v| *v = v.saturating_add(&amount))
                    .or_insert(amount);
            })
            .or_insert({
                let mut btm = BTreeMap::new();
                btm.insert(to, amount);
                btm
            });
    }
    fn mint_batch(&mut self, to: T::AccountId, token: Vec<T::TokenId>, amount: Vec<T::Balance>) {
        self.check_mint_batch(to, token.clone(), amount.clone());
        token
            .iter()
            .enumerate()
            .for_each(|(i, tk)| self.mint(to, *tk, amount[i]))
    }
}
