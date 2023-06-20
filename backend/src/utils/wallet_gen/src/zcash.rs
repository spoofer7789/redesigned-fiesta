use orchard::keys::{SpendingKey, FullViewingKey, Scope};
use rand::Rng;

pub fn generate_zcash_wallet() -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 32];
    rng.fill(&mut random_bytes);

    let sk: Option<SpendingKey> = SpendingKey::from_bytes(random_bytes).into();
    let sk = sk.expect("Failed to create spending key from bytes");
    let full_viewing_key = FullViewingKey::from(&sk);
    let address = full_viewing_key.address_at(0u32, Scope::External);

    (format!("{:?}", sk), format!("{:?}", address))
}

pub fn generate_extra_addresses(sk: SpendingKey, num_addresses: u32) -> Vec<String> {
    let full_viewing_key = FullViewingKey::from(&sk);
    let mut addresses = Vec::new();
    for i in 0..num_addresses {
        let address = full_viewing_key.address_at(i, Scope::External);
        addresses.push(format!("{:?}", address));
    }
    addresses
}
