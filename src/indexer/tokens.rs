use super::Indexer;
use crate::{
    address::{ERC165DerivedOrNot, IdentifiableAddress},
    prisma::{
        logs::{self},
        states,
        tokens::{self},
        IndexedType, Standard,
    },
};
use ethers::{
    abi::AbiEncode,
    abi::{decode, ParamType},
    types::{H160, H256},
};
use log::{error, info};
use prisma_client_rust::{raw, PrismaValue, QueryError};
use regex::Regex;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
struct NextBlock {
    block_number: i64,
}

impl Indexer {
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

        let res = addr.check_standard(&self.rpc_client).await;
        if let Err(e) = res {
            error!("Failed to identify address: {:?}", e);
            return Err(());
        };
        let to: String;
        let standard: Standard;
        let token_id: String;
        match res.unwrap() {
            ERC165DerivedOrNot::OTHER => return Ok((block_number, false)),
            ERC165DerivedOrNot::ERC1155 => {
                standard = Standard::Erc1155;
                let res =
                    decode(&vec![ParamType::Uint(256), ParamType::Uint(256)], &value).unwrap();
                info!("erc1155 {:?}", res[0].to_string());
                to = topics[3].to_string();
                token_id = res[0].to_string();
            }
            ERC165DerivedOrNot::ERC721 => {
                standard = Standard::Erc721;
                to = topics[2].to_string();
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
