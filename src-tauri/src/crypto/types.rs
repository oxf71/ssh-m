use serde::Serialize;
use zeroize::Zeroize;

/// A derived account (public info only - no private keys)
#[derive(Debug, Clone, Serialize)]
pub struct DerivedAccount {
    pub address: String,
    pub chain: String,
    pub chain_name: String,
    pub derivation_path: String,
    pub index: u32,
}

/// All derived accounts from a single mnemonic
#[derive(Debug, Clone, Serialize)]
pub struct MultiChainAccounts {
    pub evm: Vec<DerivedAccount>,
    pub solana: Vec<DerivedAccount>,
    pub bitcoin: Vec<DerivedAccount>,
    pub source_vault: String,
    pub source_item: String,
}

/// A seed that auto-zeroizes on drop
pub struct SecureSeed {
    pub bytes: Vec<u8>,
}

impl Drop for SecureSeed {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

/// Parse mnemonic and generate seed
pub fn mnemonic_to_seed(mnemonic: &str) -> Result<SecureSeed, String> {
    let mn = bip39::Mnemonic::parse(mnemonic)
        .map_err(|e| format!("Invalid mnemonic: {}", e))?;
    let seed = mn.to_seed("");
    Ok(SecureSeed {
        bytes: seed.to_vec(),
    })
}
