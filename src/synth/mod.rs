use paw::audio::{osc::simple_form::SimpleFormSource, source::AudioSourceIter};

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

pub struct Osc {
    pub kind: OscKind,
    pub name: OscName,
}

pub struct Synth {
    // pub oscs: [Osc; 3],
    sound: SimpleFormSource,
    // queue: heapless::Deque<(i32, i32), AUDIO_BUFFER_SIZE>,
}

impl Synth {
    pub fn new() -> Self {
        let sound = SimpleFormSource::infinite_mono(
            SAMPLE_RATE,
            paw::audio::osc::simple_form::WaveForm::Sine,
            220.0,
        );

        Self {
            sound, // buffer: Default::default(),
                   // queue: Default::default(),
        }
    }

    // For test
    pub fn set_freq(&mut self, freq: f32) {
        self.sound.set_freq(freq)
    }

    // pub fn next_frame(&mut self) -> [u16; 4] {
    //     let sample = (self.sound.next_sample() * 0.4 * i32::MAX as f32) as i32;
    //     let bytes = sample.to_be_bytes();

    //     let mut frame = [0; 4];
    //     frame[0] = u16::from_be_bytes([bytes[0], bytes[1]]);
    //     frame[1] = frame[0];
    //     frame[2] = u16::from_be_bytes([bytes[2], bytes[3]]);
    //     frame[3] = frame[2];

    //     frame
    // }

    // pub fn next_frame(&mut self) -> (i32, i32) {
    //     let sample = (self.sound.next_sample() * 0.4 * i32::MAX as f32) as i32;
    //     (sample, sample)
    // }

    // pub fn take_buffer(&mut self) -> &'static mut DmaAudioBuffer {
    //     let buffer = cortex_m::singleton!(: DmaAudioBuffer = [0; DMA_AUDIO_BUFFER_SIZE]).unwrap();

    //     let mut pointer = 0;
    //     while let Some(frame) = self.queue.pop_front() {
    //         let mut data = [0; 4];
    //         let left = frame.0.to_be_bytes();
    //         let right = frame.1.to_be_bytes();

    //         data[0..2].copy_from_slice(&[
    //             u16::from_be_bytes([left[0], left[1]]),
    //             u16::from_be_bytes([left[2], left[3]]),
    //         ]);

    //         data[2..4].copy_from_slice(&[
    //             u16::from_be_bytes([right[0], right[1]]),
    //             u16::from_be_bytes([right[2], right[3]]),
    //         ]);

    //         buffer[pointer..pointer + 4].copy_from_slice(&data);
    //         pointer += 4;
    //     }

    //     if pointer < buffer.len() {
    //         panic!("Underrun");
    //     }

    //     buffer
    // }

    // pub fn buffer(&mut self, next: &mut DmaAudioBuffer) -> &DmaAudioBuffer {
    //     self.
    // }

    pub fn tick(&mut self) {
        cortex_m::interrupt::free(|cs| {
            let mut buffer = AUDIO_BUFFER.borrow(cs).borrow_mut();
            if !buffer.is_full() {
                let sample = (self.sound.next_sample() * 0.4 * i32::MAX as f32) as i32;
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
