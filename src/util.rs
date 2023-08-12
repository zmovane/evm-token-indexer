use crate::prisma::Chain;

pub fn parse_chain(chain: &str) -> Option<Chain> {
    let chain = chain.to_uppercase().replace("-", "_");
    match chain.as_str() {
        "ZKSYNC_ERA_TESTNET" => Some(Chain::ZksyncEraTestnet),
        _ => Option::None,
    }
}
