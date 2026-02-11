use super::types::{DerivedAccount, SecureSeed};

/// BIP84 derivation path for Bitcoin (Native SegWit): m/84'/0'/0'/0/{index}
fn bitcoin_derivation_path(index: u32) -> String {
    format!("m/84'/0'/0'/0/{}", index)
}

/// Derive Bitcoin accounts from seed
pub fn derive_bitcoin_accounts(
    seed: &SecureSeed,
    count: u32,
) -> Result<Vec<DerivedAccount>, String> {
    let mut accounts = Vec::new();

    for i in 0..count {
        let path = bitcoin_derivation_path(i);
        let account = derive_single_bitcoin_account(seed, &path, i)?;
        accounts.push(account);
    }

    Ok(accounts)
}

fn derive_single_bitcoin_account(
    seed: &SecureSeed,
    path: &str,
    index: u32,
) -> Result<DerivedAccount, String> {
    use bitcoin::bip32::{DerivationPath, Xpriv};
    use bitcoin::{Address, CompressedPublicKey, Network, NetworkKind, PrivateKey};
    use secp256k1::Secp256k1;

    let secp = Secp256k1::new();
    let master = Xpriv::new_master(NetworkKind::Main, &seed.bytes)
        .map_err(|e| format!("Failed to create master key: {}", e))?;

    let derivation_path: DerivationPath = path
        .parse()
        .map_err(|e| format!("Invalid derivation path: {}", e))?;

    let child = master
        .derive_priv(&secp, &derivation_path)
        .map_err(|e| format!("Failed to derive key: {}", e))?;

    // Create a Native SegWit (bech32) address
    let private_key = PrivateKey::new(child.private_key, Network::Bitcoin);
    let compressed = CompressedPublicKey::from_private_key(&secp, &private_key)
        .map_err(|e| format!("Failed to compress public key: {}", e))?;
    let address = Address::p2wpkh(&compressed, Network::Bitcoin);

    Ok(DerivedAccount {
        address: address.to_string(),
        chain: "bitcoin".to_string(),
        chain_name: "Bitcoin".to_string(),
        derivation_path: path.to_string(),
        index,
    })
}
