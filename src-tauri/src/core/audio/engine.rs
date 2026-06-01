use std::path::Path;
use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use parking_lot::Mutex;

use super::decoder::{decode_bytes, decode_file, DecodedAudio};
use super::{AudioEngine, AudioError, PlaybackState, PlaybackStatus};

/// 当前正在播放的源数据与游标（由音频回调读写）。
struct Playback {
    /// 交错样本 [L, R, L, R, ...]
    samples: Arc<Vec<f32>>,
    src_channels: usize,
    src_rate: u32,
    /// 源帧的小数游标（用于线性插值重采样）
    cursor: f64,
}

impl Playback {
    fn empty() -> Self {
        Self {
            samples: Arc::new(Vec::new()),
            src_channels: 2,
            src_rate: 44100,
            cursor: 0.0,
        }
    }

    fn total_frames(&self) -> usize {
        if self.src_channels == 0 {
            0
        } else {
            self.samples.len() / self.src_channels
        }
    }
}

/// 引擎与音频回调之间共享的状态。
struct Shared {
    playback: Mutex<Playback>,
    state: AtomicU8,
    /// 音量 f32 的位表示
    volume: AtomicU32,
}

impl Shared {
    fn state(&self) -> PlaybackState {
        PlaybackState::from_u8(self.state.load(Ordering::Acquire))
    }

    fn set_state(&self, s: PlaybackState) {
        self.state.store(s.as_u8(), Ordering::Release);
    }

    fn volume(&self) -> f32 {
        f32::from_bits(self.volume.load(Ordering::Relaxed))
    }
}

/// 基于 cpal 的音频引擎。
///
/// 由于 cpal 的 `Stream` 是 `!Send`，无法直接存放在需要跨线程共享的引擎结构中，
/// 因此用一条专用线程持有并保活输出流；引擎与音频回调通过 `Arc<Shared>` 通信。
/// 输出流在设备默认采样率下持续运行，切歌时只替换源缓冲并重置游标，
/// 回调内做线性插值重采样以适配设备采样率（POC 实现，后续阶段替换为 rubato）。
pub struct CpalAudioEngine {
    shared: Arc<Shared>,
    // 持有线程句柄以保活音频线程（Drop 时不显式 join，随进程退出）
    _audio_thread: thread::JoinHandle<()>,
}

impl CpalAudioEngine {
    pub fn new() -> Result<Self, AudioError> {
        let shared = Arc::new(Shared {
            playback: Mutex::new(Playback::empty()),
            state: AtomicU8::new(PlaybackState::Idle.as_u8()),
            volume: AtomicU32::new(1.0f32.to_bits()),
        });

        let (init_tx, init_rx) = mpsc::channel::<Result<u32, AudioError>>();
        let thread_shared = shared.clone();

        let handle = thread::Builder::new()
            .name("cmmuu-audio".into())
            .spawn(move || audio_thread_main(thread_shared, init_tx))
            .map_err(|e| AudioError::Stream(format!("无法创建音频线程: {e}")))?;

        // 接收设备采样率作为初始化成功的握手信号（具体重采样在回调内按需进行）
        let _out_rate = init_rx
            .recv()
            .map_err(|_| AudioError::Stream("音频线程初始化中断".into()))??;

        Ok(Self {
            shared,
            _audio_thread: handle,
        })
    }

    fn load(&self, audio: DecodedAudio) {
        let mut pb = self.shared.playback.lock();
        pb.src_channels = audio.channels.max(1) as usize;
        pb.src_rate = audio.sample_rate.max(1);
        pb.samples = Arc::new(audio.samples);
        pb.cursor = 0.0;
        drop(pb);
        self.shared.set_state(PlaybackState::Playing);
    }
}

impl AudioEngine for CpalAudioEngine {
    fn play_file(&self, path: &Path) -> Result<(), AudioError> {
        self.shared.set_state(PlaybackState::Loading);
        let audio = decode_file(path).inspect_err(|_| {
            self.shared.set_state(PlaybackState::Error);
        })?;
        self.load(audio);
        Ok(())
    }

    fn play_bytes(&self, bytes: Vec<u8>, hint_ext: Option<&str>) -> Result<(), AudioError> {
        self.shared.set_state(PlaybackState::Loading);
        let audio = decode_bytes(bytes, hint_ext).inspect_err(|_| {
            self.shared.set_state(PlaybackState::Error);
        })?;
        self.load(audio);
        Ok(())
    }

    fn toggle_pause(&self) {
        match self.shared.state() {
            PlaybackState::Playing => self.shared.set_state(PlaybackState::Paused),
            PlaybackState::Paused => self.shared.set_state(PlaybackState::Playing),
            _ => {}
        }
    }

    fn stop(&self) {
        let mut pb = self.shared.playback.lock();
        pb.samples = Arc::new(Vec::new());
        pb.cursor = 0.0;
        drop(pb);
        self.shared.set_state(PlaybackState::Idle);
    }

    fn seek(&self, position_secs: f64) -> Result<(), AudioError> {
        let mut pb = self.shared.playback.lock();
        let target = (position_secs.max(0.0)) * pb.src_rate as f64;
        let max = pb.total_frames().saturating_sub(1) as f64;
        pb.cursor = target.min(max.max(0.0));
        Ok(())
    }

    fn set_volume(&self, volume: f32) {
        let v = volume.clamp(0.0, 1.0);
        self.shared.volume.store(v.to_bits(), Ordering::Relaxed);
    }

    fn status(&self) -> PlaybackStatus {
        let pb = self.shared.playback.lock();
        let total_frames = pb.total_frames();
        let position = if pb.src_rate > 0 {
            pb.cursor / pb.src_rate as f64
        } else {
            0.0
        };
        let duration = if total_frames > 0 && pb.src_rate > 0 {
            Some(total_frames as f64 / pb.src_rate as f64)
        } else {
            None
        };
        drop(pb);

        PlaybackStatus {
            state: self.shared.state(),
            position,
            duration,
            volume: self.shared.volume(),
        }
    }
}

/// 音频线程主函数：构建并保活输出流。
fn audio_thread_main(shared: Arc<Shared>, init_tx: mpsc::Sender<Result<u32, AudioError>>) {
    let host = cpal::default_host();
    let device = match host.default_output_device() {
        Some(d) => d,
        None => {
            let _ = init_tx.send(Err(AudioError::NoOutputDevice));
            return;
        }
    };

    let supported = match device.default_output_config() {
        Ok(c) => c,
        Err(e) => {
            let _ = init_tx.send(Err(AudioError::UnsupportedConfig(e.to_string())));
            return;
        }
    };

    let sample_format = supported.sample_format();
    let config: cpal::StreamConfig = supported.into();
    let out_rate = config.sample_rate.0;
    let out_channels = config.channels as usize;

    let stream_result = build_stream(
        &device,
        &config,
        sample_format,
        shared.clone(),
        out_channels,
        out_rate,
    );

    let stream = match stream_result {
        Ok(s) => s,
        Err(e) => {
            let _ = init_tx.send(Err(e));
            return;
        }
    };

    if let Err(e) = stream.play() {
        let _ = init_tx.send(Err(AudioError::Stream(e.to_string())));
        return;
    }

    // 初始化成功，回报设备采样率
    let _ = init_tx.send(Ok(out_rate));

    // 保活：持有 stream 不被 drop，线程长期 park
    loop {
        thread::park_timeout(Duration::from_secs(3600));
    }
}

/// 按设备采样格式构建输出流。
fn build_stream(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    format: cpal::SampleFormat,
    shared: Arc<Shared>,
    out_channels: usize,
    out_rate: u32,
) -> Result<cpal::Stream, AudioError> {
    let err_fn = |e| log::error!("音频输出流错误: {e}");

    macro_rules! make {
        ($t:ty) => {{
            let sh = shared.clone();
            device
                .build_output_stream(
                    config,
                    move |data: &mut [$t], _| fill::<$t>(data, &sh, out_channels, out_rate),
                    err_fn,
                    None,
                )
                .map_err(|e| AudioError::Stream(e.to_string()))
        }};
    }

    match format {
        cpal::SampleFormat::F32 => make!(f32),
        cpal::SampleFormat::I16 => make!(i16),
        cpal::SampleFormat::U16 => make!(u16),
        other => Err(AudioError::UnsupportedConfig(format!(
            "设备采样格式 {other:?} 暂不支持"
        ))),
    }
}

/// 音频回调：填充输出缓冲。
fn fill<T>(data: &mut [T], shared: &Shared, out_channels: usize, out_rate: u32)
where
    T: cpal::Sample + cpal::FromSample<f32>,
{
    let silence = T::from_sample(0.0f32);
    let playing = shared.state.load(Ordering::Acquire) == PlaybackState::Playing.as_u8();
    let volume = shared.volume();

    let mut pb = shared.playback.lock();
    let total_frames = pb.total_frames();
    let step = if out_rate > 0 {
        pb.src_rate as f64 / out_rate as f64
    } else {
        1.0
    };

    for frame in data.chunks_mut(out_channels.max(1)) {
        if !playing || total_frames == 0 {
            for s in frame.iter_mut() {
                *s = silence;
            }
            continue;
        }

        let idx = pb.cursor.floor() as usize;
        if idx >= total_frames {
            shared.set_state(PlaybackState::Ended);
            for s in frame.iter_mut() {
                *s = silence;
            }
            continue;
        }

        let frac = (pb.cursor - idx as f64) as f32;
        let next = (idx + 1).min(total_frames - 1);
        let ch = pb.src_channels;

        let left = lerp(
            pb.samples[idx * ch],
            pb.samples[next * ch],
            frac,
        );
        let right = if ch >= 2 {
            lerp(
                pb.samples[idx * ch + 1],
                pb.samples[next * ch + 1],
                frac,
            )
        } else {
            left
        };

        for (i, s) in frame.iter_mut().enumerate() {
            let v = match (out_channels, i) {
                (1, _) => (left + right) * 0.5,
                (_, 0) => left,
                (_, 1) => right,
                _ => 0.0,
            };
            *s = T::from_sample(v * volume);
        }

        pb.cursor += step;
    }
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
