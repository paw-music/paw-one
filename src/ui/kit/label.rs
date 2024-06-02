use alloc::string::String;
use embedded_graphics::{
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    primitives::Rectangle,
};
use embedded_text::TextBox;

use crate::ui::widget::Widget;

pub struct Label<'a> {
    textbox: TextBox<'a, MonoTextStyle<'a, BinaryColor>>,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            // TODO: Props
            textbox: TextBox::new(text, Rectangle::zero(), MonoTextStyleBuilder::new().build()),
        }
    }
}

impl<'a, Env> Widget<'a, Env> for Label<'a> {
    fn update(
        &mut self,
        _event: crate::ui::event::Event,
        _ctx: &mut crate::ui::UiCtx<'a, Env>,
    ) -> crate::ui::event::EventResponse {
        Ok(crate::ui::event::EventIgnored::Ignored)
    }
}
