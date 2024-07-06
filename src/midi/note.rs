use alloc::string::ToString;
use micromath::F32Ext;
use num_enum::{IntoPrimitive, TryFromPrimitive};

macro_rules! declare_notes {
    ($($note: ident),* $(,)?) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format, IntoPrimitive, TryFromPrimitive)]
        #[repr(u8)]
        pub enum Note {
            $($note),*
        }

        impl Note {
            pub fn name(&self) -> &str {
                match *self {
                    $(Self::$note => stringify!($note),)*
                }
            }
        }

        impl From<usbd_midi::data::midi::notes::Note> for Note {
            fn from(value: usbd_midi::data::midi::notes::Note) -> Self {
                match value {
                    $(usbd_midi::data::midi::notes::Note::$note => Self::$note,)*
                }
            }
        }
    };
}

declare_notes! {
    C1m, Cs1m, D1m, Ds1m, E1m, F1m, Fs1m, G1m, Gs1m, A1m, As1m, B1m, C0, Cs0, D0, Ds0, E0, F0, Fs0, G0, Gs0, A0, As0, B0, C1, Cs1, D1, Ds1, E1, F1, Fs1, G1, Gs1, A1, As1, B1, C2, Cs2, D2, Ds2, E2, F2, Fs2, G2, Gs2, A2, As2, B2, C3, Cs3, D3, Ds3, E3, F3, Fs3, G3, Gs3, A3, As3, B3, C4, Cs4, D4, Ds4, E4, F4, Fs4, G4, Gs4, A4, As4, B4, C5, Cs5, D5, Ds5, E5, F5, Fs5, G5, Gs5, A5, As5, B5, C6, Cs6, D6, Ds6, E6, F6, Fs6, G6, Gs6, A6, As6, B6, C7, Cs7, D7, Ds7, E7, F7, Fs7, G7, Gs7, A7, As7, B7, C8, Cs8, D8, Ds8, E8, F8, Fs8, G8, Gs8, A8, As8, B8, C9, Cs9, D9, Ds9, E9, F9, Fs9, G9, Gs9,
}

impl Note {
    pub fn freq(self) -> f32 {
        440.0 * 2f32.powf((self as u8 as f32 - 69.0) / 12.0)
    }

    pub fn transpose(self, offset: i8) -> Note {
        TryFromPrimitive::try_from_primitive(
            (self as u8 as i8).saturating_add(offset).clamp(0, i8::MAX) as u8,
        )
        .unwrap()
    }
}

impl ToString for Note {
    fn to_string(&self) -> alloc::string::String {
        self.name().to_string()
    }
}
