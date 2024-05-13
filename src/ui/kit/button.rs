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

use crate::ui::{
    builder::{BlockComponent, ComponentProps},
    Focus,
};

use super::input::InputEl;

pub struct Button<'a> {
    block: BlockComponent<BinaryColor>,
    text: TextBox<'a, MonoTextStyle<'a, BinaryColor>>,
    value: bool,
    focused: bool,
}

// pub struct ButtonBuilder<> {
//     bounds: Rectangle,
//     text: alloc::string::String,
//     char_style: Option<MonoTextStyle<'a, BinaryColor>>,
// }

impl<'a> From<ComponentProps<'a, BinaryColor>> for Button<'a> {
    fn from(ComponentProps { block, text, .. }: ComponentProps<'a, BinaryColor>) -> Self {
        Self {
            block,
            text,
            value: false,
            focused: false,
        }
    }
}

// impl BoundsBuilder for ButtonBuilder {
//     fn bounds_mut(&mut self) -> &mut Rectangle {
//         &mut self.bounds
//     }
// }

// impl ComponentBuilder for ButtonBuilder {
//     type Comp = Button;

//     fn new() -> Self {
//         Self {
//             bounds: Rectangle::zero(),
//             text: alloc::string::String::new(),
//             char_style: None,
//         }
//     }

//     fn bounds(mut self, bounds: Rectangle) -> Self {
//         self.bounds = bounds;
//         self
//     }

//     fn build(self) -> Self::Comp {
//         Button {
//             bounds: self.bounds,
//             text: self.text,
//             char_style: self.char_style.unwrap(),
//             value: false,
//             focused: false,
//         }
//     }
// }

// impl Component for Button {
//     type Builder = ButtonBuilder;
// }

impl<'a> Focus for Button<'a> {
    fn focused(&self) -> bool {
        self.focused
    }

    fn set_focus(&mut self, focus: bool) {
        self.focused = focus
    }
}

impl<'a> InputEl for Button<'a> {
    type Value = bool;

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl<'a> Drawable for Button<'a> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        self.block.draw(target)?;
        self.text.draw(target)?;

        Ok(())

        // Add 1 pixel for border and 1 for margin between border and text
        // let textbox_bounds = self.bounds.resized(
        //     self.bounds.size.saturating_sub(Size::new_equal(2)),
        //     AnchorPoint::Center,
        // );

        // TextBox::with_textbox_style(
        //     &self.text,
        //     textbox_bounds,
        //     self.char_style,
        //     TextBoxStyleBuilder::new()
        //         .alignment(embedded_text::alignment::HorizontalAlignment::Center)
        //         .line_height(embedded_graphics::text::LineHeight::Pixels(
        //             textbox_bounds.size.height,
        //         ))
        //         .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
        //         .build(),
        // )
        // .draw(target)?;
    }
}
