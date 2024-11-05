use secp256k1::{Secp256k1, SecretKey};
use rand::rngs::OsRng;
use bitcoin::util::key::PrivateKey;
use bitcoin::util::address::Address;
use bitcoin::Network;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Utxo {
    value: u64, // Valor de cada UTXO en satoshis
}

/// Función para generar una clave privada de Bitcoin
fn generate_private_key() -> PrivateKey {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let secret_key = SecretKey::new(&mut rng);
    PrivateKey::new(secret_key, Network::Bitcoin)
}

/// Función para obtener la dirección pública de Bitcoin a partir de la clave privada
fn get_address_from_private_key(private_key: &PrivateKey) -> Address {
    let secp = Secp256k1::new();
    let pubkey = private_key.public_key(&secp);
    Address::p2pkh(&pubkey, private_key.network)
}

/// Función para consultar el balance de la dirección usando Blockstream API
async fn get_balance(address: &Address) -> Result<u64, Box<dyn std::error::Error>> {
    let url = format!("https://blockstream.info/api/address/{}/utxo", address);
    let response = reqwest::get(&url).await?.json::<Vec<Utxo>>().await?;

    // Sumamos el valor de todos los UTXOs para obtener el balance total en satoshis
    let balance: u64 = response.iter().map(|utxo| utxo.value).sum();
    Ok(balance)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generar la clave privada y la dirección de Bitcoin
    let private_key = generate_private_key();
    let address = get_address_from_private_key(&private_key);

    println!("Clave privada: {:?}", private_key);
    println!("Dirección de Bitcoin: {}", address);

    // Consultar el balance de la dirección
    match get_balance(&address).await {
        Ok(balance) => println!("Balance en satoshis: {}", balance),
        Err(e) => eprintln!("Error al obtener el balance: {}", e),
    }

    Ok(())
}