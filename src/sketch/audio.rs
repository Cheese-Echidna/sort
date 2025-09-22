use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, StreamConfig};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug)]
pub struct AudioModel {
    pub phase: f64,
    pub hz: f64,
    pub volume: f64,
}

impl Default for AudioModel {
    fn default() -> Self {
        let volume = if cfg!(target_family = "wasm") {
            0.0
        } else {
            0.2
        };

        Self { phase: 0.0, hz: 440.0, volume }
    }
}

pub struct AudioHandle {
    pub model: Arc<Mutex<AudioModel>>, // shared state for callback updates
    pub stream: cpal::Stream,
}

impl AudioHandle {
    pub fn play(&self) -> Result<()> { self.stream.play()?; Ok(()) }
    pub fn pause(&self) -> Result<()> { self.stream.pause()?; Ok(()) }
    pub fn send<F: FnOnce(&mut AudioModel)>(&self, f: F) -> Result<()> {
        if let Ok(mut m) = self.model.lock() { f(&mut *m); }
        Ok(())
    }
}

pub fn start_audio(initial: AudioModel) -> Result<AudioHandle> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("No output device"))?;
    let mut config: StreamConfig = device.default_output_config()?.into();

    // Prefer a reasonable sample rate if available (keep default otherwise)
    if config.sample_rate.0 < 22050 { config.sample_rate.0 = 44100; }

    let sample_format = device.default_output_config()?.sample_format();

    let shared = Arc::new(Mutex::new(initial));
    let shared_cb = shared.clone();

    let stream = match sample_format {
        SampleFormat::F32 => build_stream_f32(&device, &config, shared_cb)?,
        SampleFormat::I16 => build_stream_i16(&device, &config, shared_cb)?,
        SampleFormat::U16 => build_stream_u16(&device, &config, shared_cb)?,
        _ => build_stream_f32(&device, &config, shared_cb)?,
    };

    Ok(AudioHandle { model: shared, stream })
}

fn build_stream_f32(device: &cpal::Device, config: &StreamConfig, shared: Arc<Mutex<AudioModel>>) -> Result<cpal::Stream> {
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate.0 as f64;
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [f32], _| {
            let mut guard = shared.lock().ok();
            let (hz, volume, phase_start) = if let Some(ref m) = guard { (m.hz, m.volume, m.phase) } else { (440.0, 0.0, 0.0) };
            let mut phase = phase_start;
            for frame in data.chunks_mut(channels) {
                let t = phase % 1.0;
                let triangle = true;
                let sample = if triangle { 4.0 * (t - 0.5).abs() - 1.0 } else { (2.0 * std::f64::consts::PI * phase).sin() } as f32;
                phase += hz / sample_rate;
                if phase >= 1.0 { phase -= 1.0; }
                let s = (sample * volume as f32).clamp(-1.0, 1.0);
                for ch in frame.iter_mut() { *ch = s; }
            }
            if let Some(ref mut m) = guard { m.phase = phase; }
        },
        err_fn,
    )?;
    Ok(stream)
}

fn build_stream_i16(device: &cpal::Device, config: &StreamConfig, shared: Arc<Mutex<AudioModel>>) -> Result<cpal::Stream> {
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate.0 as f64;
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [i16], _| {
            let mut guard = shared.lock().ok();
            let (hz, volume, phase_start) = if let Some(ref m) = guard { (m.hz, m.volume, m.phase) } else { (440.0, 0.0, 0.0) };
            let mut phase = phase_start;
            for frame in data.chunks_mut(channels) {
                let t = phase % 1.0;
                let triangle = true;
                let sample = if triangle { 4.0 * (t - 0.5).abs() - 1.0 } else { (2.0 * std::f64::consts::PI * phase).sin() } as f32;
                phase += hz / sample_rate;
                if phase >= 1.0 { phase -= 1.0; }
                let s = (sample * volume as f32).clamp(-1.0, 1.0);
                let v = (s * i16::MAX as f32) as i16;
                for ch in frame.iter_mut() { *ch = v; }
            }
            if let Some(ref mut m) = guard { m.phase = phase; }
        },
        err_fn,
    )?;
    Ok(stream)
}

fn build_stream_u16(device: &cpal::Device, config: &StreamConfig, shared: Arc<Mutex<AudioModel>>) -> Result<cpal::Stream> {
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate.0 as f64;
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [u16], _| {
            let mut guard = shared.lock().ok();
            let (hz, volume, phase_start) = if let Some(ref m) = guard { (m.hz, m.volume, m.phase) } else { (440.0, 0.0, 0.0) };
            let mut phase = phase_start;
            for frame in data.chunks_mut(channels) {
                let t = phase % 1.0;
                let triangle = true;
                let sample = if triangle { 4.0 * (t - 0.5).abs() - 1.0 } else { (2.0 * std::f64::consts::PI * phase).sin() } as f32;
                phase += hz / sample_rate;
                if phase >= 1.0 { phase -= 1.0; }
                let s = (sample * volume as f32).clamp(-1.0, 1.0);
                // Map [-1.0,1.0] -> [0,u16::MAX]
                let v = ((s * 0.5 + 0.5) * u16::MAX as f32) as u16;
                for ch in frame.iter_mut() { *ch = v; }
            }
            if let Some(ref mut m) = guard { m.phase = phase; }
        },
        err_fn,
    )?;
    Ok(stream)
}
