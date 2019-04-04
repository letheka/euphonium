use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct JSONArrangement {
    pub metadata: JSONMetadata,
    pub sample_banks: Vec<JSONSampleBank>,
    pub waveforms: Vec<JSONWaveform>,
    pub envelopes: Vec<JSONEnvelope>,
    pub instruments: Vec<JSONInstrument>,
    pub outputs: Vec<JSONOutput>,
}

#[derive(Serialize, Deserialize)]
pub struct JSONMetadata {
    pub input_file: String,
    pub comments: String
}

#[derive(Serialize, Deserialize)]
pub struct JSONSampleBank {
    pub name: String,
    pub files: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct JSONWaveform {
    pub name: String,
    pub equation: String
}

#[derive(Serialize, Deserialize)]
pub struct JSONEnvelope {
    pub name: String,
    pub phases: Vec<JSONEnvelopePhase>
}

#[derive(Serialize, Deserialize)]
pub struct JSONEnvelopePhase {
    pub start_time: f32,
    pub end_time: f32,
    pub start_val: f32,
    pub end_val: f32,
    pub ease_fn: String
}

#[derive(Serialize, Deserialize)]
pub struct JSONInstrument {
    pub name: String,
    pub midi_inst: u8,
    pub midi_percussion: bool,
    pub carrier: String,
    pub am: Vec<JSONModulator>,
}

#[derive(Serialize, Deserialize)]
pub struct JSONModulator {
    pub modulator: String,
    pub depth: f32
}

#[derive(Serialize, Deserialize)]
pub struct JSONOutput {
    pub output_file: String,
    pub channels: Vec<u8>
}
