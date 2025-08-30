#[cfg(not(target_arch = "wasm32"))]
pub use audio_backend::Stream;

#[cfg(target_arch = "wasm32")]
pub use cpal::Stream;

#[derive(Clone, Copy)]
pub struct Audio {
    pub(crate) phase: f64,
    pub(crate) hz: f64,
    pub volume: f64,
}

// -------------------- Desktop (native) --------------------
#[cfg(not(target_arch = "wasm32"))]
mod audio_backend {
    use nannou_audio as audio;
    use nannou_audio::Buffer;
    use super::Audio;

    pub type Stream = audio::Stream<Audio>;
    pub type Host = audio::Host;

    pub fn start(audio_model: Audio) -> anyhow::Result<Stream> {
        let host = Host::new();
        let stream = host
            .new_output_stream(audio_model)
            .render(audio)    // your callback
            .build()?;               // handle Result instead of unwrap
        stream.play()?;              // handle Result
        Ok(stream)
    }
    
    fn audio(audio: &mut Audio, buffer: &mut Buffer) {
        let sample_rate = buffer.sample_rate() as f64;
        let volume = audio.volume;

        for frame in buffer.frames_mut() {
            let t = audio.phase % 1.0;

            let triangle = true;

            let sample = if triangle {
                // Triangle wave
                4.0 * (t - 0.5).abs() - 1.0
            } else {
                // Sine wave
                (2.0 * std::f64::consts::PI * audio.phase).sin()
            };

            audio.phase += audio.hz / sample_rate;
            if audio.phase >= 1.0 {
                audio.phase -= 1.0;
            }

            for channel in frame {
                *channel = (sample * volume) as f32;
            }
        }
    }
}

// -------------------- Web (wasm32) --------------------
#[cfg(target_arch = "wasm32")]
mod audio_backend {
    use std::cell::RefCell;
    use std::rc::Rc;

    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{Stream};
    use nannou_audio::dasp_sample::FromSample;
    use wasm_bindgen::prelude::*;

    use super::Audio;

    pub fn start(mut audio_model: Audio) -> Result<Stream, JsValue> {
        // CPAL WebAudio host/device/config
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| JsValue::from_str("No default output device"))?;
        let config = device
            .default_output_config()
            .map_err(|e| JsValue::from_str(&format!("No default output config: {e}")))?;

        // State must be 'static for the audio callback
        let state = Rc::new(RefCell::new(audio_model));

        // Select the correct sample format
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => run(&device, &config.into(), state),
            other => return Err(JsValue::from_str(&format!("Unsupported format: {other:?}"))),
        }?;

        // Important in browsers: call play() from a user gesture (e.g. button onclick)
        // If you call start() from such a handler, play() will succeed.
        stream.play().map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(stream)
    }

    fn run(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        state: Rc<RefCell<Audio>>,
    ) -> Result<Stream, JsValue>
    {
        let channels = config.channels as usize;
        let sample_rate = config.sample_rate.0 as f64;

        let mut next_frame = move |out: &mut [f32]| {
            let mut audio = state.borrow_mut();

            // Reuse your oscillator math, adapted to interleaved frames
            for frame in out.chunks_mut(channels) {
                let t = audio.phase % 1.0;
                let sample = {
                    // triangle
                    4.0 * (t - 0.5).abs() - 1.0
                    // or use sine:
                    // (2.0 * std::f64::consts::PI * audio.phase).sin()
                } * audio.volume as f64;

                audio.phase += audio.hz / sample_rate;
                if audio.phase >= 1.0 {
                    audio.phase -= 1.0;
                }

                for s in frame.iter_mut() {
                    *s = sample as f32;
                }
            }
        };

        let err_fn = |err| {
            web_sys::console::error_1(&format!("Stream error: {err}").into());
        };

        let stream = device
            .build_output_stream(
                config,
                move |data: &mut [f32], _| next_frame(data),
                err_fn,
            )
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(stream)
    }
}

pub fn start_audio(audio: Audio) -> Option<Stream> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        return self.stream = audio_backend::start(audio).ok();
    }
    #[cfg(target_arch = "wasm32")]
    {
        return audio_backend::start(audio).ok();
    }
}