use thiserror::Error;

#[derive(Error, Debug)]
pub enum SlinkyErrors {
    #[error("Encoding or Decoding Error Using {:?} On {:?}", encoding, context)]
    EncodingError {
        encoding: EncodingErrorTypes,
        context: SlinkyContext,

        message: String,
    }
}

#[derive(Debug)]

pub enum EncodingErrorTypes {
    Hex,
    Base32z,
    Base58,
    Base64,
}

#[derive(Debug)]
pub enum SlinkyContext {
    PublicKey,
    SecretKey,
    Signature,
    Digest,
    
    Null,
}