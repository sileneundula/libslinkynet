use hex::*;

pub struct SlinkyHexEncoding;

impl SlinkyHexEncoding {
    pub fn encode<T: AsRef<[u8]>>(bytes: T) -> str {
        hex::encode_upper(bytes.as_ref()).as_str()
    }
    pub fn decode<T: AsRef<str>>(s: T) -> Result<Vec<u8>,FromHexError> {
        let decoded_hex: Result<Vec<u8>, FromHexError> = hex::decode(s.as_ref());

        if decoded_hex.is_ok() {
            return Ok(decoded_hex.unwrap())
        }
        else {
            return Err(decoded_hex.err())
        }

    }
}