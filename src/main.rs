pub mod address;
pub mod indexer;
pub mod prisma;
use tokio::join;

use crate::prisma::Chain;

const HTTP_URL: &'static str = "https://testnet.era.zksync.dev";

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let core = indexer::new(Chain::ZksyncEraTestnet, HTTP_URL).await;
    join!(core.index_logs(), core.index_tokens());
}
