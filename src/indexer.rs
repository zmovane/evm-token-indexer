use crate::{
    address::{ERC165DerivedOrNot, IdentifiableAddress},
    prisma::{
        self,
        logs::{self},
        states,
        tokens::{self},
        Chain, IndexedType, Standard,
    },
};
use ethers::{
    abi::AbiEncode,
    core::types::Filter,
    providers::{Http, Middleware, Provider},
    types::{Log, H160, H256},
};
use log::error;
use prisma::PrismaClient;
use prisma_client_rust::{raw, PrismaValue, QueryError};
use regex::Regex;
use serde::Deserialize;
use std::{process, str::FromStr, sync::Arc, time::Duration};

const EVENT_TRANSFER_ERC721: &str = "Transfer(address,address,uint256)";
const EVENT_TRANSFER_ERC1155_SINGLE: &str =
    "TransferSingle(address,address,address,uint256,uint256)";
const EVENT_TRANSFER_ERC1155_BATCH: &str =
    "TransferBatch(address,address,address,uint256[],uint256[])";

pub struct Indexer {
    pub chain: Chain,
    pub rpc_client: Arc<Provider<Http>>,
    pub db_client: PrismaClient,
    max_blocks_per_query: u64,
}

pub async fn new(chain: Chain, url: &str) -> Indexer {
    let provider = Provider::<Http>::try_from(url).unwrap();
    let rpc_client = Arc::new(provider);
    let db_client = PrismaClient::_builder().build().await.unwrap();
    Indexer {
        chain,
        rpc_client,
        db_client,
        max_blocks_per_query: 1000,
    }
}

#[derive(Deserialize)]
struct NextBlock {
    block_number: i64,
}

impl Indexer {
    async fn get_indexed_block(&self, indexed_type: IndexedType) -> i64 {
        let res = self
            .db_client
            .states()
            .find_unique(states::chain_indexed_type(self.chain, indexed_type))
            .exec()
            .await
            .unwrap();
        if let None = res {
            error!(
                "Indexed block not found for {:?} {:?}",
                self.chain, indexed_type
            );
            process::exit(1);
        }
        res.unwrap().indexed_block
    }

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

    async fn next_block(&self, current_block: i64) -> i64 {
        let res: Result<Vec<NextBlock>, QueryError> = self.db_client
        ._query_raw(
            raw!(
                "SELECT DISTINCT block_number FROM \"Logs\" WHERE block_number > $1::BIGINT ORDER BY block_number ASC LIMIT 1",
                PrismaValue::BigInt(current_block)
            )
        )
        .exec()
        .await;
        if let Err(e) = res {
            error!("failed to get next block: {}", e);
            return current_block;
        }
        let data = res.unwrap();
        if data.len() == 0 {
            return current_block;
        }
        data[0].block_number
    }

    pub async fn index_tokens(&self) {
        let mut last_block = self.get_indexed_block(IndexedType::Token).await;
        loop {
            let data = self
                .db_client
                .logs()
                .find_many(vec![logs::block_number::equals(last_block)])
                .order_by(logs::OrderByParam::LogIndex(
                    prisma_client_rust::Direction::Asc,
                ))
                .exec()
                .await
                .unwrap();
            if data.len() == 0 {
                let indexed_logs_block = self.get_indexed_block(IndexedType::Log).await;
                if last_block < indexed_logs_block {
                    last_block = self.next_block(last_block).await;
                }
                continue;
            }
            let mut try_next = true;
            'dumptoken: for log in data {
                match self.dump_token(&log).await {
                    Ok((indexed_block, _)) => {
                        last_block = indexed_block;
                    }
                    Err(_) => {
                        try_next = false;
                        break 'dumptoken;
                    }
                }
            }
            if try_next {
                let _ = self
                    .db_client
                    .states()
                    .update(
                        states::chain_indexed_type(self.chain, IndexedType::Token),
                        vec![states::indexed_block::set(last_block)],
                    )
                    .exec()
                    .await;
                last_block = self.next_block(last_block).await;
            }
        }
    }

    fn h256_to_h160(&self, h256: H256) -> H160 {
        let h256_hex = h256.encode_hex();
        let pattern: Regex = Regex::new("^0x0{24}").unwrap();
        let h160_hex = pattern.replace(h256_hex.as_str(), "0x").to_string();
        H160::from_str(h160_hex.as_str()).unwrap()
    }

    pub async fn dump_token(&self, log: &logs::Data) -> Result<(i64, bool), ()> {
        let address = self.h256_to_h160(H256::from_str(&log.address).unwrap());
        let addr = IdentifiableAddress { address };
        let block_number = log.block_number;
        let value = log.data.to_vec();
        let topics = &log.topics;
        let to = &topics[2];
        let res = addr.check_standard(&self.rpc_client).await;
        if let Err(e) = res {
            error!("Failed to identify address: {:?}", e);
            return Err(());
        };
        let standard;
        let token_id: String;
        match res.unwrap() {
            ERC165DerivedOrNot::OTHER => return Ok((block_number, false)),
            ERC165DerivedOrNot::ERC1155 => {
                standard = Standard::Erc1155;
                token_id = H256::from_slice(&value).encode_hex();
            }
            ERC165DerivedOrNot::ERC721 => {
                standard = Standard::Erc721;
                token_id = topics[3].to_string();
            }
        };

        let contract = &log.address;
        let res = self
            .db_client
            ._transaction()
            .run(|tx| async move {
                tx.tokens()
                    .upsert(
                        tokens::chain_token_id_contract(
                            self.chain,
                            token_id.to_owned(),
                            contract.to_owned(),
                        ),
                        tokens::create(
                            self.chain,
                            token_id,
                            contract.to_owned(),
                            to.to_owned(),
                            standard,
                            vec![],
                        ),
                        vec![],
                    )
                    .exec()
                    .await?;
                tx.states()
                    .update(
                        states::chain_indexed_type(self.chain, IndexedType::Token),
                        vec![states::indexed_block::set(block_number)],
                    )
                    .exec()
                    .await
                    .map(|state| (state.indexed_block, true))
            })
            .await;
        if let Err(e) = res {
            error!("Failed to upsert data: {}", e);
            return Err(());
        }
        Ok(res.unwrap())
    }
}
