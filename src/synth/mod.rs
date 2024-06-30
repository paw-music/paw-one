pub mod wavetable;

use defmt::debug;
use micromath::F32Ext;
use paw::audio::{osc::simple_form::SimpleFormSource, source::AudioSourceIter};
use usbd_midi::data::midi::notes::Note;

use crate::{AUDIO_BUFFER, SAMPLE_RATE};

#[derive(Clone, Copy)]
pub enum OscKind {
    Wave,
    Noise,
}

#[derive(Clone, Copy)]
pub enum OscName {
    Osc1,
    Osc2,
    Osc3,
}

impl OscName {
    pub fn as_str(&self) -> &'static str {
        match self {
            OscName::Osc1 => "OSC1",
            OscName::Osc2 => "OSC2",
            OscName::Osc3 => "OSC3",
        }
    }
}

impl core::fmt::Display for OscName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

trait NoteFrequency {
    fn freq(&self) -> f32;
}

impl NoteFrequency for Note {
    fn freq(&self) -> f32 {
        440.0 * 2f32.powf((Into::<u8>::into(*self) as f32 - 69.0) / 12.0)
    }
}

pub struct Osc {
    pub kind: OscKind,
    pub name: OscName,
}

pub struct Voice {
    sound: SimpleFormSource,
    note: Option<Note>,
}

impl Voice {
    pub fn note_on(&mut self, note: Note) {
        self.sound.set_freq(note.freq());
        self.note = Some(note);
    }

    pub fn note_off(&mut self) {
        self.note = None;
    }

    pub fn next_sample(&mut self) -> Option<f32> {
        if let Some(_) = self.note {
            Some(self.sound.next_sample() * 0.4)
        } else {
            None
        }
    }
}

impl Default for Voice {
    fn default() -> Self {
        let sound = SimpleFormSource::infinite_mono(
            SAMPLE_RATE,
            paw::audio::osc::simple_form::WaveForm::Sine,
            0.0,
        );

        Self { sound, note: None }
    }
}

pub struct Synth {
    voices: [Voice; 16],
}

impl Synth {
    pub fn new() -> Self {
        Self {
            voices: Default::default(),
            // buffer: Default::default(),
            // queue: Default::default(),
        }
    }

    pub fn note_on(&mut self, note: Note) {
        debug!("Note on {}", format!("{:?}", note).as_str());
        if let Some(free_voice) = self.voices.iter_mut().find(|voice| voice.note.is_none()) {
            free_voice.note_on(note);
        }
    }

    pub fn note_off(&mut self, note: Note) {
        debug!("Note off {}", format!("{:?}", note).as_str());
        if let Some(note_voice) = self
            .voices
            .iter_mut()
            .find(|voice| voice.note == Some(note))
        {
            note_voice.note_off();
        }
    }

    pub fn tick(&mut self) {
        let voices_sample: f32 = self
            .voices
            .iter_mut()
            .filter_map(|voice| voice.next_sample())
            .sum();

        cortex_m::interrupt::free(|cs| {
            let mut buffer = AUDIO_BUFFER.borrow(cs).borrow_mut();
            if !buffer.is_full() {
                let sample = (voices_sample * i32::MAX as f32) as i32;
                buffer.push_back((sample, sample)).ok();
            } else {
                // info!("Buffer is full!");
            }
        });
        // if !self.queue.is_full() {
        //     let sample = (self.sound.next_sample() * 0.4 * i32::MAX as f32) as i32;
        //     self.queue.push_back((sample, sample)).ok();
        // } else {
        //     // info!("Buffer is full!");
        // }
    }
}
