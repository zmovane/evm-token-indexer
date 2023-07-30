use crate::prisma::{
    logs::{self},
    states, IndexedType,
};
use ethers::{abi::AbiEncode, core::types::Filter, providers::Middleware, types::Log};
use log::error;
use prisma_client_rust::QueryError;
use std::time::Duration;

use super::Indexer;

const EVENT_TRANSFER_ERC721: &str = "Transfer(address,address,uint256)";
const EVENT_TRANSFER_ERC1155_SINGLE: &str =
    "TransferSingle(address,address,address,uint256,uint256)";
const EVENT_TRANSFER_ERC1155_BATCH: &str =
    "TransferBatch(address,address,address,uint256[],uint256[])";

impl Indexer {
    pub async fn index_logs(&self) {
        let mut last_block: u64 = self.get_indexed_block(IndexedType::Log).await as u64;
        loop {
            let to_block = last_block + self.max_blocks_per_query;
            let filter = Filter::new()
                .events(vec![
                    EVENT_TRANSFER_ERC721,
                    EVENT_TRANSFER_ERC1155_SINGLE,
                    EVENT_TRANSFER_ERC1155_BATCH,
                ])
                .from_block(last_block)
                .to_block(to_block);
            let res = self.rpc_client.get_logs(&filter).await;
            if let Err(e) = res {
                error!("Failed to get logs: {}", e);
                continue;
            }
            let logs = res.unwrap();
            'dumplogs: for log in logs.iter() {
                match self.dump_log(log).await {
                    Ok(indexed_block) => {
                        last_block = indexed_block as u64;
                    }
                    Err(e) => {
                        error!("failed to dump log ({:?}) cause by {}", log, e);
                        break 'dumplogs;
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    pub async fn dump_log(&self, log: &Log) -> Result<i64, QueryError> {
        let block_number = log.block_number.unwrap().as_u64() as i64;
        let log_index = log.log_index.unwrap().as_u64() as i64;
        let tx_hash = log.transaction_hash.unwrap().encode_hex();
        let address = log.address.encode_hex();
        let topics = log.topics.iter().map(|x| x.encode_hex()).collect();
        let data = log.data.to_vec();
        self.db_client
            ._transaction()
            .run(|tx| async move {
                tx.logs()
                    .upsert(
                        logs::block_number_log_index(block_number, log_index),
                        logs::create(
                            tx_hash,
                            block_number,
                            log_index,
                            address,
                            data,
                            vec![logs::topics::set(topics)],
                        ),
                        vec![],
                    )
                    .exec()
                    .await?;
                tx.states()
                    .update(
                        states::chain_indexed_type(self.chain, IndexedType::Log),
                        vec![states::indexed_block::set(block_number)],
                    )
                    .exec()
                    .await
                    .map(|state| state.indexed_block)
            })
            .await
    }
}
