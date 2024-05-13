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

use crate::{
    declare_component,
    ui::builder::{BlockComponent, ComponentProps},
};

use super::input::InputEl;

declare_component! {
    pub Button extends {block: block, text: text} {}
}
