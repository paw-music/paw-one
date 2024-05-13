use embedded_graphics::{
    geometry::{OriginDimensions, Size},
    image::{ImageDrawable, ImageRaw},
    mono_font::{self, MonoFont},
    pixelcolor::BinaryColor,
    Drawable,
};

use self::icons_5x7::MonoIcons5x7;

pub mod icons_5x7;

type Icon = ImageRaw<'static, BinaryColor>;

pub struct MonoIcon {
    raw: Icon,
    inverted: Icon,
}

impl MonoIcon {
    fn get(&self, inverted: bool) -> &Icon {
        if inverted {
            &self.inverted
        } else {
            &self.raw
        }
    }
}

#[derive(Clone, Copy)]
pub struct MonoIconDrawable<'a> {
    icon: &'a MonoIcon,
    inverted: bool,
}

impl<'a> OriginDimensions for MonoIconDrawable<'a> {
    fn size(&self) -> Size {
        self.icon.get(self.inverted).size()
    }
}

impl<'a> ImageDrawable for MonoIconDrawable<'a> {
    type Color = BinaryColor;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        self.icon.get(self.inverted).draw(target)
    }

    fn draw_sub_image<D>(
        &self,
        target: &mut D,
        area: &embedded_graphics::primitives::Rectangle,
    ) -> Result<(), D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        self.icon.get(self.inverted).draw_sub_image(target, area)
    }
}

impl<'a> Invertible for MonoIconDrawable<'a> {
    fn invert(&self) -> Self {
        Self {
            icon: self.icon,
            inverted: !self.inverted,
        }
    }
}

macro_rules! declare_mono_icons {
    ($($name: ident: $fn: ident),* $(,)?) => {
        $(
            const $name: MonoIcon;

            fn $fn(&self) -> MonoIconDrawable {
                MonoIconDrawable {
                    icon: &Self::$name,
                    inverted: false,
                }
            }
        )*
    };
}

pub trait MonoIcons: Sized {
    declare_mono_icons! {
        ARROW_LEFT: arrow_left,
        ARROW_RIGHT: arrow_right
    }

    fn size(&self) -> Size;
}

pub const fn font_icons(mono_font: &MonoFont) -> Option<impl MonoIcons> {
    match mono_font.character_size {
        Size {
            width: 5,
            height: 7,
        } => Some(MonoIcons5x7),
        _ => None,
    }
}

macro_rules! create_icons {
    ($vis: vis $name: ident / $inv_name: ident: $width: literal * $height: literal {
        $($icon: ident = &[$($data: literal),* $(,)?]),* $(,)?
    }) => {
        $vis struct $name;
        $vis struct $inv_name;

        impl MonoIcons for $name {
            $(
                const $icon: crate::ui::mono_icons::MonoIcon = crate::ui::mono_icons::MonoIcon {
                    raw: ImageRaw::new(&[$($data),*], $width),
                    inverted: ImageRaw::new(&[$(!$data),*], $width),
                };
            )*

            fn size(&self) -> embedded_graphics::geometry::Size {
                embedded_graphics::geometry::Size::new($width, $height)
            }
        }
    };
}

pub(crate) use create_icons;

use super::Invertible;
