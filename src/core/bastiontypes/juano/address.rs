pub const JUANOINTERNETADDRESSPREFIX: &str = "JUANO";
pub const JUANOINTERNETADDRESSPREFIXProtocol: &str = "://";

pub struct JuanoInternetAddress(String);

impl JuanoInternetAddress {
    pub fn new<T: AsRef<str>>(s: T) -> Self {
        return s.as_ref().to_string()
    }
}