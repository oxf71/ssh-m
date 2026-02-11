use super::types::{DerivedAccount, SecureSeed};
use solana_sdk::signer::Signer;

/// BIP44 derivation path for Solana: m/44'/501'/{index}'/0'
fn solana_derivation_path(index: u32) -> String {
    format!("m/44'/501'/{}'/0'", index)
}

/// Derive Solana accounts from seed
pub fn derive_solana_accounts(
    seed: &SecureSeed,
    count: u32,
) -> Result<Vec<DerivedAccount>, String> {
    let mut accounts = Vec::new();

    for i in 0..count {
        let path = solana_derivation_path(i);
        let account = derive_single_solana_account(seed, &path, i)?;
        accounts.push(account);
    }

    Ok(accounts)
}

fn derive_single_solana_account(
    seed: &SecureSeed,
    path: &str,
    index: u32,
) -> Result<DerivedAccount, String> {
    use bitcoin::bip32::{DerivationPath, Xpriv};
    use bitcoin::NetworkKind;
    use secp256k1::Secp256k1;

    // Solana uses Ed25519, but we can derive the key path using BIP32
    // then use the derived bytes as Ed25519 seed
    // Standard Solana wallets (Phantom, etc.) use derivation similar to:
    // Take the first 32 bytes of the derived key as Ed25519 seed

    let secp = Secp256k1::new();
    let master = Xpriv::new_master(NetworkKind::Main, &seed.bytes)
        .map_err(|e| format!("Failed to create master key: {}", e))?;

    let derivation_path: DerivationPath = path
        .parse()
        .map_err(|e| format!("Invalid derivation path: {}", e))?;

    let child = master
        .derive_priv(&secp, &derivation_path)
        .map_err(|e| format!("Failed to derive key: {}", e))?;

    // Use the derived private key bytes as Ed25519 seed
    let secret_bytes = child.private_key.secret_bytes();
    let keypair = solana_sdk::signer::keypair::keypair_from_seed(&secret_bytes)
        .map_err(|e| format!("Failed to create Solana keypair: {}", e))?;

    let address = keypair.pubkey().to_string();

    Ok(DerivedAccount {
        address,
        chain: "solana".to_string(),
        chain_name: "Solana".to_string(),
        derivation_path: path.to_string(),
        index,
    })
}
