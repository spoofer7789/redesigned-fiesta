use secp256k1::{PublicKey, SecretKey};
use anyhow::{Result};
use secp256k1::rand::{rngs, SeedableRng};
use hex;
use std::env;

pub fn generate_ether_wallet() -> Result<(SecretKey, PublicKey), secp256k1::Error> {
    let secp = secp256k1::Secp256k1::new();
    let seed_str = env::var("SEED").unwrap_or_else(|_| String::from("6"));
    let seed: u64 = seed_str.parse().unwrap_or(6);
    let mut rng = rngs::StdRng::seed_from_u64(seed);
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    // Print the keys
    println!("Secret Key: {}", hex::encode(&secret_key[..]));
    println!("Public Key: {}", hex::encode(&public_key.serialize()[..]));

    Ok((secret_key, public_key))
}