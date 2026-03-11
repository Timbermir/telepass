use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn sign(data: &str, token: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(b"WebAppData").unwrap();
    mac.update(token.as_bytes());
    let secret = mac.finalize().into_bytes();
    let mut mac2 = HmacSha256::new_from_slice(&secret).unwrap();
    mac2.update(data.as_bytes());
    hex::encode(mac2.finalize().into_bytes())
}
