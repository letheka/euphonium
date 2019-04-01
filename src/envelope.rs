use easer::functions::*;

use std::f32::NAN;

use crate::samplegen::{Params, SampleGen};

#[derive(Clone)]
pub struct EnvPhase {
    pub start_time: f32,
    pub end_time: f32,
    pub start_val: f32,
    pub delta_val: f32,
    pub ease_fn: String,
}

#[derive(Clone)]
pub struct Envelope {
    phases: Vec<EnvPhase>,
}

impl Envelope {
    pub fn new(phases: Vec<EnvPhase>) -> Envelope {
        Envelope { phases }
    }
}

impl SampleGen for Envelope {
    fn get_sample(&self, _p: &Params) -> Option<f32> {
        None
    }

    fn get_mod_sample(&self, p: &Params) -> Option<f32> {
        let mut sample: f32 = NAN;
        let time = p["sample"] as u64;
        for phase in &self.phases {
            let start_time = (phase.start_time * (p["duration"] as f32)) as u64;
            let end_time = (phase.end_time * (p["duration"] as f32)) as u64;
            if time >= start_time && time <= end_time {
                let duration = (end_time - start_time) as f32;
                let frame = (time - start_time) as f32;
                sample = match &*phase.ease_fn {
                    "BackIn" => Back::ease_in(frame, phase.start_val, phase.delta_val, duration),
                    "BackOut" => Back::ease_out(frame, phase.start_val, phase.delta_val, duration),
                    "BackInOut" => {
                        Back::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "BounceIn" => {
                        Bounce::ease_in(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "BounceOut" => {
                        Bounce::ease_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "BounceInOut" => {
                        Bounce::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "CircIn" => Circ::ease_in(frame, phase.start_val, phase.delta_val, duration),
                    "CircOut" => Circ::ease_out(frame, phase.start_val, phase.delta_val, duration),
                    "CircInOut" => {
                        Circ::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "CubicIn" => Cubic::ease_in(frame, phase.start_val, phase.delta_val, duration),
                    "CubicOut" => {
                        Cubic::ease_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "CubicInOut" => {
                        Cubic::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "ElasticIn" => {
                        Elastic::ease_in(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "ElasticOut" => {
                        Elastic::ease_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "ElasticInOut" => {
                        Elastic::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "ExpoIn" => Expo::ease_in(frame, phase.start_val, phase.delta_val, duration),
                    "ExpoOut" => Expo::ease_out(frame, phase.start_val, phase.delta_val, duration),
                    "ExpoInOut" => {
                        Expo::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "Linear" => phase.start_val + phase.delta_val / (duration / frame),
                    "LinearIn" => phase.start_val + phase.delta_val / (duration / frame),
                    "LinearOut" => phase.start_val + phase.delta_val / (duration / frame),
                    "LinearInOut" => phase.start_val + phase.delta_val / (duration / frame),
                    "QuadIn" => Quad::ease_in(frame, phase.start_val, phase.delta_val, duration),
                    "QuadOut" => Quad::ease_out(frame, phase.start_val, phase.delta_val, duration),
                    "QuadInOut" => {
                        Quad::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "QuartIn" => Quart::ease_in(frame, phase.start_val, phase.delta_val, duration),
                    "QuartOut" => {
                        Quart::ease_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "QuartInOut" => {
                        Quart::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "QuintIn" => Quint::ease_in(frame, phase.start_val, phase.delta_val, duration),
                    "QuintOut" => {
                        Quint::ease_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "QuintInOut" => {
                        Quint::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    "SineIn" => Sine::ease_in(frame, phase.start_val, phase.delta_val, duration),
                    "SineOut" => Sine::ease_out(frame, phase.start_val, phase.delta_val, duration),
                    "SineInOut" => {
                        Sine::ease_in_out(frame, phase.start_val, phase.delta_val, duration)
                    }
                    _ => unimplemented!(),
                };
            }
        }
        // Invert the value so it can be multiplied by the modulator depth
        match sample {
            x if x == NAN => None,
            x if x != NAN => Some(1.0 - x),
            _ => unreachable!()
        }
    }
}
