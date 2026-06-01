//! 音频管线（占位）。
//!
//! 对齐 SDD §7：完整的「URL 获取 → 缓存检查 → 流式下载 → 解码 → 处理 → 输出」管线。
//! POC 阶段音频引擎直接走「本地文件 → 完整解码 → 输出」的简化路径，
//! 完整管线（含 HTTP Range 流式下载、环形缓冲、特效链）将在音频管线阶段实现。

use super::AudioError;

/// 音频特效处理单元。对齐 SDD §7.4。
pub trait AudioEffect: Send + Sync {
    fn name(&self) -> &str;
    fn process(&mut self, input: &[f32], output: &mut [f32]);
    fn reset(&mut self);
}

/// 特效链（占位）。
#[derive(Default)]
pub struct AudioEffectChain {
    effects: Vec<Box<dyn AudioEffect>>,
}

impl AudioEffectChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, effect: Box<dyn AudioEffect>) {
        self.effects.push(effect);
    }

    pub fn len(&self) -> usize {
        self.effects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
}

/// URL 流式播放管线入口（占位）。
pub async fn stream_from_url(_url: &str) -> Result<(), AudioError> {
    Err(AudioError::NotImplemented("URL 流式管线尚未实现"))
}
