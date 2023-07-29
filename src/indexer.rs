use crate::{
    address::{ERC165DerivedOrNot, IdentifiableAddress},
    prisma,
};
use ethers::{
    core::types::Filter,
    providers::{Http, Middleware, Provider},
    types::Log,
};
use log::error;
use prisma::PrismaClient;
use std::{sync::Arc, time::Duration};

const EVENT_TRANSFER_ERC721: &str = "Transfer(address,address,uint256)";
const EVENT_TRANSFER_ERC1155_SINGLE: &str =
    "TransferSingle(address,address,address,uint256,uint256)";
const EVENT_TRANSFER_ERC1155_BATCH: &str =
    "TransferBatch(address,address,address,uint256[],uint256[])";

pub struct Indexer {
    pub rpc_client: Arc<Provider<Http>>,
    pub db_client: PrismaClient,
}

pub async fn new(url: &str) -> Indexer {
    let provider = Provider::<Http>::try_from(url).unwrap();
    let rpc_client = Arc::new(provider);
    let db_client = PrismaClient::_builder().build().await.unwrap();
    Indexer {
        rpc_client,
        db_client,
    }
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
            match self.rpc_client.get_logs(&filter).await {
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

    pub async fn dump_events(&self, log: &Log) {}

    pub async fn process_events(&self, log: &Log) {
        let addr = IdentifiableAddress {
            address: log.address,
        };

        match addr.check_standard(&self.rpc_client).await {
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
