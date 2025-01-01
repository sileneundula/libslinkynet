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

pub struct PublicKey {
    pk: String,
}
pub struct SecretKey;
pub struct Signature(String);


pub struct KeyAlgorithm;

pub enum KeyAlgorithms {
    ED25519,
    Schnorr,
}

pub struct KeySID(String);

impl PublicKey {

}