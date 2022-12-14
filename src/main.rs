mod lib;

use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use lib::BitcoinMiningFacility;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set default subscriber ");

    let mut bitcoin_mining_facility = BitcoinMiningFacility::new(10);
    bitcoin_mining_facility.run().await;
}
