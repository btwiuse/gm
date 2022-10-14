use crate::config::Config;
use crate::config::IText;
use crate::config::ITokenDecimal;
use crate::config::Ledger;
use crate::config::Ownable;
use crate::config::ERC20;
use crate::BTreeMap;
use crate::Option;

pub struct Contract<T: Config> {
    pub balances: BTreeMap<T::AccountId, T::Balance>,
    pub owner: T::AccountId,
    pub total_issuance: T::Balance,
    pub name: T::Text,
    pub symbol: T::Text,
    pub decimals: T::TokenDecimal,
    // https://eips.ethereum.org/EIPS/eip-1046
    pub token_uri: T::Text,
    pub allowances: BTreeMap<T::AccountId, BTreeMap<T::AccountId, T::Balance>>,
}

impl<T: Config> Contract<T> {
    pub fn new(owner: &T::AccountId) -> Self {
        Self {
            owner: *owner,
            ..Self::default()
        }
    }
}

impl<T: Config> Default for Contract<T> {
    fn default() -> Self {
        use crate::config::Zero;
        Self {
            name: T::Text::default(),
            symbol: T::Text::default(),
            total_issuance: T::Balance::default(),
            decimals: T::TokenDecimal::default(),
            token_uri: T::Text::default(),
            balances: BTreeMap::<T::AccountId, T::Balance>::default(),
            allowances: BTreeMap::<T::AccountId, BTreeMap<T::AccountId, T::Balance>>::default(),
            owner: T::AccountId::zero(),
        }
    }
}

impl<T: Config> Ownable<T> for Contract<T> {
    fn owner(&self) -> T::AccountId {
        self.owner.clone()
    }
    fn is_owner(&self, who: &T::AccountId) -> bool {
        *who == self.owner
    }
}

impl<T: Config> Ownable<T> for Option<Contract<T>> {
    fn owner(&self) -> T::AccountId {
        self.as_ref().unwrap().owner()
    }
    fn is_owner(&self, who: &T::AccountId) -> bool {
        self.as_ref().unwrap().is_owner(who)
    }
}

impl<T: Config> Ledger<T> for Contract<T> {
    fn balance_incr(&mut self, who: &T::AccountId) {
        use num_traits::CheckedAdd;
        use num_traits::One;
        let balance = Ledger::balance_of(self, &who);
        if let Some(balance_plus_one) = balance.checked_add(&T::Balance::one()) {
            self.balances.insert(*who, balance_plus_one);
        }
    }
    fn balance_of(&self, who: &T::AccountId) -> T::Balance {
        self.balances.get(&who).copied().unwrap_or_default()
    }
}

impl<T: Config> Ledger<T> for Option<Contract<T>> {
    fn balance_incr(&mut self, who: &T::AccountId) {
        self.as_mut().unwrap().balance_incr(who)
    }
    fn balance_of(&self, who: &T::AccountId) -> T::Balance {
        Ledger::balance_of(self.as_ref().unwrap(), who)
    }
}

impl<T: Config> ERC20<T> for Contract<T> {
    fn symbol(&self) -> T::Text {
        self.symbol.clone()
    }
    fn name(&self) -> T::Text {
        self.name.clone()
    }
    // fn token_uri(&self) -> T::Text { self.token_uri }
    fn decimals(&self) -> T::TokenDecimal {
        self.decimals
    }
    fn total_issuance(&self) -> T::Balance {
        self.total_issuance
    }
    fn burn(&mut self) {
        todo!()
    }
    fn mint(&mut self) {
        todo!()
    }
    fn balance_of(&self, who: &T::AccountId) -> T::Balance {
        self.balances.get(&who).copied().unwrap_or_default()
    }
    fn allowance(&self, owner: &T::AccountId, spender: &T::AccountId) -> T::Balance {
        self.allowances
            .get(&owner)
            .cloned()
            .unwrap_or_default()
            .get(&spender)
            .copied()
            .unwrap_or_default()
    }
    fn transfer(&mut self, to: &T::AccountId, amount: T::Balance) {
        todo!()
    }
    fn transfer_from(&mut self, from: &T::AccountId, to: &T::AccountId, amount: T::Balance) {
        todo!()
    }
    fn emit_transfer_event(from: &T::AccountId, to: &T::AccountId, amount: &T::Balance) {
        todo!()
    }
    fn emit_approval_event(owner: &T::AccountId, spender: &T::AccountId, amount: &T::Balance) {
        todo!()
    }
}
