use alloc::{boxed::Box, string::ToString, vec::Vec};

use crate::synth::{Osc, Synth};

use super::{
    event::{Encoder, Event, EventIgnored, EventResponse},
    kit::label::Label,
    page::Page,
    widget::Widget,
    Container, FocusResult,
};

pub struct SynthEnv {
    synth: Synth,
}

struct OscRows {
    // kind: Select
    // sync: Select,
}

impl OscRows {
    fn new(osc: &Osc) -> Self {
        Self {}
    }
}

impl<'a> Widget<'a, SynthEnv> for OscRows {
    fn update(&mut self, event: Event, ctx: &mut super::UiCtx<'a, SynthEnv>) -> EventResponse {
        Ok(EventIgnored::Ignored)
    }
}

// enum OscPageFocus {
//     ChooseOsc(usize),
//     Osc(usize),
// }

pub struct OscPage<'a> {
    header: Container<'a, SynthEnv>,
    oscs: Container<'a, SynthEnv>,
    // focus: OscPageFocus,
}

impl<'a> OscPage<'a> {
    pub fn new(env: &SynthEnv) -> Self {
        Self {
            oscs: Container::new(
                env.synth
                    .oscs
                    .iter()
                    .map(|osc| Box::new(OscRows::new(osc)) as Box<dyn Widget<'a, SynthEnv>>)
                    .collect::<Vec<_>>(),
            ),
            // focus: OscPageFocus::ChooseOsc(0),
            header: Container::new(
                env.synth
                    .oscs
                    .iter()
                    .map(|osc| Box::new(Label::new(osc.name.as_str())))
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl<'a> Widget<'a, SynthEnv> for OscPage<'a> {
    fn update(
        &mut self,
        event: Event,
        ctx: &mut super::UiCtx<'a, SynthEnv>,
    ) -> super::event::EventResponse {
        match event {
            Event::Encoder(enc, offset, vel) => match enc {
                Encoder::Main => match self.oscs.offset_focus(offset) {
                    FocusResult::Ok => Ok(EventIgnored::Ignored),
                    FocusResult::Out(_) => todo!(),
                },
                Encoder::Red => todo!(),
                Encoder::Green => todo!(),
            },
            Event::EncoderButton(_, _) => todo!(),
        }
    }
}
