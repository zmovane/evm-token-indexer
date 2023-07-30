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
    ) -> Result<ERC165DerivedOrNot, Vec<ContractError<Provider<Http>>>> {
        let erc721_match_res = self.is_matched(client, ERC721_BYTE_CODE.clone()).await;
        let erc1155_match_res = self.is_matched(client, ERC1155_BYTE_CODE.clone()).await;
        let mut unknown = false;
        if let Ok(matched) = erc721_match_res {
            match matched {
                true => return Ok(ERC165DerivedOrNot::ERC721),
                false => unknown = true,
            }
        }
        if let Ok(matched) = erc1155_match_res {
            match matched {
                true => return Ok(ERC165DerivedOrNot::ERC1155),
                false => unknown = true,
            }
        }
        if unknown {
            return Ok(ERC165DerivedOrNot::OTHER);
        }
        let mut reverted = false;
        let mut errors = vec![];
        if let Err(e) = erc721_match_res {
            reverted = e.is_revert();
            errors.push(e);
        }
        if let Err(e) = erc1155_match_res {
            reverted = e.is_revert();
            errors.push(e);
        }
        if reverted {
            return Ok(ERC165DerivedOrNot::OTHER);
        }
        Err(errors)
    }
}
