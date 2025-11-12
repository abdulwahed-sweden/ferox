//! DNS C2 helpers (scaffold)
//! Encode/decode helpers and a stub query interface.
//!
//! TODO: Implement real DNS query transport with trust-dns or system resolver

use anyhow::Result;
use data_encoding::{BASE32, BASE64};

pub fn encode_base32(data: &[u8]) -> String {
    BASE32.encode(data)
}
pub fn decode_base32(s: &str) -> Result<Vec<u8>> {
    Ok(BASE32.decode(s.as_bytes())?)
}

pub fn encode_base64(data: &[u8]) -> String {
    BASE64.encode(data)
}
pub fn decode_base64(s: &str) -> Result<Vec<u8>> {
    Ok(BASE64.decode(s.as_bytes())?)
}

/// Stub query function; returns Ok(None) to indicate no transport.
pub async fn dns_query_stub(_qname: &str) -> Result<Option<Vec<u8>>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base32_round_trip() {
        let s = encode_base32(b"ferox");
        let v = decode_base32(&s).unwrap();
        assert_eq!(v, b"ferox");
    }

    #[test]
    fn base64_round_trip() {
        let s = encode_base64(b"ferox");
        let v = decode_base64(&s).unwrap();
        assert_eq!(v, b"ferox");
    }
}
