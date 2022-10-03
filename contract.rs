//! contract implementation

use crate::*;

/// Contract struct
pub struct Contract<T: IConfig> {
    pub balances: BTreeMap<T::AccountId, T::AccountBalance>,
    pub owner: T::AccountId,
    pub total_issuance: T::AccountBalance,
    pub name: T::Text,
    pub symbol: T::Text,
    pub decimals: T::TokenDecimal,
    // https://eips.ethereum.org/EIPS/eip-1046
    pub token_uri: T::Text,
    pub allowances: BTreeMap<T::AccountId, BTreeMap<T::AccountId, T::AccountBalance>>,
    pub base_uri: T::Text,
}

/// constructor method
impl<T: IConfig> Contract<T> {
    pub fn new(owner: &T::AccountId) -> Self {
        Self {
            owner: *owner,
            ..Self::default()
        }
    }
}

/// Default interface
impl<T: IConfig> Default for Contract<T> {
    fn default() -> Self {
        Self {
            name: T::Text::default(),
            symbol: T::Text::default(),
            total_issuance: T::AccountBalance::default(),
            decimals: T::TokenDecimal::default(),
            token_uri: T::Text::default(),
            balances: BTreeMap::<T::AccountId, T::AccountBalance>::default(),
            allowances:
                BTreeMap::<T::AccountId, BTreeMap<T::AccountId, T::AccountBalance>>::default(),
            owner: T::AccountId::zero(),
            base_uri: T::Text::default(),
        }
    }
}

/// IOwnable interface
impl<T: IConfig> IOwnable<T> for Contract<T> {
    fn owner(&self) -> T::AccountId {
        self.owner.clone()
    }
    fn is_owner(&self, who: &T::AccountId) -> bool {
        *who == self.owner
    }
}

/// IOwnable interface
impl<T: IConfig> IOwnable<T> for Option<Contract<T>> {
    fn owner(&self) -> T::AccountId {
        self.as_ref().unwrap().owner()
    }
    fn is_owner(&self, who: &T::AccountId) -> bool {
        self.as_ref().unwrap().is_owner(who)
    }
}

mod ignore {
    /*
        use super::*;
        /// Ledger interface
        impl<T: IConfig> ILedger<T> for Contract<T> {
            fn balance_incr(&mut self, who: &T::AccountId) {
                use num_traits::CheckedAdd;
                use num_traits::One;
                let balance = ILedger::balance_of(self, &who);
                if let Some(balance_plus_one) = balance.checked_add(&T::AccountBalance::one()) {
                    self.balances.insert(*who, balance_plus_one);
                }
            }
            fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance {
                self.balances.get(&who).copied().unwrap_or_default()
            }
        }

        /// Ledger interface
        impl<T: IConfig> ILedger<T> for Option<Contract<T>> {
            fn balance_incr(&mut self, who: &T::AccountId) {
                self.as_mut().unwrap().balance_incr(who)
            }
            fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance {
                ILedger::balance_of(self.as_ref().unwrap(), who)
            }
        }

        /// ERC20 interface
        impl<T: IConfig> IERC20<T> for Contract<T> {
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
            fn total_issuance(&self) -> T::AccountBalance {
                self.total_issuance
            }
            fn burn(&mut self) {
                todo!()
            }
            fn mint(&mut self) {
                todo!()
            }
            fn balance_of(&self, who: &T::AccountId) -> T::AccountBalance {
                self.balances.get(&who).copied().unwrap_or_default()
            }
            fn allowance(&self, owner: &T::AccountId, spender: &T::AccountId) -> T::AccountBalance {
                self.allowances
                    .get(&owner)
                    .cloned()
                    .unwrap_or_default()
                    .get(&spender)
                    .copied()
                    .unwrap_or_default()
            }
            fn transfer(&mut self, to: &T::AccountId, amount: T::AccountBalance) {
                todo!()
            }
            fn transfer_from(
                &mut self,
                from: &T::AccountId,
                to: &T::AccountId,
                amount: T::AccountBalance,
            ) {
                todo!()
            }
            fn emit_transfer_event(from: &T::AccountId, to: &T::AccountId, amount: &T::AccountBalance) {
                todo!()
            }
            fn emit_approval_event(
                owner: &T::AccountId,
                spender: &T::AccountId,
                amount: &T::AccountBalance,
            ) {
                todo!()
            }
        }
    */
}

/// ERC1155 interface
impl<T: IConfig> IERC1155<T> for Contract<T> {
    fn balance_of(&self, who: T::AccountId, token: T::TokenId) -> T::AccountBalance {
        todo!()
    }
    fn balance_of_batch(
        &self,
        who: Vec<T::AccountId>,
        token: Vec<T::TokenId>,
    ) -> Vec<T::AccountBalance> {
        todo!()
    }
    fn safe_transfer_from(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    ) {
        todo!()
    }
    fn safe_batch_transfer_from(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        todo!()
    }
    /*
    fn transfer_from(
        &mut self,
        from: &T::AccountId,
        to: &T::AccountId,
        token: &T::TokenId,
        amount: &T::AccountBalance,
    ) {
        todo!()
    }
    */
    fn set_approval_for_all(&mut self, operator: T::AccountId, approved: bool) {
        todo!()
    }
    fn is_approved_for_all(&mut self, owner: T::AccountId, operator: T::AccountId) -> bool {
        todo!()
    }
}

/// ERC1155MetadataURI interface
impl<T: IConfig> IERC1155MetadataURI<T> for Contract<T> {
    fn uri(&self, _token: T::TokenId) -> T::Text {
        self.base_uri.clone()
    }
}

/// ERC1155Ext interface
impl<T: IConfig> IERC1155Ext<T> for Contract<T> {
    fn name(&self) -> T::Text {
        todo!()
    }
    fn symbol(&self) -> T::Text {
        todo!()
    }
    fn burn(&mut self, from: T::AccountId, token: T::TokenId, amount: T::AccountBalance) {
        todo!()
    }
    fn burn_batch(
        &mut self,
        from: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        todo!()
    }
    fn mint(&mut self, to: T::AccountId, token: T::TokenId, amount: T::AccountBalance) {
        todo!()
    }
    fn mint_batch(
        &mut self,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        todo!()
    }
}
/*
/// ERC1155Ext interface
impl<T: IConfig> IERC1155Ext<T> for Option<Contract<T>> {
    fn name(&self) -> T::Text {
        // self.as_ref().unwrap().name()
        // IERC1155Ext::name(&self.as_ref().unwrap())
        todo!()
    }
    fn symbol(&self) -> T::Text {
        // self.as_ref().unwrap().symbol()
        // IERC1155Ext::symbol(&self.as_ref().unwrap())
        todo!()
    }
    fn emit_transfer_single_event(
        &self,
        operator: &T::AccountId,
        from: &T::AccountId,
        to: &T::AccountId,
        token: &T::TokenId,
        amount: &T::AccountBalance,
    ) {
        todo!()
    }
    fn emit_transfer_batch_event(
        operator: &T::AccountId,
        from: &T::AccountId,
        to: &T::AccountId,
        token: &[T::TokenId],
        amount: &[T::AccountBalance],
    ) {
        todo!()
    }
    fn emit_approval_for_all_event(owner: &T::AccountId, spender: &T::AccountId, approved: bool) {
        todo!()
    }
    fn emit_uri_event(value: &T::Text, token: &T::TokenId) {
        todo!()
    }
    fn burn(&mut self, from: &T::AccountId, token: &T::TokenId, amount: &T::AccountBalance) {
        todo!()
    }
    fn mint(&mut self, to: &T::AccountId, token: &T::TokenId, amount: &T::AccountBalance) {
        todo!()
    }
}
*/

/// ERC1155Ext interface
impl IERC1155GearExt for Contract<GearConfig> {
    fn emit_transfer_single_event(
        &self,
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        token: u128,
        amount: u128,
    ) {
        /*
        msg::reply(Event::TransferSingle{
            operator: *operator,
            from: *from,
            to: *to,
            token: *token,
            amount: *amount,
        }, 0).expect("Failed to reply Event::TransferSingle");
        */
        todo!()
    }
    fn emit_transfer_batch_event(
        &self,
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        token: Vec<u128>,
        amount: Vec<u128>,
    ) {
        todo!()
    }
    fn emit_approval_for_all_event(&self, owner: ActorId, spender: ActorId, approved: bool) {
        todo!()
    }
    fn emit_uri_event(&self, value: String, token: u128) {
        todo!()
    }
}

/*
/// ERC1155Ext interface
impl<T: IConfig> IERC1155Ext<T> for Contract<T> {
    fn name(&self) -> T::Text {
        self.name.clone()
    }
    fn symbol(&self) -> T::Text {
        self.symbol.clone()
    }
    fn emit_transfer_single_event(
        &self,
        operator: &T::AccountId,
        from: &T::AccountId,
        to: &T::AccountId,
        token: &T::TokenId,
        amount: &T::AccountBalance,
    ) {
        /*
        msg::reply(Event::TransferSingle{
            operator: *operator,
            from: *from,
            to: *to,
            token: *token,
            amount: *amount,
        }, 0).expect("Failed to reply Event::TransferSingle");
        */
        todo!()
    }
    fn emit_transfer_batch_event(
        operator: &T::AccountId,
        from: &T::AccountId,
        to: &T::AccountId,
        token: &[T::TokenId],
        amount: &[T::AccountBalance],
    ) {
        todo!()
    }
    fn emit_approval_for_all_event(owner: &T::AccountId, spender: &T::AccountId, approved: bool) {
        todo!()
    }
    fn emit_uri_event(value: &T::Text, token: &T::TokenId) {
        todo!()
    }
    fn burn(&mut self, from: &T::AccountId, token: &T::TokenId, amount: &T::AccountBalance) {
        todo!()
    }
    fn mint(&mut self, to: &T::AccountId, token: &T::TokenId, amount: &T::AccountBalance) {
        todo!()
    }
}
*/
