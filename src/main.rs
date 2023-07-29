pub mod address;
pub mod indexer;
pub mod prisma;
use tokio::join;

const HTTP_URL: &'static str = "https://testnet.era.zksync.dev";

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let core = indexer::new(HTTP_URL);
    join!(core.excute(0));
}
