pub mod logs;
pub mod tokens;
use crate::prisma::{self, states, Chain, IndexedType};
use ethers::providers::{Http, Provider};
use log::error;
use prisma::PrismaClient;
use std::{process, sync::Arc};

pub struct Indexer {
    chain: Chain,
    rpc_client: Arc<Provider<Http>>,
    db_client: PrismaClient,
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

impl Indexer {
    pub async fn get_indexed_block(&self, indexed_type: IndexedType) -> i64 {
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
}
