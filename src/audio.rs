use rodio::{OutputStream, Source};
use std::f32::consts::PI;
use std::fmt::Debug;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MusicTier {
    Low,
    Medium,
    High,
}

impl From<usize> for MusicTier {
    fn from(level: usize) -> Self {
        AudioManager::tier_for_level(level)
    }
}

#[derive(Clone, Copy, Debug)]
struct MusicProfile {
    tier: MusicTier,
    danger: bool,
    master_volume: f32,
    music_volume: f32,
}

/// Audio system for synthesized SFX and layered procedural music via Rodio.
pub struct AudioManager {
    pub sfx_enabled: bool,
    pub music_enabled: bool,
    pub volume: f32, // 0.0 to 1.0
    music_volume: f32,
    music_profile: Arc<Mutex<MusicProfile>>,
    music_handle: Option<thread::JoinHandle<()>>,
    music_running: Arc<Mutex<bool>>,
}

impl Debug for AudioManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let profile = self
            .music_profile
            .lock()
            .map(|profile| *profile)
            .unwrap_or(MusicProfile {
                tier: MusicTier::Low,
                danger: false,
                master_volume: self.volume,
                music_volume: self.music_volume,
            });

        f.debug_struct("AudioManager")
            .field("sfx_enabled", &self.sfx_enabled)
            .field("music_enabled", &self.music_enabled)
            .field("volume", &self.volume)
            .field("music_volume", &self.music_volume)
            .field("music_tier", &profile.tier)
            .field("danger", &profile.danger)
            .finish()
    }
}

impl AudioManager {
    pub fn new() -> Self {
        let profile = MusicProfile {
            tier: MusicTier::Low,
            danger: false,
            master_volume: 0.7,
            music_volume: 0.3,
        };

        AudioManager {
            sfx_enabled: true,
            music_enabled: true,
            volume: 0.7,
            music_volume: 0.3,
            music_profile: Arc::new(Mutex::new(profile)),
            music_handle: None,
            music_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn tier_for_level(level: usize) -> MusicTier {
        match level {
            1..=3 => MusicTier::Low,
            4..=7 => MusicTier::Medium,
            _ => MusicTier::High,
        }
    }

    pub fn set_music_state(&mut self, tier: MusicTier, danger: bool) {
        if let Ok(mut profile) = self.music_profile.lock() {
            profile.tier = tier;
            profile.danger = danger;
            profile.master_volume = self.volume;
            profile.music_volume = self.music_volume;
        }
    }

    pub fn start_music(&mut self) {
        if !self.music_enabled {
            return;
        }

        self.stop_music();

        let running = Arc::clone(&self.music_running);
        let profile = Arc::clone(&self.music_profile);
        *running.lock().unwrap() = true;

        let handle = thread::spawn(move || {
            let mut step = 0usize;
            while *running.lock().unwrap() {
                let profile_snapshot =
                    profile
                        .lock()
                        .map(|profile| *profile)
                        .unwrap_or(MusicProfile {
                            tier: MusicTier::Low,
                            danger: false,
                            master_volume: 0.0,
                            music_volume: 0.0,
                        });

                let note = build_music_step(profile_snapshot, step);
                let playback_ms = note.0;
                let wav_data = note.1;

                if let Ok((_stream, handle)) = OutputStream::try_default() {
                    if let Ok(decoder) = rodio::Decoder::new(Cursor::new(wav_data)) {
                        let _ = handle.play_raw(decoder.convert_samples::<f32>());
                        thread::sleep(Duration::from_millis(playback_ms as u64));
                    }
                } else {
                    thread::sleep(Duration::from_millis(playback_ms as u64));
                }

                step = (step + 1) % 16;
            }
        });

        self.music_handle = Some(handle);
    }

    pub fn stop_music(&mut self) {
        *self.music_running.lock().unwrap() = false;
        if let Some(handle) = self.music_handle.take() {
            let _ = handle.join();
        }
    }

    pub fn toggle_music(&mut self) {
        self.music_enabled = !self.music_enabled;
        if self.music_enabled {
            self.start_music();
        } else {
            self.stop_music();
        }
    }

    pub fn set_music_volume(&mut self, vol: f32) {
        self.music_volume = vol.clamp(0.0, 1.0);
        if let Ok(mut profile) = self.music_profile.lock() {
            profile.music_volume = self.music_volume;
        }
    }

    #[allow(dead_code)]
    pub fn is_music_enabled(&self) -> bool {
        self.music_enabled
    }

    pub fn play_paddle_hit(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(400.0, 0.05, 0.3);
    }

    pub fn play_brick_destroy(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(600.0, 0.1, 0.4);
    }

    pub fn play_frozen_shatter(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone_sequence(&[
            (980.0, 0.05, 0.18),
            (760.0, 0.08, 0.2),
            (1180.0, 0.04, 0.12),
        ]);
    }

    pub fn play_exploding_burst(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone_sequence(&[(160.0, 0.06, 0.32), (110.0, 0.08, 0.38), (240.0, 0.05, 0.2)]);
    }

    pub fn play_steel_hit(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone_sequence(&[(540.0, 0.035, 0.2), (720.0, 0.045, 0.18)]);
    }

    pub fn play_regenerating_break(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone_sequence(&[(460.0, 0.05, 0.18), (620.0, 0.08, 0.2)]);
    }

    pub fn play_regenerating_respawn(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone_sequence(&[
            (520.0, 0.05, 0.14),
            (700.0, 0.05, 0.16),
            (920.0, 0.08, 0.18),
        ]);
    }

    pub fn play_powerup_pickup(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(900.0, 0.15, 0.35);
    }

    pub fn play_paddle_shrink(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(400.0, 0.2, 0.4);
    }

    pub fn play_level_complete(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(700.0, 0.2, 0.3);
    }

    pub fn play_game_over(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(300.0, 0.3, 0.4);
    }

    pub fn play_victory(&self) {
        if !self.sfx_enabled {
            return;
        }
        self.play_tone(800.0, 0.4, 0.3);
    }

    fn play_tone_sequence(&self, notes: &[(f32, f32, f32)]) {
        let volume = self.volume;
        let notes = notes.to_vec();

        std::thread::spawn(move || {
            for (frequency, duration, base_volume) in notes {
                AudioManager::play_tone_with_volume(volume, frequency, duration, base_volume);
                thread::sleep(Duration::from_millis((duration * 1000.0 * 0.55) as u64));
            }
        });
    }

    fn play_tone(&self, frequency: f32, duration: f32, base_volume: f32) {
        let volume = self.volume;

        std::thread::spawn(move || {
            AudioManager::play_tone_with_volume(volume, frequency, duration, base_volume);
        });
    }

    fn play_tone_with_volume(volume: f32, frequency: f32, duration: f32, base_volume: f32) {
        const SAMPLE_RATE: u32 = 44100;
        let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / SAMPLE_RATE as f32;
            let sample = (2.0 * PI * frequency * t).sin();
            let fade = if i < num_samples.saturating_sub(2000) {
                1.0
            } else {
                ((num_samples.saturating_sub(i)) as f32 / 2000.0).max(0.0)
            };

            let value = (sample * volume * base_volume * fade * 32767.0) as i16;
            samples.push(value);
        }

        let mut wav_data = Vec::new();
        create_wav_header(&mut wav_data, SAMPLE_RATE, samples.len());
        for sample in samples {
            wav_data.extend_from_slice(&sample.to_le_bytes());
        }

        if let Ok((_stream, handle)) = rodio::OutputStream::try_default() {
            if let Ok(decoder) = rodio::Decoder::new(Cursor::new(wav_data)) {
                let _ = handle.play_raw(decoder.convert_samples::<f32>());
                std::thread::sleep(std::time::Duration::from_millis(
                    (duration * 1000.0) as u64 + 100,
                ));
            }
        }
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol.clamp(0.0, 1.0);
        if let Ok(mut profile) = self.music_profile.lock() {
            profile.master_volume = self.volume;
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn increase_volume(&mut self) {
        self.set_volume(self.volume + 0.1);
    }

    pub fn decrease_volume(&mut self) {
        self.set_volume(self.volume - 0.1);
    }

    #[allow(dead_code)]
    pub fn set_sfx_enabled(&mut self, enabled: bool) {
        self.sfx_enabled = enabled;
    }

    #[allow(dead_code)]
    pub fn toggle_sfx(&mut self) {
        self.sfx_enabled = !self.sfx_enabled;
    }

    #[allow(dead_code)]
    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        self.stop_music();
    }
}

fn build_music_step(profile: MusicProfile, step: usize) -> (usize, Vec<u8>) {
    const SAMPLE_RATE: u32 = 44100;
    let base_step_ms = match profile.tier {
        MusicTier::Low => 220.0,
        MusicTier::Medium => 180.0,
        MusicTier::High => 150.0,
    };
    let duration_ms = if profile.danger {
        base_step_ms * 0.92
    } else {
        base_step_ms
    };
    let duration = duration_ms / 1000.0;
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let progression = [196.0, 220.0, 246.94, 293.66];
    let root = progression[(step / 4) % progression.len()];
    let lead_pattern = match profile.tier {
        MusicTier::Low => [0.0, 7.0, 12.0, 7.0, 5.0, 7.0, 12.0, 14.0],
        MusicTier::Medium => [0.0, 7.0, 10.0, 12.0, 7.0, 14.0, 12.0, 10.0],
        MusicTier::High => [12.0, 10.0, 7.0, 14.0, 15.0, 12.0, 19.0, 17.0],
    };
    let lead_note = root * semitone_ratio(lead_pattern[step % lead_pattern.len()]);
    let chord = [root, root * semitone_ratio(4.0), root * semitone_ratio(7.0)];
    let bass_note = root / 2.0;

    let music_gain = (profile.master_volume * profile.music_volume).clamp(0.0, 1.0);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let envelope = stepped_envelope(i, num_samples);
        let mut sample = 0.0;

        sample += sine_osc(bass_note, t) * 0.30;
        sample += soft_square_osc(bass_note * 2.0, t) * 0.08;

        for (idx, freq) in chord.iter().enumerate() {
            sample += sine_osc(*freq, t) * (0.11 - idx as f32 * 0.015);
        }

        sample += soft_square_osc(lead_note, t) * 0.18;
        sample += sine_osc(lead_note * 2.0, t) * 0.05;

        if matches!(profile.tier, MusicTier::Medium | MusicTier::High) {
            let pulse = if (step + (i / 700).min(3)).is_multiple_of(2) {
                1.0
            } else {
                -1.0
            };
            sample += pulse * percussive_decay(i, num_samples, 0.05) * 0.05;
        }

        if matches!(profile.tier, MusicTier::High) || profile.danger {
            let hat = ((((i * 97 + step * 31) % 1024) as f32) / 512.0 - 1.0)
                * percussive_decay(i, num_samples, 0.12);
            sample += hat * if profile.danger { 0.12 } else { 0.07 };
        }

        if profile.danger {
            sample += sine_osc(lead_note * 1.5, t) * 0.06;
            sample += soft_square_osc(root * 4.0, t) * 0.04;
        }

        let value = (sample * envelope * music_gain * 32767.0).clamp(-32767.0, 32767.0) as i16;
        samples.push(value);
    }

    let mut wav_data = Vec::new();
    create_wav_header(&mut wav_data, SAMPLE_RATE, samples.len());
    for sample in &samples {
        wav_data.extend_from_slice(&sample.to_le_bytes());
    }

    (duration_ms as usize, wav_data)
}

fn stepped_envelope(index: usize, total: usize) -> f32 {
    let attack = (total as f32 * 0.08).max(1.0) as usize;
    let release = (total as f32 * 0.18).max(1.0) as usize;
    if index < attack {
        index as f32 / attack as f32
    } else if index > total.saturating_sub(release) {
        total.saturating_sub(index) as f32 / release as f32
    } else {
        1.0
    }
}

fn percussive_decay(index: usize, total: usize, fraction: f32) -> f32 {
    let burst = (total as f32 * fraction).max(1.0) as usize;
    if index > burst {
        0.0
    } else {
        1.0 - index as f32 / burst as f32
    }
}

fn sine_osc(freq: f32, t: f32) -> f32 {
    (2.0 * PI * freq * t).sin()
}

fn soft_square_osc(freq: f32, t: f32) -> f32 {
    let fundamental = sine_osc(freq, t);
    let third = sine_osc(freq * 3.0, t) / 3.0;
    let fifth = sine_osc(freq * 5.0, t) / 5.0;
    (fundamental + third + fifth) * 0.8
}

fn semitone_ratio(semitones: f32) -> f32 {
    2.0_f32.powf(semitones / 12.0)
}

fn create_wav_header(buffer: &mut Vec<u8>, sample_rate: u32, sample_count: usize) {
    let data_size = (sample_count * 2) as u32;
    let chunk_size = 36 + data_size;

    buffer.extend_from_slice(b"RIFF");
    buffer.extend_from_slice(&chunk_size.to_le_bytes());
    buffer.extend_from_slice(b"WAVE");
    buffer.extend_from_slice(b"fmt ");
    buffer.extend_from_slice(&16u32.to_le_bytes());
    buffer.extend_from_slice(&1u16.to_le_bytes());
    buffer.extend_from_slice(&1u16.to_le_bytes());
    buffer.extend_from_slice(&sample_rate.to_le_bytes());
    buffer.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    buffer.extend_from_slice(&2u16.to_le_bytes());
    buffer.extend_from_slice(&16u16.to_le_bytes());
    buffer.extend_from_slice(b"data");
    buffer.extend_from_slice(&data_size.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_music_tier_for_level_ranges() {
        assert_eq!(AudioManager::tier_for_level(1), MusicTier::Low);
        assert_eq!(AudioManager::tier_for_level(4), MusicTier::Medium);
        assert_eq!(AudioManager::tier_for_level(8), MusicTier::High);
    }

    #[test]
    fn test_build_music_step_emits_audio() {
        let (duration_ms, wav_data) = build_music_step(
            MusicProfile {
                tier: MusicTier::Medium,
                danger: true,
                master_volume: 0.8,
                music_volume: 0.4,
            },
            3,
        );
        assert!(duration_ms > 0);
        assert!(wav_data.len() > 44);
        assert_eq!(&wav_data[0..4], b"RIFF");
    }
}
