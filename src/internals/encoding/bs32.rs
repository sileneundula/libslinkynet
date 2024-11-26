use base32::*;

pub struct SlinkyBase32;

impl SlinkyBase32 {
    pub fn encode<T: AsRef<[u8]>>(bytes: T) -> str {
        return encode(Alphabet::Z,bytes.as_ref()).as_str()
    }
    pub fn decode<T: AsRef<str>>(s: T) -> Result<Vec<u8> {
        return decode(Alphabet::Z,s.as_ref())?
    }
}