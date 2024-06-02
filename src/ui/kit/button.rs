use core::borrow::Borrow;

use embedded_graphics::{
    geometry::{AnchorPoint, Size},
    mono_font::{self, MonoTextStyle},
    pixelcolor::BinaryColor,
    primitives::{PrimitiveStyle, Rectangle, StyledDrawable},
    Drawable,
};
use embedded_text::{
    style::{TextBoxStyle, TextBoxStyleBuilder},
    TextBox,
};

use crate::ui::widget::Widget;

pub struct Button {}

impl<'a, Env> Widget<'a, Env> for Button {
    fn update(
        &mut self,
        event: crate::ui::event::Event,
        ctx: &mut crate::ui::UiCtx<'a, Env>,
    ) -> crate::ui::event::EventResponse {
        Ok(crate::ui::event::EventIgnored::Ignored)
    }
}
