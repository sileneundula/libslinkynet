use libsumatracrypt_rs::{pq::signatures::falcon::SumatraFalcon1024, signatures::{ed25519::{ED25519SecretKey, SumatraED25519}, schnorr::{SchnorrSecretKey, SumatraSchnorrAPI}}};
use crate::internals::crypto::signature::schnorr::*;
/// # KeyService
/// 
/// A Permanent Storage of Keys. Keys are stored here and the KeySID is derived from them.
/// 
/// Supported Keys:
/// - ED25519
/// - Schnorr
/// 
/// - [PQ] Falcon1024
/// 
/// Keypairs are encoded in hexadecimal and signatures are encoded in base58.
/// 
/// 


/// Node Implementation of KeyService
pub mod node;

/// PublicKey
pub struct PubKey(String,KeyAlgorithms);

/// SecretKey
pub struct SecKey(String,KeyAlgorithms);

/// Signature
pub struct Signature(String,KeyAlgorithms);

/// KeyAlgorithm
pub struct KeyAlgorithm;

/// KeyAlgorithms
pub enum KeyAlgorithms {
    ED25519,
    Schnorr,
}

/// KeySID derived from PublicKey
pub struct KeySID(String);

impl SecKey {
    pub fn generate(alg: KeyAlgorithms) {
        let alg_id: u8 = match alg {
            KeyAlgorithms::ED25519 => 0u8,
            KeyAlgorithms::Schnorr => 1u8,
            _ => panic!("No Value")
        };


    }
    fn generate_ed25519() -> ED25519SecretKey {
        SumatraED25519::new()
    }
    fn generate_schnorr() -> SchnorrSecretKey {
        let (_,sk) = SchnorrPublicKey::generate();
        return sk
    }
}

