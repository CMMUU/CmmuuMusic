//! 路径处理工具。

/// 提取文件扩展名（小写，不含点）。
pub fn extension_lower(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
}

/// 判断是否为受支持的音频扩展名（对齐 SDD §5.1.3）。
pub fn is_supported_audio(path: &str) -> bool {
    matches!(
        extension_lower(path).as_deref(),
        Some("mp3" | "flac" | "m4a" | "aac" | "wav" | "ogg" | "opus" | "wma" | "alac")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_audio() {
        assert!(is_supported_audio("/music/song.FLAC"));
        assert!(is_supported_audio("a.mp3"));
        assert!(!is_supported_audio("a.txt"));
        assert!(!is_supported_audio("noext"));
    }
}
