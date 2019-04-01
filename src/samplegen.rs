use std::collections::HashMap;

pub type Params = HashMap<String, f32>;

pub trait SampleGen {
    fn get_sample(&self, p: &Params) -> Option<f32>;

    // Return a value between 0.0 and 1.0 suitable for a modulator multiplied by depth.
    fn get_mod_sample(&self, p: &Params) -> Option<f32>;
}
