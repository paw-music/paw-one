pub mod event;

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
    pub oscs: [Osc; 3],
}
