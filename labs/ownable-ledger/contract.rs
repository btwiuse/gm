use crate::config::Config;
use crate::config::Ledger;
use crate::config::Ownable;
use crate::BTreeMap;
use crate::Option;

pub struct Contract<T: Config> {
    pub balances: BTreeMap<T::AccountId, T::AccountBalance>,
    pub owner: T::AccountId,
}

impl<T: Config> Contract<T> {
    pub fn new(owner: &T::AccountId) -> Self {
        Self {
            balances: BTreeMap::new(),
            owner: *owner,
        }
    }
}

impl<T: Config> Default for Contract<T> {
    fn default() -> Self {
        use crate::config::Zero;
        Self::new(&T::AccountId::zero())
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
        let balance = self.balance_of(&who);
        if let Some(balance_plus_one) = balance.checked_add(&T::AccountBalance::one()) {
            self.balances.insert(*who, balance_plus_one);
        }
    }
    fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance {
        self.balances.get(&who).copied().unwrap_or_default()
    }
}

impl<T: Config> Ledger<T> for Option<Contract<T>> {
    fn balance_incr(&mut self, who: &T::AccountId) {
        self.as_mut().unwrap().balance_incr(who)
    }
    fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance {
        self.as_ref().unwrap().balance_of(who)
    }
}
