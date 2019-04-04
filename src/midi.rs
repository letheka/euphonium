use ghakuf::messages::*;
use ghakuf::reader::*;

use std::collections::HashMap;

struct PlayingNote {
    channel: u8,
    program: u8,
    note: u8,
    velocity: u8,
    start_time: u64,
}

#[derive(Clone)]
pub struct Note {
    pub channel: u8,
    pub program: u8,
    pub start_time: u64,
    pub end_time: u64,
    pub midi_note: u8,
    pub freq: f32,
    // pub frequencies: Envelope,
    // pub amplitudes: Envelope,
}

#[derive(Default)]
pub struct MidiHandler {
    time_base: u16,
    tempo: u64,
    cur_time: u32,
    playing_notes: Vec<PlayingNote>,
    pub finished_notes: Vec<Note>,
    cur_programs: HashMap<u8, u8>,
}

impl MidiHandler {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Handler for MidiHandler {
    fn header(&mut self, _format: u16, _track: u16, time_base: u16) {
        // Time base is the number of ticks per quarter note
        self.time_base = time_base;
        // Tempo is the the number of samples per tick
        self.tempo = 44100 * 60 / 120;
    }
    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        match event {
            MetaEvent::EndOfTrack => {
                // Seems like the time is reset at the end of each track
                self.cur_time = 0;
            },
            MetaEvent::SetTempo => {
                let buf: [u8; 8] = [0, 0, 0, 0, 0, data[0], data[1], data[2]];
                let bpm = 60000000 / u64::from_be_bytes(buf);
                self.tempo = 44100 * 60 / bpm;
            }
            _ => (),
        }
        self.cur_time += delta_time;
    }
    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        self.cur_time += delta_time;
        match event {
            MidiEvent::NoteOn { ch, note, velocity } => {
                let program = match self.cur_programs.get(ch) {
                    Some(p) => *p,
                    _ => 0
                };
                self.playing_notes.push(PlayingNote {
                    channel: *ch,
                    program,
                    note: *note,
                    velocity: *velocity,
                    start_time: u64::from(self.cur_time) / (self.time_base as u64) * self.tempo,
                    // start_freq: 440.0 / (2.0).pow(((event.note as f32) / 12.0)),
                    // start_amp: (event.velocity as f32) / 128.0
                });
            }
            MidiEvent::NoteOff { ch, note, .. } => {
                // Try to find a matching playing note
                let mut pos = None;
                for n in 0..self.playing_notes.len() {
                    if self.playing_notes[n].channel == *ch && self.playing_notes[n].note == *note {
                        pos = Some(n);
                    };
                }
                match pos {
                    Some(n) => {
                        let pn = &self.playing_notes[n];
                        let start_time = pn.start_time;
                        let end_time = u64::from(self.cur_time) /  (self.time_base as u64) * self.tempo;
                        //let duration = end_time - start_time;
                        let freq = 440.0 * (2.0_f32).powf((f32::from(pn.note) - 69.0) / 12.0);
                        let _start_amp = f32::from(pn.velocity) / 128.0;
                        // let frequencies = Envelope::new(vec![EnvPhase {
                        //     start_time: 0,
                        //     end_time: duration,
                        //     start_val: start_freq,
                        //     delta_val: 0.0,
                        //     ease_fn: "Linear".to_string(),
                        // }]);
                        // let amplitudes = Envelope::new(vec![EnvPhase {
                        //     start_time: 0,
                        //     end_time: duration,
                        //     start_val: start_amp,
                        //     delta_val: 0.0,
                        //     ease_fn: "Linear".to_string(),
                        // }]);
                        self.finished_notes.push(Note {
                            channel: *ch,
                            program: pn.program,
                            start_time,
                            end_time,
                            midi_note: pn.note,
                            freq,
                            // amplitudes,
                        });
                        self.playing_notes.remove(n);
                    }
                    None => panic!("Reached a MIDI NoteOff event with no corresponding NoteOn"),
                }
            }
            MidiEvent::ProgramChange { ch, program } => {
                self.cur_programs.insert(*ch, *program);
            }
            _ => (),
        }
    }
    fn sys_ex_event(&mut self, delta_time: u32, _event: &SysExEvent, _data: &Vec<u8>) {
        self.cur_time += delta_time;
    }
    fn track_change(&mut self) {
        // unimplemented!();
    }
}
