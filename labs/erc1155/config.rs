use gstd::ActorId;
use gstd::String;
use gstd::ToString;

pub trait ITokenId = Eq + Copy + Clone + core::hash::Hash + Ord;

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
    + Default
    + From<u16>
    + From<u32>;

pub trait ITokenDecimal = num_traits::Zero
    + num_traits::One
    + num_traits::CheckedAdd
    + num_traits::CheckedSub
    + Copy
    + Clone
    + Default;

pub trait IText: ToString + Clone {
    fn default() -> Self;
}

impl IText for String {
    fn default() -> Self {
        return String::from("");
    }
}

pub trait Config {
    type AccountId: IAccountId;
    type AccountBalance: IAccountBalance;
    type Text: IText;
    type TokenDecimal: ITokenDecimal;
    type TokenId: ITokenId;
}

// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/access/Ownable.sol
pub trait Ownable<T: Config> {
    fn owner(&self) -> T::AccountId;
    fn is_owner(&self, who: &T::AccountId) -> bool;
}

pub trait Ledger<T: Config> {
    fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance;
    fn balance_incr(&mut self, who: &T::AccountId);
}

// https://github.com/paritytech/ink/blob/master/examples/erc20/lib.rs
// https://github.com/ethereum/EIPs/blob/master/EIPS/eip-20.md
pub trait ERC20<T: Config> {
    fn symbol(&self) -> T::Text;
    fn name(&self) -> T::Text;
    fn decimals(&self) -> T::TokenDecimal;
    fn total_issuance(&self) -> T::AccountBalance;
    fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance;
    fn transfer(&mut self, to: &T::AccountId, amount: T::AccountBalance);
    fn transfer_from(&mut self, from: &T::AccountId, to: &T::AccountId, amount: T::AccountBalance);
    fn allowance(&self, owner: &T::AccountId, spender: &T::AccountId) -> T::AccountBalance;
    fn emit_transfer_event(from: &T::AccountId, to: &T::AccountId, amount: &T::AccountBalance);
    fn emit_approval_event(
        owner: &T::AccountId,
        spender: &T::AccountId,
        amount: &T::AccountBalance,
    );
    fn burn(&mut self);
    fn mint(&mut self);
}

// https://eips.ethereum.org/EIPS/eip-721
pub trait ERC721<T: Config> {
    fn symbol(&self) -> T::Text;
    fn name(&self) -> T::Text;
    fn decimals(&self) -> T::TokenDecimal;
    fn total_issuance(&self) -> T::AccountBalance;
    fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance;
    fn owner_of(&self, token: &T::TokenId) -> T::AccountId;
    fn safe_transfer_from(&mut self, from: &T::AccountId, to: &T::AccountId, token: &T::TokenId);
    fn transfer_from(&mut self, from: &T::AccountId, to: &T::AccountId, token: &T::TokenId);
    fn approve(&self, who: &T::AccountId, token: &T::TokenId);
    fn get_approved(&self, token: &T::TokenId) -> T::AccountId;
    fn emit_transfer_event(from: &T::AccountId, to: &T::AccountId, token: &T::TokenId);
    fn emit_approval_event(owner: &T::AccountId, spender: &T::AccountId, token: &T::TokenId);
    fn burn(&mut self, token: &T::TokenId);
    fn mint(&mut self, to: &T::AccountId, token: &T::TokenId);
}

// https://eips.ethereum.org/EIPS/eip-1155
// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC1155/ERC1155.sol
pub trait ERC1155<T: Config> {
    // fn symbol(&self) -> T::Text;
    // fn name(&self) -> T::Text;
    // fn decimals(&self) -> T::TokenDecimal;
    // fn total_issuance(&self) -> T::AccountBalance;
    fn balance_of(&self, who: &T::AccountId, token: &T::TokenId) -> T::AccountBalance;
    fn balance_of_batch(&self, who: &[T::AccountId], token: &[T::TokenId]) -> &[T::AccountBalance];
    fn safe_transfer_from(&mut self, from: &T::AccountId, to: &T::AccountId, token: &T::TokenId);
    fn transfer_from(
        &mut self,
        from: &T::AccountId,
        to: &T::AccountId,
        token: &T::TokenId,
        amount: &T::AccountBalance,
    );
    fn set_approval_for_all(&mut self, operator: &T::AccountId, approved: bool);
    fn is_approved_for_all(&mut self, owner: &T::AccountId, operator: &T::AccountId) -> bool;
    /// fn approve(&self, who: &T::AccountId, token: &T::TokenId);
    // fn get_approved(&self, token: &T::TokenId) -> T::AccountId;
    fn emit_transfer_single_event(
        operator: &T::AccountId,
        from: &T::AccountId,
        to: &T::AccountId,
        token: &T::TokenId,
        amount: &T::AccountBalance,
    );
    fn emit_transfer_batch_event(
        operator: &T::AccountId,
        from: &T::AccountId,
        to: &T::AccountId,
        token: &[T::TokenId],
        amount: &[T::AccountBalance],
    );
    fn emit_approval_for_all_event(owner: &T::AccountId, spender: &T::AccountId, approved: bool);
    fn emit_uri_event(value: &T::Text, token: &T::TokenId);
    fn burn(&mut self, from: &T::AccountId, token: &T::TokenId, amount: &T::AccountBalance);
    fn mint(&mut self, to: &T::AccountId, token: &T::TokenId, amount: &T::AccountBalance);
}
