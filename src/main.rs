pub mod address;
pub mod indexer;
use tokio::join;

const HTTP_URL: &'static str = "https://testnet.era.zksync.dev";

#[tokio::main]
async fn main() {
    let core = indexer::new(HTTP_URL);
    join!(core.excute(0));
}
