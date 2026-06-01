//! 加密工具。对齐 SDD §6.1 utils.crypto。
//!
//! POC 阶段提供 MD5 与 Base64；AES/RSA 将在插件系统阶段补全。

use base64::Engine as _;
use md5::{Digest, Md5};

/// MD5 十六进制摘要。
pub fn md5_hex(input: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

/// Base64 标准编码。
pub fn base64_encode(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

/// Base64 标准解码。
pub fn base64_decode(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::engine::general_purpose::STANDARD.decode(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_roundtrip() {
        let data = b"\xde\xad\xbe\xef";
        let encoded = base64_encode(data);
        assert_eq!(base64_decode(&encoded).unwrap(), data);
    }

    #[test]
    fn md5_known() {
        assert_eq!(md5_hex(b"abc"), "900150983cd24fb0d6963f7d28e17f72");
    }
}
