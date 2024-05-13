use embedded_graphics::Drawable;
use embedded_layout::layout::linear::LinearLayout;

use crate::{
    control::{ControlPanel, ControlsState, ControlsStateChanged},
    synth::{self, event::SynthEvent},
};

use super::{
    builder::{Component, DefaultColor},
    pages::preset::PresetPage,
};

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

        pub enum Pages<'a, C: $crate::ui::builder::DefaultColor> {
            $($name($ty<'a, C>)),*
        }

        impl<'a, C: $crate::ui::builder::DefaultColor> Page for Pages<'a, C> {
            fn input(&mut self, control_panel: crate::control::ControlsStateChanged) -> Result<PageEvent, PageError> {
                match self {
                    $(Self::$name(page) => page.input(control_panel)),*
                }
            }
        }

        impl<'a, C: $crate::ui::builder::DefaultColor> Drawable for Pages<'a, C> {
            type Color = C;
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
    // Osc: OscPage,
}

pub struct PageFactory;

impl PageFactory {
    pub fn page<C: DefaultColor>(&self, id: PageId) -> Pages<C> {
        match id {
            PageId::Preset => Pages::Preset(PresetPage::new()),
        }
    }
}

#[macro_export]
macro_rules! make_page {
    ($vis: vis $name: ident {
        $($field: ident: $field_ty: ty = $field_val: expr,)*

        @draw
        $($el: ident: $el_ty: ident $($child_props: tt)?)*
    } $(focus $default_focus: ident,)?
    @schema {
        color: $color_ty: ty
    }
    ) => {
        mod focus {
            $crate::ui::focus::declare_focus! {
                $($el),* $(default $default_focus)?
            }
        }

        $vis struct $name <'a, C: $crate::ui::builder::DefaultColor> {
            /// What is focused on the page?
            focus: focus::Focus,
            /// Is page focused?
            focused: bool,
            $($field: $field_ty),*
            $($el: $el_ty<'a, C>),*
        }

        impl<'a, C: $crate::ui::builder::DefaultColor> $name<'a, C> {
            pub fn new() -> Self {
                Self {
                    focus: focus::Focus::default(),
                    focused: false,
                    $($field: $field_val),*
                    $($el: crate::ui::builder::component!($el_ty $($child_props)?)),*
                }
            }
        }

        impl<'a, C: $crate::ui::builder::DefaultColor> Drawable for $name<'a, C> {
            type Color = C;
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
