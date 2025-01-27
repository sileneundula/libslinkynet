use base32::*;
use crate::errors::{SlinkyErrors,EncodingErrorTypes};

pub struct SlinkyBase32z;

impl SlinkyBase32z {
    pub fn encode<T: AsRef<[u8]>>(bytes: T) -> String {
        return encode(Alphabet::Z,bytes.as_ref())
    }
    pub fn decode<T: AsRef<str>>(s: T) -> Result<Vec<u8>,SlinkyErrors> {
        let decoded_bytes: Option<Vec<u8>> = decode(Alphabet::Z,s.as_ref());

        if decoded_bytes.is_some() {
            return Ok(decoded_bytes.unwrap())
        }
        else {
            return Err(SlinkyErrors::EncodingError { encoding: EncodingErrorTypes::Base32z, context: crate::errors::SlinkyContext::Null, message: String::from("Failed To Decode From Base32z")})
        }
    }
}