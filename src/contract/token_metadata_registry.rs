//! contract implementation

use crate::*;

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
