//! Data Encoding and Chunking for Exfiltration
//!
//! Provides encoding, encryption, and chunking for covert data transfer.
//!
//! MITRE ATT&CK: T1048 (Exfiltration Over Alternative Protocol)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};

/// Encoding method for exfiltration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EncodingMethod {
    /// Base64 encoding
    #[default]
    Base64,
    /// Base32 encoding (DNS-safe)
    Base32,
    /// Hexadecimal encoding
    Hex,
    /// URL-safe Base64
    Base64Url,
    /// Custom dictionary encoding
    Dictionary,
    /// Raw bytes (no encoding)
    Raw,
}

impl EncodingMethod {
    /// Get expansion ratio (encoded size / raw size)
    pub fn expansion_ratio(&self) -> f32 {
        match self {
            Self::Base64 | Self::Base64Url => 1.33,
            Self::Base32 => 1.6,
            Self::Hex => 2.0,
            Self::Dictionary => 1.5,
            Self::Raw => 1.0,
        }
    }

    /// Is this encoding DNS-safe?
    pub fn is_dns_safe(&self) -> bool {
        matches!(self, Self::Base32 | Self::Hex)
    }
}

/// Compression method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CompressionMethod {
    #[default]
    None,
    Gzip,
    Zlib,
    Lz4,
}

/// Encryption method for exfil data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EncryptionMethod {
    #[default]
    None,
    Xor,
    Aes256Gcm,
    ChaCha20,
}

/// Data chunk for transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataChunk {
    pub sequence: u32,
    pub total_chunks: u32,
    pub data: Vec<u8>,
    pub checksum: u32,
    pub session_id: String,
}

impl DataChunk {
    /// Create new data chunk
    pub fn new(sequence: u32, total: u32, data: Vec<u8>, session_id: &str) -> Self {
        let checksum = Self::calculate_checksum(&data);
        Self {
            sequence,
            total_chunks: total,
            data,
            checksum,
            session_id: session_id.to_string(),
        }
    }

    /// Calculate CRC32 checksum
    fn calculate_checksum(data: &[u8]) -> u32 {
        let mut crc: u32 = 0xFFFFFFFF;
        for byte in data {
            crc ^= *byte as u32;
            for _ in 0..8 {
                if crc & 1 == 1 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }
        !crc
    }

    /// Verify checksum
    pub fn verify(&self) -> bool {
        Self::calculate_checksum(&self.data) == self.checksum
    }

    /// Encode chunk for transmission
    pub fn encode(&self, method: EncodingMethod) -> String {
        match method {
            EncodingMethod::Base64 => base64_encode(&self.data),
            EncodingMethod::Base64Url => base64_url_encode(&self.data),
            EncodingMethod::Base32 => base32_encode(&self.data),
            EncodingMethod::Hex => hex_encode(&self.data),
            EncodingMethod::Raw => String::from_utf8_lossy(&self.data).to_string(),
            EncodingMethod::Dictionary => dictionary_encode(&self.data),
        }
    }
}

/// Data Encoder Engine
#[derive(Debug, Clone)]
pub struct DataEncoder {
    encoding: EncodingMethod,
    encryption: EncryptionMethod,
    compression: CompressionMethod,
    chunk_size: usize,
    encryption_key: Vec<u8>,
}

impl Default for DataEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl DataEncoder {
    /// Create new data encoder
    pub fn new() -> Self {
        Self {
            encoding: EncodingMethod::Base64,
            encryption: EncryptionMethod::Xor,
            compression: CompressionMethod::None,
            chunk_size: 1024,
            encryption_key: Vec::new(),
        }
    }

    /// Set encoding method
    pub fn with_encoding(mut self, method: EncodingMethod) -> Self {
        self.encoding = method;
        self
    }

    /// Set encryption method and key
    pub fn with_encryption(mut self, method: EncryptionMethod, key: Vec<u8>) -> Self {
        self.encryption = method;
        self.encryption_key = key;
        self
    }

    /// Set compression method
    pub fn with_compression(mut self, method: CompressionMethod) -> Self {
        self.compression = method;
        self
    }

    /// Set chunk size
    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    /// Encode data into chunks ready for exfiltration
    pub fn encode_data(&self, data: &[u8], session_id: &str) -> Vec<DataChunk> {
        // Step 1: Compress if needed
        let compressed = self.compress(data);

        // Step 2: Encrypt if needed
        let encrypted = self.encrypt(&compressed);

        // Step 3: Split into chunks
        let chunks: Vec<Vec<u8>> = encrypted.chunks(self.chunk_size).map(|c| c.to_vec()).collect();

        let total = chunks.len() as u32;

        // Step 4: Create chunk objects
        chunks
            .into_iter()
            .enumerate()
            .map(|(i, data)| DataChunk::new(i as u32, total, data, session_id))
            .collect()
    }

    /// Decode chunks back to original data
    pub fn decode_chunks(&self, chunks: &[DataChunk]) -> Result<Vec<u8>, String> {
        // Verify all chunks are present
        let total = chunks.first().map(|c| c.total_chunks).unwrap_or(0);
        if chunks.len() != total as usize {
            return Err("Missing chunks".to_string());
        }

        // Sort by sequence
        let mut sorted = chunks.to_vec();
        sorted.sort_by_key(|c| c.sequence);

        // Verify checksums
        for chunk in &sorted {
            if !chunk.verify() {
                return Err(format!("Checksum failed for chunk {}", chunk.sequence));
            }
        }

        // Reassemble
        let encrypted: Vec<u8> = sorted.into_iter().flat_map(|c| c.data).collect();

        // Decrypt
        let decrypted = self.decrypt(&encrypted);

        // Decompress
        let decompressed = self.decompress(&decrypted)?;

        Ok(decompressed)
    }

    /// Compress data
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        match self.compression {
            CompressionMethod::None => data.to_vec(),
            CompressionMethod::Gzip => {
                // Would use flate2 crate
                data.to_vec()
            }
            CompressionMethod::Zlib => data.to_vec(),
            CompressionMethod::Lz4 => data.to_vec(),
        }
    }

    /// Decompress data
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        match self.compression {
            CompressionMethod::None => Ok(data.to_vec()),
            _ => Ok(data.to_vec()), // Simplified
        }
    }

    /// Encrypt data
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        match self.encryption {
            EncryptionMethod::None => data.to_vec(),
            EncryptionMethod::Xor => {
                if self.encryption_key.is_empty() {
                    return data.to_vec();
                }
                data.iter()
                    .enumerate()
                    .map(|(i, b)| b ^ self.encryption_key[i % self.encryption_key.len()])
                    .collect()
            }
            EncryptionMethod::Aes256Gcm => {
                // Would use aes-gcm crate
                data.to_vec()
            }
            EncryptionMethod::ChaCha20 => data.to_vec(),
        }
    }

    /// Decrypt data
    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        // XOR is symmetric
        self.encrypt(data)
    }

    /// Calculate total encoded size
    pub fn calculate_encoded_size(&self, data_size: usize) -> usize {
        let ratio = self.encoding.expansion_ratio();
        (data_size as f32 * ratio) as usize
    }

    /// Generate random session ID
    pub fn generate_session_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x12345678);

        (0..16)
            .map(|i| chars[((seed >> (i % 8)) ^ (i as u64 * 31)) as usize % chars.len()])
            .collect()
    }

    /// Get current encoding method
    pub fn encoding(&self) -> EncodingMethod {
        self.encoding
    }

    /// Get current chunk size
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }
}

// ========== Encoding Functions ==========

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

        result.push(CHARS[(b0 >> 2) & 0x3F] as char);
        result.push(CHARS[((b0 << 4) | (b1 >> 4)) & 0x3F] as char);

        if chunk.len() > 1 {
            result.push(CHARS[((b1 << 2) | (b2 >> 6)) & 0x3F] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARS[b2 & 0x3F] as char);
        } else {
            result.push('=');
        }
    }

    result
}

fn base64_url_encode(data: &[u8]) -> String {
    base64_encode(data)
        .replace('+', "-")
        .replace('/', "_")
        .trim_end_matches('=')
        .to_string()
}

fn base32_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut result = String::new();
    let mut buffer: u64 = 0;
    let mut bits = 0;

    for byte in data {
        buffer = (buffer << 8) | *byte as u64;
        bits += 8;

        while bits >= 5 {
            bits -= 5;
            result.push(CHARS[((buffer >> bits) & 0x1F) as usize] as char);
        }
    }

    if bits > 0 {
        buffer <<= 5 - bits;
        result.push(CHARS[(buffer & 0x1F) as usize] as char);
    }

    result
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

fn dictionary_encode(data: &[u8]) -> String {
    // Encode using common words to look like text
    let words = [
        "the", "and", "is", "to", "of", "in", "it", "for", "on", "with", "as", "at", "by", "an",
        "be", "or",
    ];

    data.iter()
        .map(|b| words[(*b as usize) % words.len()])
        .collect::<Vec<_>>()
        .join(" ")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_chunk_checksum() {
        let chunk = DataChunk::new(0, 1, vec![1, 2, 3, 4, 5], "test");
        assert!(chunk.verify());
    }

    #[test]
    fn test_encoder_creation() {
        let encoder = DataEncoder::new()
            .with_encoding(EncodingMethod::Base32)
            .with_chunk_size(512);

        assert_eq!(encoder.encoding, EncodingMethod::Base32);
        assert_eq!(encoder.chunk_size, 512);
    }

    #[test]
    fn test_encode_decode() {
        let encoder =
            DataEncoder::new().with_encryption(EncryptionMethod::Xor, vec![0xAA, 0xBB]);

        let data = b"Hello, World!";
        let session = DataEncoder::generate_session_id();

        let chunks = encoder.encode_data(data, &session);
        let decoded = encoder.decode_chunks(&chunks).unwrap();

        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_encode() {
        let encoded = base64_encode(b"Hello");
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_hex_encode() {
        let encoded = hex_encode(&[0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(encoded, "deadbeef");
    }

    #[test]
    fn test_base32_encode() {
        let encoded = base32_encode(b"test");
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_base64_url_encode() {
        let encoded = base64_url_encode(b"Hello World!");
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));
        assert!(!encoded.contains('='));
    }

    #[test]
    fn test_dictionary_encode() {
        let encoded = dictionary_encode(&[1, 2, 3]);
        assert!(encoded.contains(' '));
    }

    #[test]
    fn test_encoding_expansion_ratio() {
        assert!(EncodingMethod::Hex.expansion_ratio() > EncodingMethod::Base64.expansion_ratio());
    }

    #[test]
    fn test_dns_safe_encoding() {
        assert!(EncodingMethod::Base32.is_dns_safe());
        assert!(EncodingMethod::Hex.is_dns_safe());
        assert!(!EncodingMethod::Base64.is_dns_safe());
    }

    #[test]
    fn test_generate_session_id() {
        let id = DataEncoder::generate_session_id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_chunk_encoding() {
        let chunk = DataChunk::new(0, 1, vec![1, 2, 3, 4, 5], "test");
        let encoded = chunk.encode(EncodingMethod::Hex);
        assert_eq!(encoded, "0102030405");
    }
}
