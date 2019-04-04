//use crate::envelope::Envelope;
use crate::samplegen::{Params, SampleGen};
use crate::waveform::Waveform;

pub struct Modulator {
    pub modulator: Box<SampleGen>,
    pub depth: f32
}

pub struct Instrument {
    pub name: String,
    pub midi_inst: u8,
    pub midi_percussion: bool,
    pub carrier: Box<SampleGen>,
    pub am: Vec<Modulator>,
    //pub fm: Vec<Modulator>
}

impl SampleGen for Instrument {
    fn cache(&mut self, p: &Params) {
        self.carrier.cache(&p);
    }

    fn get_sample(&self, p: &Params) -> Option<f32> {
        let mut c = self.carrier.get_sample(&p).unwrap();
        for modulator in &self.am {
            match modulator.modulator.get_mod_sample(&p) {
                Some(m) => c *= 1.0 - m * modulator.depth,
                None => ()
            }
        }
        Some(c)
    }

    fn get_mod_sample(&self, _p: &Params) -> Option<f32> {
        None
    }
}
