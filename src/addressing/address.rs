pub struct ProtocolPrefix(String);

/// InternetAddress
pub struct InternetAddress(String);

impl InternetAddress {
    pub fn from_str<T: AsRef<str>>(s: T) -> Self {
        Self(s.as_ref().to_string())
    }
}

impl ProtocolPrefix {
    pub fn from_str<T: AsRef<str>>(s: T) -> Self {
        Self(s.as_ref().to_string())
    }
}