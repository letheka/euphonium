extern crate failure;
#[macro_use] extern crate failure_derive;

extern crate serde;
#[macro_use] extern crate serde_derive;

use failure::Error;

use ghakuf::reader::*;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod envelope;
use envelope::{EnvPhase, Envelope};

pub mod instrument;
use instrument::{Modulator, Instrument};

pub mod midi;
use midi::{MidiHandler, Note};

pub mod parse;
use parse::{JSONArrangement};

pub mod sample_bank;
use sample_bank::SampleBank;

pub mod samplegen;
use samplegen::{Params, SampleGen};

pub mod waveform;
use waveform::Waveform;

pub mod error;

fn read_midi_file(h: &mut MidiHandler, p: &Path) {
    // This function does no error handling as it causes lifetime problems.
    // Any errors thrown by the reader will be ugly, but it's the user's fault for supplying an
    // invalid MIDI file.
    let mut reader = Reader::new(h, p).unwrap();
    let _ = reader.read();
}

fn parse_midi_file(f: String) -> Result<Vec<Note>, Error> {
    let p = Path::new(&f);

    // Check if the path is valid
    let _ = fs::metadata(p)?;

    let mut handler = MidiHandler::new();
    read_midi_file(&mut handler, &p);
    Ok(handler.finished_notes.clone())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);
    let json: JSONArrangement = serde_json::from_reader(reader)?;

    // Read the input midi file
    let input_file = json.metadata.input_file;
    let notes = parse_midi_file(input_file)?;

    // Create sample banks based on the JSON parameters
    let mut sample_banks = HashMap::new();
    for b in json.sample_banks {
        let name = b.name.to_string();
        sample_banks.insert(name, SampleBank::new(b.name.to_string(), b.files));
    }

    // Create waveforms based on the JSON parameters
    let mut waveforms = HashMap::new();
    for w in json.waveforms {
        let name = w.name.to_string();
        waveforms.insert(name, Waveform::new(w.equation.to_string()));
    }

    //Create envelopes based on the JSON parameters
    let mut envelopes = HashMap::new();
    for e in json.envelopes {
        let name = e.name.to_string();
        let mut phases = Vec::new();
        for p in e.phases {
            let efn = p.ease_fn.to_owned();
            phases.push(EnvPhase {
                start_time: p.start_time,
                end_time: p.end_time,
                start_val: p.start_val,
                delta_val: p.end_val,
                ease_fn: efn
            });
        }
        envelopes.insert(name, Envelope::new(phases));
    }

    //Create instruments based on the JSON parameters
    let mut instruments = HashMap::new();
    for i in json.instruments {
        let name = i.name.to_string();

        let mut am = Vec::new();
        for m in i.am {
            // TODO: allow using instruments as modulators
            let mod_name = m.modulator.to_string();
            if waveforms.contains_key(&mod_name) {
                am.push(Modulator {
                    modulator: Box::new(waveforms[&mod_name].clone()),
                    depth: m.depth
                });
            } else if envelopes.contains_key(&mod_name) {
                am.push(Modulator {
                    modulator: Box::new(envelopes[&mod_name].clone()),
                    depth: m.depth
                });
            }
        }
        // What type is the carrier?
        if sample_banks.contains_key(&i.carrier) {
            instruments.insert(name.clone(), Instrument {
                name,
                midi_inst: i.midi_inst,
                midi_percussion: i.midi_percussion,
                carrier: Box::new(sample_banks[&i.carrier].clone()),
                am
            });
        } else if waveforms.contains_key(&i.carrier) {
            instruments.insert(name.clone(), Instrument {
                name,
                midi_inst: i.midi_inst,
                midi_percussion: i.midi_percussion,
                carrier: Box::new(waveforms[&i.carrier].clone()),
                am
            });
        } else {
            unimplemented!()
        }
    }

    // Figure out which MIDI channels we need to pay attention to
    let mut output_channels: HashMap<u8, Vec<f32>> = HashMap::new();
    for o in &json.outputs {
        for c in &o.channels {
            if !output_channels.contains_key(&c) {
                output_channels.insert(*c, Vec::new());
            }
        }
    }

    // Find the end time of the final note
    // TODO: Trim silence from the beginning and end of every output
    let mut final_note: usize = 0;
    for n in &notes {
        if output_channels.contains_key(&n.channel) && n.end_time > (final_note as u64) {
            final_note = n.end_time as usize;
        }
    }

    for o in json.outputs {
        let mut output = vec![0.0; final_note];

        let mut loudest = 0.0;
        for n in &notes {
            if o.channels.contains(&n.channel) {
                let dur = n.end_time - n.start_time;
                let begin = n.start_time;

                // Figure out which instrument to use
                let mut maybe_inst = None;
                for i in instruments.values() {
                    if !i.midi_percussion && n.program == i.midi_inst {
                        maybe_inst = Some(i.name.clone());
                    }
                }
                if maybe_inst == None {
                    let error = format!("Could not find an instrument mapped to MIDI patch {:?}", n.program);
                    panic!(error)
                }
                let inst = maybe_inst.unwrap();

                let mut cache_p: Params = HashMap::new();
                cache_p.insert("midi_note".to_string(), n.midi_note as f32);
                instruments.get_mut(&inst).unwrap().cache(&cache_p);

                for s in 0..dur {
                    let mut p: Params = HashMap::new();
                    p.insert("duration".to_string(), dur as f32);
                    p.insert("sample".to_string(), s as f32);
                    p.insert("time".to_string(), s as f32 / 44100.0);
                    p.insert("rate".to_string(), 44100.0);
                    p.insert("midi_note".to_string(), n.midi_note as f32);
                    p.insert("x".to_string(), s as f32 * n.freq as f32 / 44100.0);
                    // instruments.get_mut(&inst).unwrap().cache(&p);
                    let o = instruments[&inst].get_sample(&p).unwrap();
                    output[(begin + s) as usize] += o;
                    if output[(begin + s) as usize] > loudest {
                        loudest = output[(begin + s) as usize];
                    }
                }
            }

            // Prepare to write the output as notes
            let int_max = i16::max_value() as f32;
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: 44100,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let filename = o.output_file.clone();
            let mut writer = hound::WavWriter::create(filename, spec).unwrap();

            for t in 0..final_note {
                let sample = ((output[t] / loudest) * int_max) as i16;
                writer.write_sample(sample).unwrap();
            }
        }
    }

    Ok(())
}
