pub trait PublicKey {
    fn verify(&self, sig)
    fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Self;
    fn from_str<T: AsRef<str>>(s: T) -> Self;
}