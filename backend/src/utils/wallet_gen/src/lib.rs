mod eth;
mod btc;
 mod zcash;
mod xmr;

pub use eth::generate_ether_wallet;
pub use btc::generate_bitcoin_wallet;
pub use zcash::generate_zcash_wallet;
pub use xmr::generate_xmr_wallet;