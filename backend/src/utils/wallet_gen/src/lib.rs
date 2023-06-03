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

pub fn generate_ethereum_wallet() -> (String, String) {
    let keypair = ethsign::KeyPair::generate().unwrap();
    let secret = keypair.secret();
    let address = keypair.address();
    
    (format!("{:?}", secret), format!("{:?}", address))
}

pub fn generate_bitcoin_wallet() -> (String, String) {
    let secp = secp256k1::Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    let secret = bitcoin::PrivateKey {
        compressed: true,
        network: bitcoin::Network::Bitcoin,
        key: secret_key,
    };

    let address = bitcoin::Address::p2pkh(&public_key, bitcoin::Network::Bitcoin);

    (format!("{:?}", secret), format!("{:?}", address))
}
