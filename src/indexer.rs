use crate::address::{ERC165DerivedOrNot, IdentifiableAddress};
use ethers::{
    core::types::Filter,
    providers::{Http, Middleware, Provider},
    types::Log,
};
use log::error;
use std::{sync::Arc, time::Duration};

const EVENT_TRANSFER_ERC721: &str = "Transfer(address,address,uint256)";
const EVENT_TRANSFER_ERC1155_SINGLE: &str =
    "TransferSingle(address,address,address,uint256,uint256)";
const EVENT_TRANSFER_ERC1155_BATCH: &str =
    "TransferBatch(address,address,address,uint256[],uint256[])";

pub struct Indexer {
    pub client: Arc<Provider<Http>>,
}

pub fn new(url: &str) -> Indexer {
    let provider = Provider::<Http>::try_from(url).unwrap();
    let client = Arc::new(provider);
    Indexer { client }
}

impl Indexer {
    pub async fn excute(&self, start_block: u64) {
        let mut last_block = start_block;
        loop {
            let to_block = last_block + 100;
            let filter = Filter::new()
                .events(vec![
                    EVENT_TRANSFER_ERC721,
                    EVENT_TRANSFER_ERC1155_SINGLE,
                    EVENT_TRANSFER_ERC1155_BATCH,
                ])
                .from_block(last_block)
                .to_block(to_block);
            match self.client.get_logs(&filter).await {
                Ok(logs) => {
                    for log in logs.iter() {
                        // TODO: check process status and update block
                        self.process_events(log).await;
                    }
                }
                Err(e) => {
                    error!("Failed to get logs: {}", e);
                    continue;
                }
            }
            last_block = to_block;
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn process_events(&self, log: &Log) {
        let addr = IdentifiableAddress {
            address: log.address,
        };
        match addr.check_standard(&self.client).await {
            Ok(ERC165DerivedOrNot::ERC721) => {
                // TODO
            }
            Ok(ERC165DerivedOrNot::ERC1155) => {
                // TODO
            }
            Ok(ERC165DerivedOrNot::OTHER) => {
                // TODO
            }
            Err(e) => {
                error!("Failed to identify address: {:?}", e)
            }
        }
    }
}
