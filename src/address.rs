use ethers::{
    prelude::{abigen, ContractError},
    providers::{Http, Provider},
    types::H160,
};
use eyre::Result;
use std::sync::Arc;

const ERC721_BYTE_CODE: [u8; 4] = [128, 172, 88, 205]; // "0x80ac58cd"
const ERC1155_BYTE_CODE: [u8; 4] = [217, 182, 122, 38]; // "0xd9b67a26"

abigen!(
    IERC165,
    r#"[function supportsInterface(bytes4 interfaceId) external view returns (bool)]"#
);
pub enum ERC165DerivedOrNot {
    ERC721,
    ERC1155,
    OTHER,
}

pub struct IdentifiableAddress {
    pub address: H160,
}

impl IdentifiableAddress {
    pub async fn is_matched(
        &self,
        client: &Arc<Provider<Http>>,
        interface_id: [u8; 4],
    ) -> Result<bool, ContractError<Provider<Http>>> {
        let standard = IERC165::new(self.address, client.to_owned());
        standard.supports_interface(interface_id).call().await
    }

    pub async fn check_standard(
        &self,
        client: &Arc<Provider<Http>>,
    ) -> Result<ERC165DerivedOrNot, ContractError<Provider<Http>>> {
        let erc721_matched = self.is_matched(client, ERC721_BYTE_CODE.clone()).await?;
        let erc1155_matched = self.is_matched(client, ERC1155_BYTE_CODE.clone()).await?;
        if erc721_matched {
            return Ok(ERC165DerivedOrNot::ERC721);
        }
        if erc1155_matched {
            return Ok(ERC165DerivedOrNot::ERC1155);
        }
        Ok(ERC165DerivedOrNot::OTHER)
    }
}
