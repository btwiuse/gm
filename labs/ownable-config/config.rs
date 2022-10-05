use gstd::ActorId;

pub trait IAccountId = Zero + Eq + Copy + Clone;

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for ActorId {
    fn zero() -> Self {
        ActorId::zero()
    }
}

pub trait Config {
    type AccountId: IAccountId;
}

pub trait Config2 {
    type AccountId: IAccountId;
}

pub trait C = Config + Config2;

pub trait Ownable<T: Config> {
    fn owner(&self) -> T::AccountId;
    fn is_owner(&self, who: &T::AccountId) -> bool;
}
