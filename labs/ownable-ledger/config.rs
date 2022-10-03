use gstd::ActorId;

pub trait IAccountId = Zero + Eq + Copy + Clone + core::hash::Hash + Ord;

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

impl Zero for ActorId {
    fn zero() -> Self {
        ActorId::zero()
    }
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

pub trait IAccountBalance = num_traits::Zero
    + num_traits::One
    + num_traits::CheckedAdd
    + num_traits::CheckedSub
    + Copy
    + Clone
    + Default;

pub trait Config {
    type AccountId: IAccountId;
    type AccountBalance: IAccountBalance;
}

pub trait Ownable<T: Config> {
    fn owner(&self) -> T::AccountId;
    fn is_owner(&self, who: &T::AccountId) -> bool;
}

pub trait Ledger<T: Config> {
    fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance;
    fn balance_incr(&mut self, who: &T::AccountId);
}
