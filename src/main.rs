pub mod address;
pub mod indexer;
pub mod prisma;
mod util;
use crate::util::parse_chain;
use tokio::join;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let chain = std::env::var("CHAIN").expect("CHAIN must be set");
    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set");
    let chain = parse_chain(&chain).unwrap();
    let core = indexer::new(chain, &rpc_url).await;
    join!(core.index_logs(), core.index_tokens());
}
