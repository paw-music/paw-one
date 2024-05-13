use embedded_graphics::{pixelcolor::BinaryColor, Drawable};
use embedded_layout::layout::linear::LinearLayout;

use crate::{
    control::{ControlPanel, ControlsState, ControlsStateChanged},
    synth::{self, event::SynthEvent},
};

use super::pages::{osc::OscPage, preset::PresetPage};

pub enum PageEvent {
    None,
    Synth(SynthEvent),
    NextPage,
    PrevPage,
}

#[derive(defmt::Format, Debug)]
pub enum PageError {}

pub trait Page: Drawable {
    fn input(&mut self, control_panel: ControlsStateChanged) -> Result<PageEvent, PageError>;
}

macro_rules! declare_pages {
    ($($name: ident: $ty: ident),* $(,)?) => {
        pub enum PageId {
            $($name),*
        }

        pub enum Pages<'a> {
            $($name($ty<'a>)),*
        }

        impl<'a> Page for Pages<'a> {
            fn input(&mut self, control_panel: crate::control::ControlsStateChanged) -> Result<PageEvent, PageError> {
                match self {
                    $(Self::$name(page) => page.input(control_panel)),*
                }
            }
        }

        impl<'a> Drawable for Pages<'a> {
            type Color = BinaryColor;
            type Output = ();

            fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
                where
                    D: embedded_graphics::prelude::DrawTarget<Color = Self::Color> {
                match self {
                    $(Self::$name(page) => page.draw(target)),*
                }
            }
        }
    };
}

declare_pages! {
    Preset: PresetPage,
    Osc: OscPage,
}

pub struct PageFactory;

impl PageFactory {
    pub fn page(&self, id: PageId) -> Pages {
        match id {
            PageId::Preset => Pages::Preset(PresetPage::new()),
            PageId::Osc => Pages::Osc(OscPage::new()),
        }
    }
}

#[macro_export]
macro_rules! make_page {
    ($vis: vis $name: ident {
        $($field: ident: $field_ty: ty = $field_val: expr,)*

        @draw
        $($el: ident: $el_ty: ident {$($props: tt)*})*
    } $(focus $default_focus: ident,)?
    @schema {
        color: $color_ty: ty
    }
    ) => {
        mod focus {
            #[allow(non_camel_case_types)]
            #[derive(Clone, Copy)]
            pub enum Focus {
                $($el),*
            }

            const FOCUSES: &[Focus] = &[$(Focus::$el),*];

            $(
                impl Default for Focus {
                    fn default() -> Self {
                        Self::$default_focus
                    }
                }
            )?

            impl TryFrom<i32> for Focus {
                type Error = ();

                fn try_from(value: i32) -> Result<Self, Self::Error> {
                    if value > 0 && value < FOCUSES.len() as i32 {
                        Err(())
                    } else {
                        Ok(FOCUSES[value as usize])
                    }
                }
            }

            impl core::ops::Add<i32> for Focus {
                type Output = Self;

                fn add(self, rhs: i32) -> Self::Output {
                    let size = FOCUSES.len() as i32;
                    let i = self as i32 + rhs;
                    ((size + i % size) % size).try_into().unwrap()
                }
            }
        }

        $vis struct $name <'a> {
            /// What is focused on the page?
            focus: focus::Focus,
            /// Is page focused?
            focused: bool,
            $($field: $field_ty),*
            $($el: $el_ty<'a>),*
        }

        impl<'a> $name<'a> {
            pub fn new() -> Self {
                Self {
                    focus: focus::Focus::default(),
                    focused: false,
                    $($field: $field_val),*
                    $($el: crate::ui::builder::component!($el_ty {$($props)*} @schema {color: $color_ty})),*
                }
            }
        }

        impl<'a> Drawable for $name<'a> {
            type Color = BinaryColor;
            type Output = ();

            fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
            where
                D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
            {
                $(self.$el.draw(target)?;)*
                Ok(())
            }
        }
    };
}

pub(crate) use make_page;
