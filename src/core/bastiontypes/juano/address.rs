pub const JUANOINTERNETADDRESSPREFIX: &str = "JUANO";
pub const JUANOINTERNETADDRESSPREFIXProtocol: &str = "://";

pub struct JuanoInternetAddress(String);

impl JuanoInternetAddress {
    pub fn new<T: AsRef<str>>(s: T) -> Self {
        return Self(s.as_ref().to_string())
    }
    pub fn validate() {
        
    }
}