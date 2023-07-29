use crate::{
    address::{ERC165DerivedOrNot, IdentifiableAddress},
    prisma::{
        self,
        logs::{self, Data},
    },
};
use ethers::{
    abi::AbiEncode,
    core::types::Filter,
    providers::{Http, Middleware, Provider},
    types::{Log, U64},
};
use log::error;
use prisma::PrismaClient;
use prisma_client_rust::QueryError;
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
        let mut last_block: u64 = start_block;
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
                    'dumplogs: for log in logs.iter() {
                        match self.dump_log(log).await {
                            Ok(_) => {
                                last_block =
                                    log.block_number.unwrap_or(U64::from(last_block)).as_u64();
                            }
                            Err(e) => {
                                error!("failed to dump log ({:?}) cause by {}", log, e);
                                break 'dumplogs;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to get logs: {}", e);
                    continue;
                }
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    pub async fn dump_log(&self, log: &Log) -> Result<Data, QueryError> {
        let block_number = log.block_number.unwrap().as_u64() as i64;
        let log_index = log.log_index.unwrap().encode_hex();
        let tx_hash = log.transaction_hash.unwrap().encode_hex();
        let data = log.data.to_vec();
        self.db_client
            .logs()
            .upsert(
                logs::block_number_log_index(block_number, log_index.to_owned()),
                logs::create(tx_hash, block_number, log_index, data, vec![]),
                vec![],
            )
            .exec()
            .await
    }

    pub async fn process_logs(&self, log: &Log) {
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
