//! contract implementation

use crate::*;

/// Contract struct
pub struct Contract<T: IConfig> {
    pub owner: T::AccountId,
    // pub total_issuance: T::AccountBalance,
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
}

impl<T: IConfig> Default for Contract<T> {
    fn default() -> Self {
        Self {
            owner: T::AccountId::zero(),
            // total_issuance: T::AccountBalance::default(),
            name: T::Text::default(),
            symbol: T::Text::default(),
            base_uri: T::Text::default(),
            balances: BTreeMap::<T::TokenId, BTreeMap<T::AccountId, T::AccountBalance>>::default(),
            approvals: BTreeMap::<T::AccountId, BTreeMap<T::AccountId, bool>>::default(),
            metadata_registry: BTreeMap::<T::TokenId, TokenMetadata>::default(),
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

/// ERC1155Check interface
impl<T: IConfig> IERC1155Check<T> for Contract<T> {
    fn check_transfer_from(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        from: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    ) {
        if from != signer && from != origin && !self.is_approved_for_all(from, signer) {
            panic!("permission denied")
        }
        if self.balance_of(from, token) < amount {
            panic!("insufficient balance")
        }
        if from == to {
            panic!("self transfer not permitted")
        }
        if to.is_zero() {
            panic!("transfer to black hole not permitted")
        }
    }
    fn check_batch_transfer_from(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        from: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        if token.len() != amount.len() {
            panic!("token and amount length mismatch")
        }
        for (tk, am) in token.iter().zip(amount.clone()) {
            self.check_transfer_from(signer, origin, from, to, *tk, am)
        }
    }
    fn check_mint(
        &mut self,
        _signer: T::AccountId,
        _origin: T::AccountId,
        to: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    ) {
        if to.is_zero() {
            panic!("cannot mint to black hole address")
        }
        if amount.is_zero() {
            panic!("cannot mint 0 amount")
        }
        if self.balances.get(&token).is_some() {
            panic!("cannot mint twice")
        }
    }
    fn check_mint_batch(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        to: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        if token.len() != amount.len() {
            panic!("token and amount length mismatch")
        }
        for (tk, am) in token.iter().zip(amount.clone()) {
            self.check_mint(signer, origin, to, *tk, am)
        }
    }
    fn check_set_approval_for_all(
        &mut self,
        _signer: T::AccountId,
        _origin: T::AccountId,
        owner: T::AccountId,
        operator: T::AccountId,
        _approved: bool,
    ) {
        if !self.is_approved_for_all(owner, operator) {
            panic!("needs approval")
        }
    }
    fn check_burn(
        &mut self,
        _signer: T::AccountId,
        _origin: T::AccountId,
        from: T::AccountId,
        token: T::TokenId,
        amount: T::AccountBalance,
    ) {
        if self.balance_of(from, token) < amount {
            panic!("insufficient balance")
        }
    }
    fn check_burn_batch(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        from: T::AccountId,
        token: Vec<T::TokenId>,
        amount: Vec<T::AccountBalance>,
    ) {
        if token.len() != amount.len() {
            panic!("token and amount length mismatch")
        }
        for (tk, am) in token.iter().zip(amount.clone()) {
            self.check_burn(signer, origin, from, *tk, am)
        }
    }
    // allow owner of token to update metadata
    fn check_update_token_metadata(
        &mut self,
        signer: T::AccountId,
        origin: T::AccountId,
        token: T::TokenId,
        _metadata: Option<TokenMetadata>,
    ) {
        if self.balance_of(signer, token).is_zero() || self.balance_of(origin, token).is_zero() {
            panic!("no permission")
        }
    }
}

/// ERC1155 interface
impl<T: IConfig> IERC1155<T> for Contract<T> {
    fn balance_of(&self, who: T::AccountId, token: T::TokenId) -> T::AccountBalance {
        self.balances
            .get(&token)
            .cloned()
            .unwrap_or_default()
            .get(&who)
            .cloned()
            .unwrap_or_default()
    }
    fn balance_of_batch(
        &self,
        who: Vec<T::AccountId>,
        token: Vec<T::TokenId>,
    ) -> Vec<T::AccountBalance> {
        if who.len() != token.len() {
            panic!("Error: length of accounts and tokens mismatch")
        }
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
        if self.balance_of(from, token) < amount {
            panic!("Error: insufficient balance")
        }
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
        self.approvals
            .get(&owner)
            .cloned()
            .unwrap_or_default()
            .get(&operator)
            .cloned()
            .unwrap_or_default()
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
        token
            .iter()
            .enumerate()
            .for_each(|(i, tk)| self.burn(from, *tk, amount[i]))
    }
    fn mint(&mut self, to: T::AccountId, token: T::TokenId, amount: T::AccountBalance) {
        // debug!("minting {:?} {:?} {:?}", to, token, amount);
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
    fn emit_transfer_single_event(
        &self,
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        token: u128,
        amount: u128,
    ) {
        msg::reply(
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
        msg::reply(
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
        msg::reply(
            Event::ApprovalForAll {
                owner,
                operator,
                approved,
            },
            0,
        )
        .expect("Failed to reply Event::ApprovalForAll");
    }
    fn emit_uri_event(&self, value: String, token: u128) {
        msg::reply(Event::URI { value, token }, 0).expect("Failed to reply Event::URI");
    }
}
