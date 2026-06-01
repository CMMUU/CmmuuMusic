//! 可视化数据提取（占位）。
//!
//! 对齐 SDD §7.3：实时频谱分析器，从 PCM 数据计算 FFT 频谱、波形、峰值与 RMS，
//! 推送到前端用于可视化。POC 阶段先提供数据结构与 RMS/峰值计算，
//! FFT 频谱将在引入 FFT 依赖后补全。

use serde::Serialize;

/// 频谱数据（推送到前端的格式）。对齐 SDD §7.3。
#[derive(Debug, Clone, Default, Serialize)]
pub struct SpectrumData {
    /// 频率带幅度
    pub bands: Vec<f32>,
    /// 波形数据（降采样）
    pub waveform: Vec<f32>,
    /// 峰值
    pub peak: f32,
    /// 均方根
    pub rms: f32,
}

/// 从交错 PCM 计算峰值与 RMS（轻量，可在回调外周期性调用）。
pub fn analyze_level(samples: &[f32]) -> SpectrumData {
    if samples.is_empty() {
        return SpectrumData::default();
    }

    let mut peak = 0.0f32;
    let mut sum_sq = 0.0f64;
    for &s in samples {
        let a = s.abs();
        if a > peak {
            peak = a;
        }
        sum_sq += (s as f64) * (s as f64);
    }
    let rms = (sum_sq / samples.len() as f64).sqrt() as f32;

    SpectrumData {
        bands: Vec::new(),
        waveform: Vec::new(),
        peak,
        rms,
    }
}
