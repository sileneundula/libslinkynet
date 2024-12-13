pub const LAMENTDELIMITER: str = "::";

pub struct LamentLabel(String);
pub struct LamentPath(String);

impl LamentLabel {
    pub fn new<T: AsRef<str>>(s: T) -> Self {
        return s.as_ref().to_string()
    }
}