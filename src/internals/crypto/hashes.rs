use libsumatracrypt_rs::digest::*;

pub struct PalisadeBLAKE3(String);
pub struct PalisadeBLAKE2B(String);

impl PalisadeBLAKE3 {
    pub fn new<T: AsRef<[u8]>>(bytes: T) -> SumatraDigest {
        return SumatraBlake3::new(bytes);
    }
}

impl PalisadeBLAKE2B {
    pub fn new<T: AsRef<[u8]>>(bytes: T, key: T, digest_size: usize) -> SumatraDigest {
        return SumatraBlake2b::new(bytes,key,digest_size)
    }
}