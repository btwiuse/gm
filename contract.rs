//! contract implementation

use crate::*;

/// Contract struct
pub struct Contract<T: IConfig> {
    pub env: T,
    pub owner: T::AccountId,
    pub name: T::Text,
    pub symbol: T::Text,
    pub base_uri: T::Text,
    pub balances: BTreeMap<T::TokenId, BTreeMap<T::AccountId, T::AccountBalance>>,
    pub approvals: BTreeMap<T::AccountId, BTreeMap<T::AccountId, bool>>,
    pub metadata_registry: BTreeMap<T::TokenId, TokenMetadata>,
}

/// constructor method
impl<T: IConfig> Contract<T> {
    pub fn new(owner: &T::AccountId) -> Self {
        Self {
            owner: *owner,
            ..Self::default()
        }
    }
    pub fn sender(&self) -> T::AccountId {
        self.env.sender()
    }
    pub fn origin(&self) -> T::AccountId {
        self.env.origin()
    }
}

impl<T: IConfig> Default for Contract<T> {
    fn default() -> Self {
        Self {
            env: T::default(),
            owner: T::AccountId::zero(),
            name: T::Text::default(),
            symbol: T::Text::default(),
            base_uri: T::Text::default(),
            balances: BTreeMap::<T::TokenId, BTreeMap<T::AccountId, T::AccountBalance>>::default(),
            approvals: BTreeMap::<T::AccountId, BTreeMap<T::AccountId, bool>>::default(),
            metadata_registry: BTreeMap::<T::TokenId, TokenMetadata>::default(),
        }
    }
}

/// ERC1155Check interface
impl<T: IConfig> IERC1155Check<T> for Contract<T> {
    fn check_transfer_from(
        &self,
        from: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    ) {
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

/// ERC1155 interface
impl<T: IConfig> IERC1155<T> for Contract<T> {
    fn balance_of(&self, who: T::AccountId, token: T::TokenId) -> T::AccountBalance {
        *self
            .balances
            .get(&token)
            .and_then(|kv| kv.get(&who))
            .unwrap_or(&T::AccountBalance::zero())
    }
    fn balance_of_batch(
        &self,
        who: Vec<T::AccountId>,
        token: Vec<T::TokenId>,
    ) -> Vec<T::AccountBalance> {
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
        amount: T::AccountBalance,
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
        amount: Vec<T::AccountBalance>,
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

/// ERC1155MetadataURI interface
impl<T: IConfig> IERC1155MetadataURI<T> for Contract<T> {
    fn uri(&self, _token: T::TokenId) -> T::Text {
        self.base_uri.clone()
    }
}

/// ERC1155Ext interface
impl<T: IConfig> IERC1155Ext<T> for Contract<T> {
    fn name(&self) -> T::Text {
        self.name.clone()
    }
    fn symbol(&self) -> T::Text {
        self.symbol.clone()
    }
    fn burn(&mut self, from: T::AccountId, token: T::TokenId, amount: T::AccountBalance) {
        self.check_burn(from, token, amount);
        self.balances.entry(token).and_modify(|kv| {
            kv.entry(from)
                .and_modify(|v| *v = v.saturating_sub(&amount));
        });
    }
    fn burn_batch(
        &mut self,
        from: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        self.check_burn_batch(from, token.clone(), amount.clone());
        token
            .iter()
            .enumerate()
            .for_each(|(i, tk)| self.burn(from, *tk, amount[i]))
    }
    fn mint(&mut self, to: T::AccountId, token: T::TokenId, amount: T::AccountBalance) {
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
    fn mint_batch(
        &mut self,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        self.check_mint_batch(to, token.clone(), amount.clone());
        token
            .iter()
            .enumerate()
            .for_each(|(i, tk)| self.mint(to, *tk, amount[i]))
    }
}

/// ITokenMetadataRegistry interface
impl<T: IConfig> ITokenMetadataRegistry<T> for Contract<T> {
    fn get_token_metadata(&self, token: T::TokenId) -> Option<TokenMetadata> {
        self.metadata_registry.get(&token).cloned()
    }
    fn update_token_metadata(&mut self, token: T::TokenId, metadata: Option<TokenMetadata>) {
        self.check_update_token_metadata(token, metadata.clone());
        match metadata {
            Some(m) => {
                self.metadata_registry.insert(token, m);
            }
            None => {
                self.metadata_registry.remove_entry(&token);
            }
        }
    }
}

/// ERC1155GearExt interface
impl IERC1155GearExt for Contract<GearConfig> {
    fn emit_update_token_metadata_event(&self, token: u128, metadata: Option<TokenMetadata>) {
        gstd::msg::reply(Event::UpdateTokenMetadata { token, metadata }, 0)
            .expect("Failed to reply Event::UpdateTokenMetadata");
    }
    fn emit_whoami_event(&self) {
        gstd::msg::reply(
            Event::Whoami {
                sender: self.sender(),
                origin: self.origin(),
            },
            0,
        )
        .expect("Failed to reply Event::Whoami");
    }
    fn emit_transfer_single_event(
        &self,
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        token: u128,
        amount: u128,
    ) {
        gstd::msg::reply(
            Event::TransferSingle {
                operator,
                from,
                to,
                token,
                amount,
            },
            0,
        )
        .expect("Failed to reply Event::TransferSingle");
    }
    fn emit_transfer_batch_event(
        &self,
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        token: Vec<u128>,
        amount: Vec<u128>,
    ) {
        gstd::msg::reply(
            Event::TransferBatch {
                operator,
                from,
                to,
                token,
                amount,
            },
            0,
        )
        .expect("Failed to reply Event::TransferBatch");
    }
    fn emit_approval_for_all_event(&self, owner: ActorId, operator: ActorId, approved: bool) {
        gstd::msg::reply(
            Event::ApprovedForAll {
                owner,
                operator,
                approved,
            },
            0,
        )
        .expect("Failed to reply Event::ApprovedForAll");
    }
    fn emit_uri_event(&self, value: String, token: u128) {
        gstd::msg::reply(Event::URI { value, token }, 0).expect("Failed to reply Event::URI");
    }
}
