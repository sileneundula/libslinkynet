/// # SlinkrVersion
/// 
/// Number Type: u32
pub struct SlinkrVersion(u32);

impl SlinkrVersion {
    pub fn new(version: u32) -> Self {
        Self(version)
    }
    pub fn get_version(&self) -> u32 {
        self.0
    }
}

/// # SlinkrKeypairTypes
/// 
/// Keypair Algorithms (Chooses ED25519 by default):
/// 
/// - ECDSA
/// - SECP256k1
/// - ED25519
pub enum SlinkrKeypairTypes {
    ECDSA,
    SECP256k1,
    ED25519,
}

/// # GenerateKeypair (Type)
/// 
/// Boolean
pub type GenerateKeypair = bool;