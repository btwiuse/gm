//! contract implementation

use crate::*;

/// ERC1155MetadataURI interface
impl<T: IConfig> IERC1155MetadataURI<T> for Contract<T> {
    fn uri(&self, _token: T::TokenId) -> T::Text {
        self.base_uri.clone()
    }
}
