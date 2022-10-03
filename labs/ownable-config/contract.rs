use crate::config::Config;
use crate::config::Ownable;
use crate::config::Zero;

#[derive(Clone, Copy)]
pub struct Contract<T: Config> {
    pub owner: T::AccountId,
}

impl<T: Config> Contract<T> {
    pub fn new(owner: &T::AccountId) -> Self {
        Self { owner: *owner }
    }
}

impl<T: Config> Default for Contract<T> {
    fn default() -> Self {
        Self {
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
