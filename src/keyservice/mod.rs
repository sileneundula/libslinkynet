use libsumatracrypt_rs::{pq::signatures::{dilithium::SumatraDilithium3, falcon::SumatraFalcon1024}, signatures::{ed25519::{ED25519SecretKey, SumatraED25519, ED25519PublicKey}, schnorr::{SchnorrSecretKey, SumatraSchnorrAPI}}};
use crate::internals::crypto::{signature::schnorr::*};
use crate::internals::crypto::signature::pq::falcon1024::*;
use crate::internals::encoding::bs32::SlinkyBase32z;
use base58::*;

use crate::internals::crypto::hashing::blake3;
use crate::internals::crypto::hashing::shake256;

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
/// # Format
/// 
/// - All Public Keys and Secret Keys are encoded in upper-hexadecimal
/// - A Keypair is serialized to YAML 


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
#[derive(Clone,Debug,Serialize,Deserialize,Zeroize,ZeroizeOnDrop)]
pub struct PubKey(String,KeyAlgorithms);

/// SecretKey
pub struct SecKey(String,KeyAlgorithms);

/// Signature
#[derive(Clone,Debug,Serialize,Deserialize,Zeroize,ZeroizeOnDrop)]
pub struct Signature {
    sig: String,
    alg: KeyAlgorithms,
}

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
#[derive(Clone,Debug,Serialize,Deserialize,Zeroize,ZeroizeOnDrop)]
pub struct KeySID(String);

impl KeySID {
    pub fn derive_from_pubkey<T: AsRef<str>>(pk: T) -> Self {
        let digest: libsumatracrypt_rs::digest::SumatraDigest = blake3::SumatraBlake3::new(pk.as_ref().as_bytes());
        let sumatra_digest = digest.to_string().to_string();
        return Self(sumatra_digest)
    }
}

impl KeyPair {
    pub fn generate(alg: KeyAlgorithms) -> Self {
        let algorithm_id: u8 = match alg {
            KeyAlgorithms::ED25519 => 0u8,
            KeyAlgorithms::Schnorr => 1u8,
            KeyAlgorithms::PQFALCON => 8u8,
            KeyAlgorithms::PQDilithium3 => 9u8,
        };

        let keypair = match algorithm_id {
            0u8 => Self::generate_ed25519(),
            1u8 => Self::generate_schnorr(),
            8u8 => Self::generate_falcon(),
            9u8 => Self::generate_dilithium3(),
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
    pub fn sign<T: AsRef<[u8]>>(&self, bytes: T) -> Signature {
        let id: u8 = match self.alg {
            KeyAlgorithms::ED25519 => 0u8,
            KeyAlgorithms::Schnorr => 1u8,
            KeyAlgorithms::PQFALCON => 8u8,
            KeyAlgorithms::PQDilithium3 => 9u8,
        };

        if id == 0u8 {
            let sig = self.sign_ed25519(bytes);
            return sig
        }
        else {
            panic!("No Algorithm Chosen")
        }
    }
    fn sign_ed25519<T: AsRef<[u8]>>(&self, bytes: T) -> Signature {
        let m = bytes.as_ref();
        
        let sk = ED25519SecretKey::from_str(&self.sk);
        let pk = sk.to_public_key();
        
        let sig = sk.sign(m);

        Signature {
            sig: sig.to_string(),
            alg: KeyAlgorithms::ED25519,
        }

    }
    /// Returns the Algorithm Type
    pub fn algorithm(&self) -> KeyAlgorithms {
        return self.alg.clone()
    }
    /// From Decoded Bytes (Not Hexadecimal or encoded; actual bytes)
    pub fn from_plain_bytes<T: AsRef<[u8]>>(pk: T, sk: T, ka: KeyAlgorithms) -> Self {
        Self {
            pk: hex::encode_upper(pk),
            sk: hex::encode_upper(sk),
            alg: ka,
        }
    }
    pub fn from_base58_str<T: AsRef<str>>(sk: T, pk: T, ka: KeyAlgorithms) -> Self {
        let sk_bytes = sk.as_ref().from_base58().expect("Failed to get from base58");
        let pk_bytes = pk.as_ref().from_base58().expect("Failed To Get From Base58");

        let sk_final = hex::encode_upper(sk_bytes);
        let pk_final = hex::encode_upper(pk_bytes);

        Self {
            sk: sk_final,
            pk: pk_final,
            alg: ka,
        }
    }
    pub fn export_keypair_yaml(&self) -> String {
        serde_yaml::to_string(&self).expect("Failed To Serialize To YAML")
    }
    pub fn import_keypair_yaml<T: AsRef<str>>(keypair_yaml: T) -> Self {
        serde_yaml::from_str(keypair_yaml.as_ref()).expect("Failed To Import Keypair")
    }
}

impl PubKey {
    pub fn from_hex_str<T: AsRef<str>>(s: T, ka: KeyAlgorithms) -> Self {
        return Self(s.as_ref().to_string(), ka)
    }
    pub fn from_plain_bytes<T: AsRef<[u8]>>(bytes: T, ka: KeyAlgorithms) -> Self {
        return Self(hex::encode_upper(bytes.as_ref()),ka)
    }
}