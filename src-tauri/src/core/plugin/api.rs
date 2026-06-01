//! 宿主 API（部分实现）。对齐 SDD §6.1。
//!
//! 注入沙箱的 `cmmuumusic` 对象的宿主侧实现。POC 阶段先实现安全相关的纯逻辑
//! （URL 白名单校验、MD5），HTTP 请求与 AES/RSA 加密将在插件系统阶段补全。

use md5::{Digest, Md5};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RequestError {
    #[error("URL 解析失败: {0}")]
    InvalidUrl(String),

    #[error("URL 指向受限地址（内网/本地）")]
    BlockedUrl,

    #[error("仅允许 HTTPS 请求")]
    InsecureHttp,
}

/// MD5 摘要（十六进制）。供插件 `cmmuumusic.utils.crypto.md5` 使用。
pub fn crypto_md5(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// URL 安全校验：防 SSRF / 内网访问，仅允许 HTTPS。对齐 SDD §6.1 validate_url。
pub fn validate_url(url: &str) -> Result<(), RequestError> {
    // 仅允许 HTTPS
    let lower = url.trim().to_ascii_lowercase();
    if !lower.starts_with("https://") {
        return Err(RequestError::InsecureHttp);
    }

    // 提取 host（scheme 后到首个 '/'、'?'、'#' 或 ':' 之前）
    let after_scheme = &url["https://".len()..];
    let host_end = after_scheme
        .find(|c| c == '/' || c == '?' || c == '#')
        .unwrap_or(after_scheme.len());
    let authority = &after_scheme[..host_end];
    // 去掉可能的 userinfo 与端口
    let host = authority
        .rsplit('@')
        .next()
        .unwrap_or(authority)
        .split(':')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    if host.is_empty() {
        return Err(RequestError::InvalidUrl(url.to_string()));
    }

    // 禁止内网、本地地址
    const BLOCKED_PREFIXES: &[&str] = &[
        "127.", "0.0.0.0", "10.", "192.168.", "169.254.",
    ];
    if host == "localhost"
        || host.ends_with(".localhost")
        || BLOCKED_PREFIXES.iter().any(|p| host.starts_with(p))
        || is_private_172(&host)
    {
        return Err(RequestError::BlockedUrl);
    }

    Ok(())
}

/// 判断是否落在 172.16.0.0/12 私有段。
fn is_private_172(host: &str) -> bool {
    let mut parts = host.split('.');
    if parts.next() != Some("172") {
        return false;
    }
    match parts.next().and_then(|s| s.parse::<u32>().ok()) {
        Some(second) => (16..=31).contains(&second),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md5_known_vector() {
        assert_eq!(crypto_md5("abc"), "900150983cd24fb0d6963f7d28e17f72");
    }

    #[test]
    fn rejects_non_https() {
        assert_eq!(validate_url("http://example.com"), Err(RequestError::InsecureHttp));
    }

    #[test]
    fn rejects_internal_hosts() {
        for u in [
            "https://127.0.0.1/a",
            "https://localhost/a",
            "https://10.0.0.5/a",
            "https://192.168.1.1/a",
            "https://172.16.0.1/a",
            "https://172.31.255.255/a",
            "https://169.254.1.1/a",
        ] {
            assert_eq!(validate_url(u), Err(RequestError::BlockedUrl), "应拦截: {u}");
        }
    }

    #[test]
    fn allows_public_https() {
        assert!(validate_url("https://music.example.com/api?x=1").is_ok());
        // 172.32 不在私有段内
        assert!(validate_url("https://172.32.0.1/a").is_ok());
    }
}
