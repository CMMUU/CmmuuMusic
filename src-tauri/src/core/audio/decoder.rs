use std::fs::File;
use std::path::Path;

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::conv::IntoSample;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::sample::Sample;

use super::AudioError;

/// 已完整解码的 PCM 数据（交错存储的 f32 样本）。
pub struct DecodedAudio {
    /// 交错样本: [L, R, L, R, ...]
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl DecodedAudio {
    /// 总时长（秒）
    pub fn duration_secs(&self) -> f64 {
        if self.sample_rate == 0 || self.channels == 0 {
            return 0.0;
        }
        let frames = self.samples.len() as f64 / self.channels as f64;
        frames / self.sample_rate as f64
    }
}

/// 将本地音频文件完整解码为交错 f32 PCM。
///
/// POC 阶段一次性解码全部数据到内存，换取实现简洁与无锁播放。
/// 后续阶段会替换为基于环形缓冲区的流式解码（SDD §7.2）。
pub fn decode_file(path: &Path) -> Result<DecodedAudio, AudioError> {
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| AudioError::Decode(format!("探测格式失败: {e}")))?;

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| AudioError::Decode("文件中未找到可解码的音轨".into()))?;

    let track_id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| AudioError::Decode(format!("创建解码器失败: {e}")))?;

    let mut samples: Vec<f32> = Vec::new();
    let mut sample_rate: u32 = track.codec_params.sample_rate.unwrap_or(44100);
    let mut channels: u16 = track
        .codec_params
        .channels
        .map(|c| c.count() as u16)
        .unwrap_or(2);

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            // 读到流末尾或重置，结束解码循环
            Err(symphonia::core::errors::Error::IoError(ref e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(symphonia::core::errors::Error::ResetRequired) => break,
            Err(e) => return Err(AudioError::Decode(format!("读取数据包失败: {e}"))),
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                sample_rate = decoded.spec().rate;
                channels = decoded.spec().channels.count() as u16;
                append_samples(&decoded, &mut samples);
            }
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
            Err(e) => return Err(AudioError::Decode(format!("解码数据包失败: {e}"))),
        }
    }

    Ok(DecodedAudio {
        samples,
        sample_rate,
        channels,
    })
}

/// 将 Symphonia 任意采样格式的缓冲区转换为 f32 并追加到输出。
fn append_samples(decoded: &AudioBufferRef<'_>, out: &mut Vec<f32>) {
    match decoded {
        AudioBufferRef::F32(buf) => interleave(buf, out, |s| s),
        AudioBufferRef::U8(buf) => interleave(buf, out, |s| s.into_sample()),
        AudioBufferRef::U16(buf) => interleave(buf, out, |s| s.into_sample()),
        AudioBufferRef::U24(buf) => interleave(buf, out, |s| s.into_sample()),
        AudioBufferRef::U32(buf) => interleave(buf, out, |s| s.into_sample()),
        AudioBufferRef::S8(buf) => interleave(buf, out, |s| s.into_sample()),
        AudioBufferRef::S16(buf) => interleave(buf, out, |s| s.into_sample()),
        AudioBufferRef::S24(buf) => interleave(buf, out, |s| s.into_sample()),
        AudioBufferRef::S32(buf) => interleave(buf, out, |s| s.into_sample()),
        AudioBufferRef::F64(buf) => interleave(buf, out, |s| s.into_sample()),
    }
}

/// 将平面（planar）多声道缓冲交错为 [L, R, L, R, ...]。
fn interleave<S, F>(
    buf: &symphonia::core::audio::AudioBuffer<S>,
    out: &mut Vec<f32>,
    convert: F,
) where
    S: Sample + Copy,
    F: Fn(S) -> f32,
{
    let channels = buf.spec().channels.count();
    let frames = buf.frames();
    out.reserve(frames * channels);
    for frame in 0..frames {
        for ch in 0..channels {
            out.push(convert(buf.chan(ch)[frame]));
        }
    }
}
