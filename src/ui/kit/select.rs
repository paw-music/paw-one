use embedded_graphics::{
    geometry::Point,
    image::Image,
    mono_font,
    pixelcolor::{BinaryColor, PixelColor},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable},
    text::renderer::{CharacterStyle, TextRenderer},
    Drawable,
};
use embedded_layout::View;
use embedded_text::{style::TextBoxStyleBuilder, TextBox};

use crate::{
    control::enc::EditByEncoder,
    ui::{
        mono_icons::{
            font_icons,
            icons_5x7::{self, MonoIcons5x7},
            MonoIcons,
        },
        Focus, Invertible,
    },
};

use super::input::InputEl;

pub trait SelectOption: core::fmt::Display {}

impl<'a> SelectOption for &'a str {}

pub struct Select<'a, O> {
    options: &'a [O],
    selected: usize,
    circular: bool,
}

#[derive(Clone)]
pub struct SelectView<'a, O>
where
    O: SelectOption,
{
    options: &'a [O],
    selected: usize,
    bounds: Rectangle,
    char_style: mono_font::MonoTextStyle<'a, BinaryColor>,

    /// Allow cycling through options circularly
    circular: bool,

    focused: bool,
}

impl<'a, O> SelectView<'a, O>
where
    O: SelectOption,
{
    pub fn new(
        options: &'a [O],
        preselected: usize,
        bounds: Rectangle,
        char_style: mono_font::MonoTextStyle<'a, BinaryColor>,
        circular: bool,
    ) -> Self {
        Self {
            options,
            selected: preselected,
            bounds,
            char_style,
            circular,
            focused: false,
        }
    }
}

impl<'a, O> View for SelectView<'a, O>
where
    O: SelectOption,
{
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_impl(by)
    }

    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

impl<'a, O> Focus for SelectView<'a, O>
where
    O: SelectOption,
{
    fn focused(&self) -> bool {
        self.focused
    }

    fn set_focus(&mut self, focus: bool) {
        self.focused = focus
    }
}

impl<'a, O> InputEl for SelectView<'a, O>
where
    O: SelectOption,
{
    type Value = O;

    fn value(&self) -> &Self::Value {
        &self.options[self.selected]
    }
}

impl<'a, O> EditByEncoder for SelectView<'a, O>
where
    O: SelectOption,
{
    type Meta = ();

    fn edit_by_encoder(&mut self, offset: i32, _vel: f32, _: Self::Meta) -> &mut Self {
        if self.circular {
            let new_selected = (self.selected as i32 + offset) % self.options.len() as i32;
            let new_selected = if new_selected < 0 {
                self.options.len() as i32 + new_selected
            } else {
                new_selected
            } as usize;

            assert!((0..self.options.len()).contains(&new_selected));

            self.selected = new_selected;
        } else {
            self.selected =
                (self.selected as i32 + offset).clamp(0, self.options.len() as i32) as usize;
        }

        self
    }
}

impl<'a, O> Drawable for SelectView<'a, O>
where
    O: SelectOption,
{
    type Color = BinaryColor;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        self.bounds.draw_styled(
            &PrimitiveStyle::with_fill(BinaryColor::Off).do_invert(self.focused),
            target,
        )?;

        let icons = font_icons(self.char_style.font).unwrap();

        Image::new(
            &icons.arrow_left().do_invert(self.focused),
            self.bounds.top_left + Point::new_equal(1),
        )
        .draw(target)?;

        let text_bounds = Rectangle::new(
            self.bounds.top_left + icons.size().x_axis() + Point::new(1, 0),
            self.bounds.size - icons.size().x_axis() * 2,
        );

        TextBox::with_textbox_style(
            &format!("{}", self.options[self.selected]),
            text_bounds,
            self.char_style.do_invert(self.focused),
            TextBoxStyleBuilder::new()
                .alignment(embedded_text::alignment::HorizontalAlignment::Center)
                .line_height(embedded_graphics::text::LineHeight::Pixels(
                    self.bounds.size.height,
                ))
                .build(),
        )
        .draw(target)?;

        Image::new(
            &icons.arrow_right().do_invert(self.focused),
            self.bounds.top_left + Point::new(self.bounds.size.width as i32 - 1, 1),
        )
        .draw(target)?;

        Ok(())
    }
}
