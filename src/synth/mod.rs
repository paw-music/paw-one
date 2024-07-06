pub mod wavetable;

use defmt::{debug, warn};
use paw::audio::{osc::simple_form::SimpleFormSource, source::AudioSourceIter};

use crate::{midi::note::Note, AUDIO_BUFFER, SAMPLE_RATE};

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

    pub fn current_note(&self) -> Option<Note> {
        self.note
    }

    pub fn next_sample(&mut self) -> Option<f32> {
        if let Some(_) = self.note {
            Some(self.sound.next_sample() * 0.2)
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
        if let Some(free_voice) = self
            .voices
            .iter_mut()
            .position(|voice| voice.note.is_none())
        {
            debug!(
                "Note on {} [voice={}]",
                format!("{:?}", note).as_str(),
                free_voice
            );
            self.voices[free_voice].note_on(note);
        } else {
            debug!("No free voice to play [{}]", note);
        }
    }

    pub fn note_off(&mut self, note: Note) {
        if let Some(note_voice) = self
            .voices
            .iter_mut()
            .position(|voice| voice.note == Some(note))
        {
            debug!(
                "Note off {} [voice={}]",
                format!("{:?}", note).as_str(),
                note_voice
            );
            self.voices[note_voice].note_off();
        } else {
            warn!("No voice found with note [{}] to off", note);
        }
    }

    pub fn tick(&mut self) {
        cortex_m::interrupt::free(|cs| {
            let mut buffer = AUDIO_BUFFER.borrow(cs).borrow_mut();
            if !buffer.is_full() {
                let voices_sample: f32 = self
                    .voices
                    .iter_mut()
                    .filter_map(|voice| voice.next_sample())
                    .sum();

                let sample = (voices_sample * i32::MAX as f32) as i32;
                buffer.push_back((sample, sample)).ok();
            } else {
                // debug!("Buffer is full!");
            }
        });
        // if !self.queue.is_full() {
        //     let sample = (self.sound.next_sample() * 0.4 * i32::MAX as f32) as i32;
        //     self.queue.push_back((sample, sample)).ok();
        // } else {
        //     // info!("Buffer is full!");
        // }
    }

    pub fn active_voices(&self) -> impl Iterator<Item = &Voice> {
        self.voices.iter().filter(|voice| voice.note.is_some())
    }
}
