pub struct PublicKey(String);
pub struct SecretKey(String);
pub struct Signature(String);

pub struct KeyMeta {
    version: KeyVersion,
    alg: KeyAlgorithm,
    encoding: KeyEncoding,
}

pub enum KeyVersion {
    DefaultTest,
}

pub enum KeyEncoding {
    Hex,
    Base32,
    Base58,
    Base64,
}

pub enum KeyAlgorithm {
    Pk_ED25519,
    Pk_Schnorr,

    Pk_RSA,
    // PQ
    Pk_Dilithium3,
    Pk_Falcon1024,
}