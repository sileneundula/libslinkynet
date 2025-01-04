use libsumatracrypt_rs::{pq::signatures::{dilithium::SumatraDilithium3, falcon::SumatraFalcon1024}, signatures::{ed25519::{ED25519SecretKey, SumatraED25519}, schnorr::{SchnorrSecretKey, SumatraSchnorrAPI}}};
use crate::internals::crypto::signature::schnorr::*;
use crate::internals::crypto::signature::pq::falcon1024::*;


use crate::internals::serde::{Serialize,Deserialize};
use zeroize::{Zeroize,ZeroizeOnDrop};
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

/// # Keypair Struct
#[derive(Clone,Debug,Serialize,Deserialize,Zeroize,ZeroizeOnDrop)]
pub struct KeyPair {
    pk: String,
    sk: String,
    alg: KeyAlgorithms,
}

/// PublicKey
pub struct PubKey(String,KeyAlgorithms);

/// SecretKey
pub struct SecKey(String,KeyAlgorithms);

/// Signature
pub struct Signature(String,KeyAlgorithms);

/// KeyAlgorithm
pub struct KeyAlgorithm;


#[derive(Clone,Debug,Serialize,Deserialize,Zeroize,ZeroizeOnDrop)]
/// KeyAlgorithms
pub enum KeyAlgorithms {
    ED25519,
    Schnorr,
    
    PQFALCON,
    PQDilithium3,
}

/// KeySID derived from PublicKey
pub struct KeySID(String);

impl KeyPair {
    pub fn generate(alg: KeyAlgorithms) -> Self {
        let algorithm_id: u8 = match alg {
            KeyAlgorithms::ED25519 => 0u8,
            KeyAlgorithms::Schnorr => 1u8,
            KeyAlgorithms::PQFALCON => 8u8,
        };

        let keypair = match algorithm_id {
            0u8 => Self::generate_ed25519(),
            1u8 => Self::generate_schnorr(),
            8u8 => Self::generate_falcon(),
            _ => panic!("No Keypair Chosen")
        };
        return keypair
    }
    fn generate_ed25519() -> Self {
        let sk = SumatraED25519::new();
        let sk_str = sk.to_string();
        let pk = sk.to_public_key().to_string();
        
        Self {
            sk: sk_str,
            pk: pk,
            alg: KeyAlgorithms::ED25519,
        }
    }
    fn generate_schnorr() -> Self {
        let (pk,sk) = SchnorrPublicKey::generate();

        Self {
            pk: pk.public_key(),
            sk: sk.secret_key(),
            alg: KeyAlgorithms::Schnorr,
        }
    }
    fn generate_falcon() -> Self {
        let (sk,pk) = SumatraFalcon1024::new();

        Self {
            pk: pk.to_string(),
            sk: sk.to_string(),
            alg: KeyAlgorithms::PQFALCON,
        }
    }
    fn generate_dilithium3() -> Self {
        let (sk,pk) = SumatraDilithium3::new();

        return Self {
            pk: pk.to_string(),
            sk: sk.to_string(),
            alg: KeyAlgorithms::PQDilithium3,
        }
    }
    pub fn from_hex_str<T: AsRef<str>>(sk: T, pk: T, alg: KeyAlgorithms) -> Self {
        return Self {
            sk: sk.as_ref().to_string(),
            pk: pk.as_ref().to_string(),
            alg: alg,
        }
    }   
        
        

}

}

impl SecKey {
    pub fn generate(alg: KeyAlgorithms) -> Self {
        let alg_id: u8 = match alg {
            KeyAlgorithms::ED25519 => 0u8,
            KeyAlgorithms::Schnorr => 1u8,
            _ => panic!("No Value")
        };

        if alg_id == 0 {
            Self(Self::generate_ed25519().to_string(),KeyAlgorithms::ED25519)
        }
        else if alg_id == 1 {
            Self(Self::generate_schnorr().secret_key(),KeyAlgorithms::Schnorr)
        }
        else {
            panic!("Nowhere to go.")
        }


    }
    fn generate_ed25519() -> ED25519SecretKey {
        SumatraED25519::new()
    }
    fn generate_schnorr() -> SchnorrSecretKey {
        let (_,sk) = SchnorrPublicKey::generate();
        return sk
    }
}

