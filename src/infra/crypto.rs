//! Crypto utilities for Ferox Phase 3 C2 scaffolding.
//!
//! Conservative, minimal wrappers around common primitives:
//! - AES-GCM (256-bit) for authenticated encryption
//! - HMAC-SHA256 for message authentication
//! - HKDF-SHA256 for key derivation (derive distinct subkeys from a seed)
//!
//! Design goals:
//! - Avoid exposing low-level misuse hazards (nonce length, key sizes)
//! - Return anyhow::Result with clear error contexts
//! - Keep dependencies lightweight (aes-gcm, hmac, sha2, hkdf)
//! - Provide deterministic test vectors
//!
//! TODO: Consider adding XChaCha20-Poly1305 for wider nonce space if needed.

use anyhow::{anyhow, Result};
use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Nonce};
use aes_gcm::aead::rand_core::RngCore; // bring trait for OsRng::fill_bytes
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hkdf::Hkdf;
// no external rand crate needed; we use rand_core re-exported via aes_gcm

/// Length constants
pub const AES_KEY_LEN: usize = 32; // 256-bit
pub const NONCE_LEN: usize = 12;   // 96-bit (AES-GCM standard)
pub const HMAC_KEY_LEN: usize = 32; // Using 256-bit key for HMAC-SHA256

/// Derived key set used for separating concerns (encryption vs integrity)
#[derive(Clone, Debug)]
pub struct DerivedKeys {
    pub enc_key: [u8; AES_KEY_LEN],
    pub hmac_key: [u8; HMAC_KEY_LEN],
}

/// Derive encryption and HMAC keys from a seed using HKDF-SHA256.
pub fn derive_keys(seed: &[u8], salt: &[u8]) -> Result<DerivedKeys> {
    let hk = Hkdf::<Sha256>::new(Some(salt), seed);
    let mut okm = [0u8; AES_KEY_LEN + HMAC_KEY_LEN];
    hk.expand(b"ferox-phase3", &mut okm).map_err(|_| anyhow!("HKDF expand failed"))?;

    let mut enc_key = [0u8; AES_KEY_LEN];
    enc_key.copy_from_slice(&okm[0..AES_KEY_LEN]);
    let mut hmac_key = [0u8; HMAC_KEY_LEN];
    hmac_key.copy_from_slice(&okm[AES_KEY_LEN..]);

    Ok(DerivedKeys { enc_key, hmac_key })
}

/// Generate a random AES-GCM nonce.
pub fn generate_nonce() -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Encrypt plaintext with AES-256-GCM using provided key and random nonce.
/// Returns (nonce, ciphertext). Caller should transmit both.
pub fn aes_encrypt(key: &[u8; AES_KEY_LEN], plaintext: &[u8], aad: &[u8]) -> Result<([u8; NONCE_LEN], Vec<u8>)> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| anyhow!("cipher init: {e}"))?;
    let nonce_bytes = generate_nonce();
    #[allow(deprecated)]
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ct = cipher.encrypt(nonce, aes_gcm::aead::Payload { msg: plaintext, aad }).map_err(|e| anyhow!("encrypt failed: {e}"))?;
    Ok((nonce_bytes, ct))
}

/// Decrypt ciphertext with AES-256-GCM.
pub fn aes_decrypt(key: &[u8; AES_KEY_LEN], nonce: &[u8; NONCE_LEN], ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| anyhow!("cipher init: {e}"))?;
    #[allow(deprecated)]
    let nonce = Nonce::from_slice(nonce);
    let pt = cipher.decrypt(nonce, aes_gcm::aead::Payload { msg: ciphertext, aad }).map_err(|e| anyhow!("decrypt failed: {e}"))?;
    Ok(pt)
}

/// Compute HMAC-SHA256.
pub fn hmac_sign(key: &[u8; HMAC_KEY_LEN], data: &[u8]) -> Vec<u8> {
    let mut mac = <Hmac<Sha256> as hmac::digest::KeyInit>::new_from_slice(key)
        .expect("HMAC key size valid");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// Verify HMAC-SHA256 in constant time.
pub fn hmac_verify(key: &[u8; HMAC_KEY_LEN], data: &[u8], expected: &[u8]) -> bool {
    let mut mac = <Hmac<Sha256> as hmac::digest::KeyInit>::new_from_slice(key)
        .expect("HMAC key size valid");
    mac.update(data);
    mac.verify_slice(expected).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation_and_hmac() {
        let seed = b"phase3-seed";
        let salt = b"ferox-salt";
        let keys = derive_keys(seed, salt).expect("keys");
        assert_ne!(keys.enc_key, [0u8; AES_KEY_LEN]);
        assert_ne!(keys.hmac_key, [0u8; HMAC_KEY_LEN]);

        let msg = b"test-message";
        let tag = hmac_sign(&keys.hmac_key, msg);
        assert!(hmac_verify(&keys.hmac_key, msg, &tag));
        // Negative test
        assert!(!hmac_verify(&keys.hmac_key, b"tampered", &tag));
    }

    #[test]
    fn test_aes_round_trip() {
        let seed = b"another-seed";
        let salt = b"another-salt";
        let keys = derive_keys(seed, salt).unwrap();

        let plaintext = b"secret payload";
        let aad = b"context";
        let (nonce, ct) = aes_encrypt(&keys.enc_key, plaintext, aad).unwrap();
        let recovered = aes_decrypt(&keys.enc_key, &nonce, &ct, aad).unwrap();
        assert_eq!(plaintext.to_vec(), recovered);
    }
}
