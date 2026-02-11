use alloy::signers::local::PrivateKeySigner;

use super::types::{DerivedAccount, SecureSeed};

/// BIP44 derivation path for Ethereum: m/44'/60'/0'/0/{index}
fn evm_derivation_path(index: u32) -> String {
    format!("m/44'/60'/0'/0/{}", index)
}

/// Derive EVM (Ethereum-compatible) accounts from seed
pub fn derive_evm_accounts(seed: &SecureSeed, count: u32) -> Result<Vec<DerivedAccount>, String> {
    let mut accounts = Vec::new();

    for i in 0..count {
        let path = evm_derivation_path(i);

        // Use alloy's MnemonicBuilder-like approach via HD derivation
        // For now, we derive using the seed directly with BIP32
        let account = derive_single_evm_account(seed, &path, i)?;
        accounts.push(account);
    }

    Ok(accounts)
}

fn derive_single_evm_account(
    seed: &SecureSeed,
    path: &str,
    index: u32,
) -> Result<DerivedAccount, String> {
    // Use bitcoin crate's BIP32 for HD derivation, then convert to ethereum address
    use bitcoin::bip32::{DerivationPath, Xpriv};
    use bitcoin::NetworkKind;
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

    // Get the private key bytes and create an Ethereum signer
    let private_key_bytes = child.private_key.secret_bytes();
    let signer = PrivateKeySigner::from_slice(&private_key_bytes)
        .map_err(|e| format!("Failed to create ETH signer: {}", e))?;

    let address = format!("{:?}", signer.address());

    Ok(DerivedAccount {
        address,
        chain: "evm".to_string(),
        chain_name: "Ethereum".to_string(),
        derivation_path: path.to_string(),
        index,
    })
}
