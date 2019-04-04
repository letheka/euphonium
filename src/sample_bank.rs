use hound;
use std::collections::HashMap;

use crate::samplegen::{Params, SampleGen};

#[derive(Clone)]
pub struct SampleBank {
    pub name: String,
    pub files: HashMap<String, String>,
    cache: HashMap<String, Vec<f32>>
}

impl SampleBank {
    pub fn new(name: String, files: HashMap<String, String>) -> SampleBank{
        SampleBank {
            name,
            files,
            cache: HashMap::new()
        }
    }

    fn resample(&self, snd: Vec<f32>, mult: f32) -> Vec<f32> {
        // A naive nearest neighbor resampling algorithm.
        // Should be replaced with something else, probably from a crate.
        let mut output = Vec::new();
        let len = snd.len() - 1;
        for s in 0..len {
            let loc = s as f32 * mult;
            let loc_floor = loc.floor();
            if loc > (len as f32) - 1.0 {
                // out of samples - push silence
                output.push(0.0);
            } else if (loc - loc_floor) < 0.5 {
                let sample = snd[loc_floor as usize] as f32;
                output.push(sample as f32);
            } else {
                let sample = snd[loc_floor as usize + 1] as f32;
                output.push(sample as f32);
            }
        }
        output
    }
}

impl SampleGen for SampleBank {
    fn cache(&mut self, p: &Params) {
        let midi_note = p["midi_note"].to_string();
        if !self.cache.contains_key(&midi_note) {
            if self.files.contains_key(&midi_note) {
                let mut reader = hound::WavReader::open(self.files.get(&midi_note).unwrap()).unwrap();
                let snd = reader.samples::<i16>().map(|s| s.unwrap() as f32).collect();
                self.cache.insert(midi_note, snd);
            } else {
                let str = format!("WARNING: Could not find sample with MIDI pitch {} - attempting to resample", p["midi_note"]);
                dbg!(str);
                let target_freq = 440.0 * (2.0_f32).powf((p["midi_note"] - 69.0) / 12.0);
                // find the closest midi note
                for i in 0..127 {
                    if self.files.contains_key(&(p["midi_note"] as u8 + i).to_string()) {
                        let closest_note = f32::from(p["midi_note"] as u8 + i);
                        let closest_freq = 440.0 * (2.0_f32).powf((closest_note - 69.0) / 12.0);
                        let mult = target_freq / closest_freq;
                        let mut reader = hound::WavReader::open(self.files.get(&closest_note.to_string()).unwrap()).unwrap();
                        let orig_snd = reader.samples::<i16>().map(|s| s.unwrap() as f32).collect();
                        let snd = self.resample(orig_snd, mult);
                        self.cache.insert(midi_note, snd);
                        break;
                    } else if self.files.contains_key(&(p["midi_note"] as u8 - i).to_string()) {
                        let closest_note = f32::from(p["midi_note"] as u8 - i);
                        let closest_freq = 440.0 * (2.0_f32).powf((closest_note - 69.0) / 12.0);
                        let mult = target_freq / closest_freq;
                        let mut reader = hound::WavReader::open(self.files.get(&closest_note.to_string()).unwrap()).unwrap();
                        let orig_snd = reader.samples::<i16>().map(|s| s.unwrap() as f32).collect();
                        let snd = self.resample(orig_snd, mult);
                        self.cache.insert(midi_note, snd);
                        break;
                    }
                }
            }
        }
    }

    fn get_sample(&self, p: &Params) -> Option<f32> {
        let midi_note = p["midi_note"].to_string();
        if self.cache.contains_key(&midi_note) {
            let sample = p["sample"] as usize;
            let snd = self.cache.get(&midi_note).unwrap();
            if snd.len() > sample {
                Some(self.cache.get(&midi_note).unwrap()[sample] as f32)
            } else {
                // The note is longer than the sample - return silence
                Some(0.0)
            }
        } else {
            // Sample may be missing from cache for unknown reason
            None
        }
    }

    fn get_mod_sample(&self, _p: &Params) -> Option<f32> {
        None
    }
}
