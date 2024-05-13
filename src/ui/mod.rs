pub mod builder;
pub mod kit;
mod logo;
pub mod mono_icons;
pub mod page;
pub mod pages;

use embedded_graphics::{
    image::ImageRaw,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    primitives::{PrimitiveStyle, Rectangle},
    text::DecorationColor,
};
use embedded_text::style::TextBoxStyle;

use self::page::{Page, Pages};

pub mod text {
    use embedded_graphics::mono_font::{
        ascii::{FONT_4X6, FONT_5X7, FONT_6X9},
        MonoFont,
    };

    pub const FONT_SMALL: &MonoFont = &FONT_4X6;
    pub const FONT_MEDIUM: &MonoFont = &FONT_5X7;
    pub const FONT_BIG: &MonoFont = &FONT_6X9;
}

pub const LOGO: ImageRaw<'static, BinaryColor> = ImageRaw::new(&logo::LOGO, 128);

// pub trait PP {
//     fn pp(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result;
// }

// pub trait Element {
//     fn view(&self) ->
// }

pub trait Focus {
    fn focused(&self) -> bool;
    fn set_focus(&mut self, focus: bool);

    fn focus(&mut self) {
        self.set_focus(true)
    }
    fn blur(&mut self) {
        self.set_focus(false)
    }
}

pub trait ToHeaplessString {
    fn to_heapless_string<const SIZE: usize>(&self) -> heapless::String<SIZE>;
}

impl<T: core::fmt::Display> ToHeaplessString for T {
    fn to_heapless_string<const SIZE: usize>(&self) -> heapless::String<SIZE> {
        use core::fmt::Write;
        let mut s = heapless::String::<SIZE>::new();
        core::write!(s, "{}", self).unwrap();

        s
    }
}

pub trait Invertible {
    fn invert(&self) -> Self;

    fn do_invert(&self, invert: bool) -> Self
    where
        Self: Clone,
    {
        if invert {
            self.invert()
        } else {
            self.clone()
        }
    }
}

impl Invertible for Option<BinaryColor> {
    fn invert(&self) -> Self {
        self.map(|c| c.invert())
    }
}

impl Invertible for PrimitiveStyle<BinaryColor> {
    fn invert(&self) -> Self {
        let mut this = self.clone();
        this.fill_color = self.fill_color.invert();
        this.stroke_color = self.stroke_color.invert();
        this
    }
}

impl Invertible for DecorationColor<BinaryColor> {
    fn invert(&self) -> Self {
        match self {
            DecorationColor::None => *self,
            DecorationColor::TextColor => *self,
            DecorationColor::Custom(c) => Self::Custom(c.invert()),
        }
    }
}

impl<'a> Invertible for MonoTextStyle<'a, BinaryColor> {
    fn invert(&self) -> Self {
        let mut this = self.clone();
        this.text_color = this.text_color.invert();
        this.underline_color = this.underline_color.invert();
        this.background_color = this.background_color.invert();
        this.strikethrough_color = this.strikethrough_color.invert();
        this
    }
}
