use thiserror::Error;

#[derive(Error, Debug)]
pub enum SlinkyErrors {
    #[error("Encoding or Decoding Error Using {} On {}", encoding, context)]
    EncodingError {
        encoding: EncodingErrorTypes,
        context: SlinkyContext,

        message: String,
    }
}


pub enum EncodingErrorTypes {
    Hex,
    Base32z,
    Base58,
    Base64,
}

pub enum SlinkyContext {
    PublicKey,
    SecretKey,
    Signature,
    Digest,
}