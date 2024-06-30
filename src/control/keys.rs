use alloc::{string::String, vec::Vec};
use bitflags::bitflags;
use defmt::debug;
use usbd_midi::data::midi::notes::Note;

// macro_rules! declare_keys {
//     (@next_key $prev: expr) => {
//         1 << (prev + 1)
//     };

//     (@keys $next: expr; $key: ident, $($rest: ident),*) => {
//         const $key = 1 << $next;
//         declare_keys!(@keys $next + 1; $($rest),*);
//     };

//     (@keys) => {};

//     ($($key: ident),*) => {
//         bitflags! {
//         }

//         impl defmt::Format for Keys {
//             fn format(&self, fmt: defmt::Formatter) {
//                 defmt::write!(fmt, "C1")
//             }
//         }
//     };
// }

// declare_keys! {

// }

pub trait MidiNoteExt {
    fn from_name(name: &str) -> Self;
    fn transpose(offset: i16) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub enum KeysMessage {
    NoteOn(Note),
    NoteOff(Note),
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Keys: u16 {
        const C  = 1 << 0;
        const CS = 1 << 1;
        const D  = 1 << 2;
        const DS = 1 << 3;
        const E  = 1 << 4;
        const F  = 1 << 5;
        const FS = 1 << 6;
        const G  = 1 << 7;
        const GS = 1 << 8;
        const A  = 1 << 9;
        const AS = 1 << 10;
        const B  = 1 << 11;
    }
}

impl defmt::Format for Keys {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "{}",
            self.iter_names()
                .map(|(name, b)| format!("{name}={}|", self.contains(b)))
                .collect::<String>()
                .as_str()
        )
    }
}

impl Keys {
    // pub fn into_notes(self, transpose: u8) -> Vec<Note> {
    //     self.iter_names()
    //         .enumerate()
    //         .filter_map(|(index, (name, keys))| {
    //             debug!("index {}; name {}", index, name);
    //             if !keys.is_empty() {
    //                 Some(Note::try_from(index as u8 + transpose).unwrap())
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect()
    // }

    pub fn expect_single_note(self) -> Note {
        (match self.iter_names().next().unwrap().0 {
            "C" => 0,
            "CS" => 1,
            "D" => 2,
            "DS" => 3,
            "E" => 4,
            "F" => 5,
            "FS" => 6,
            "G" => 7,
            "GS" => 8,
            "A" => 9,
            "AS" => 10,
            "B" => 11,
            _ => unreachable!(),
        })
        .try_into()
        .unwrap()
    }

    pub fn into_midi(self, prev: Keys) -> Vec<KeysMessage> {
        self.difference(prev)
            .iter()
            .map(
                |set| match (!(prev & set).is_empty(), !(self & set).is_empty()) {
                    (false, true) => KeysMessage::NoteOn(set.expect_single_note()),
                    (true, false) => KeysMessage::NoteOff(set.expect_single_note()),
                    (_, _) => unreachable!(),
                },
            )
            .collect()
    }
}
