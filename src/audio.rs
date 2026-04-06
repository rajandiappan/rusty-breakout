use rodio::Source;
use std::f32::consts::PI;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

/// Audio system for game sounds using synthesized sine waves via Rodio
/// Generates real PCM audio and plays it through the system audio device
#[derive(Clone, Debug)]
pub struct AudioManager {
    pub sfx_enabled: bool,
    pub volume: f32,                // 0.0 to 1.0
    master_volume: Arc<Mutex<f32>>, // Master volume control
}

impl AudioManager {
    pub fn new() -> Self {
        AudioManager {
            sfx_enabled: true,
            volume: 0.7,
            master_volume: Arc::new(Mutex::new(0.7)),
        }
    }

    /// Play a paddle hit sound (short beep: 400 Hz, 50ms)
    pub fn play_paddle_hit(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(400.0, 0.05, 0.3);
    }

    /// Play a brick destruction sound (medium beep: 600 Hz, 100ms)
    pub fn play_brick_destroy(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(600.0, 0.1, 0.4);
    }

    /// Play a power-up pickup sound (high beep: 900 Hz, 150ms)
    pub fn play_powerup_pickup(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(900.0, 0.15, 0.35);
    }

    /// Play a level complete sound (ascending melody)
    pub fn play_level_complete(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(700.0, 0.2, 0.3);
    }

    /// Play a game over sound (descending beep)
    pub fn play_game_over(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(300.0, 0.3, 0.4);
    }

    /// Play a victory sound (ascending fanfare)
    pub fn play_victory(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(800.0, 0.4, 0.3);
    }

    /// Generate and play a sine wave tone at the given frequency
    ///
    /// Parameters:
    /// - frequency: Hz (e.g., 440 = A4)
    /// - duration: seconds
    /// - base_volume: 0.0 to 1.0 (will be multiplied by master volume)
    fn play_tone(&self, frequency: f32, duration: f32, base_volume: f32) {
        let volume = self.volume * base_volume;

        // Spawn audio playback in a background thread to avoid blocking game
        std::thread::spawn(move || {
            const SAMPLE_RATE: u32 = 44100; // 44.1 kHz
            let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
            let mut samples = Vec::with_capacity(num_samples);

            // Generate sine wave samples
            for i in 0..num_samples {
                let t = i as f32 / SAMPLE_RATE as f32;
                let sample = (2.0 * PI * frequency * t).sin();

                // Apply volume and fade out at the end to prevent clicks
                let fade = if i < num_samples.saturating_sub(2000) {
                    1.0
                } else {
                    ((num_samples.saturating_sub(i)) as f32 / 2000.0).max(0.0)
                };

                let value = (sample * volume * fade * 32767.0) as i16;
                samples.push(value);
            }

            // Create WAV data in memory
            let mut wav_data = Vec::new();
            create_wav_header(&mut wav_data, SAMPLE_RATE, samples.len());

            // Add PCM sample data
            for sample in samples {
                wav_data.extend_from_slice(&sample.to_le_bytes());
            }

            // Play the audio using Rodio
            if let Ok((_stream, handle)) = rodio::OutputStream::try_default() {
                if let Ok(decoder) = rodio::Decoder::new(Cursor::new(wav_data)) {
                    let _ = handle.play_raw(decoder.convert_samples::<f32>());
                    // Keep _stream alive for the duration of playback
                    std::thread::sleep(std::time::Duration::from_millis(
                        (duration * 1000.0) as u64 + 100,
                    ));
                }
            }
        });
    }

    /// Set the master volume (0.0 to 1.0)
    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol.clamp(0.0, 1.0);
        let _ = self.master_volume.lock().map(|mut v| *v = self.volume);
    }

    /// Get the current master volume
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    /// Increase volume by 10%
    pub fn increase_volume(&mut self) {
        self.set_volume(self.volume + 0.1);
    }

    /// Decrease volume by 10%
    pub fn decrease_volume(&mut self) {
        self.set_volume(self.volume - 0.1);
    }

    pub fn set_sfx_enabled(&mut self, enabled: bool) {
        self.sfx_enabled = enabled;
    }

    pub fn toggle_sfx(&mut self) {
        self.sfx_enabled = !self.sfx_enabled;
    }
}

/// Create a minimal WAV file header for PCM audio
fn create_wav_header(buf: &mut Vec<u8>, sample_rate: u32, num_samples: usize) {
    let num_channels = 1u16;
    let bits_per_sample = 16u16;
    let byte_rate = sample_rate * u32::from(num_channels) * u32::from(bits_per_sample) / 8;
    let block_align = (num_channels * bits_per_sample) / 8;
    let subchunk2_size = (num_samples as u32) * u32::from(block_align);
    let chunk_size = 36 + subchunk2_size;

    // RIFF header
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&chunk_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");

    // fmt sub-chunk
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1 size
    buf.extend_from_slice(&1u16.to_le_bytes()); // Audio format (PCM)
    buf.extend_from_slice(&num_channels.to_le_bytes());
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    buf.extend_from_slice(&byte_rate.to_le_bytes());
    buf.extend_from_slice(&block_align.to_le_bytes());
    buf.extend_from_slice(&bits_per_sample.to_le_bytes());

    // data sub-chunk
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&subchunk2_size.to_le_bytes());
}
