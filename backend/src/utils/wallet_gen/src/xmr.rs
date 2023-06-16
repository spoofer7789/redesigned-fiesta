use rand::Rng;
use hex;
use std::env;
use monero::{Address, KeyPair, PrivateKey, PublicKey };


pub fn generate_xmr_keypair() -> KeyPair {
    let mut rng = rand::thread_rng();

    let secret_view_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let secret_view = PrivateKey::from_slice(&secret_view_bytes).unwrap();

    let secret_spend_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let secret_spend = PrivateKey::from_slice(&secret_spend_bytes).unwrap();

    KeyPair {
        view: secret_view,
        spend: secret_spend,
    }
}

pub fn generate_address() -> Address {
    let keypair = generate_xmr_keypair();
    Address::from_keypair(monero::Network::Mainnet, &keypair)

}
pub fn generate_xmr_wallet() -> (KeyPair, Address) {
    let keypair = generate_xmr_keypair();
    let address = Address::from_keypair(monero::Network::Mainnet, &keypair);

    println!("Private Key: {:?}", keypair);
    println!("Public Address: {:?}", address);
    
    (keypair, address)
}