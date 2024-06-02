pub mod event;
pub mod focus;
pub mod kit;
mod logo;
pub mod mono_icons;
pub mod page;
pub mod pages;
pub mod widget;
use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{
    image::ImageRaw,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    primitives::{PrimitiveStyle, Rectangle},
    text::DecorationColor,
};
use embedded_text::style::TextBoxStyle;

use self::{focus::Focus, page::Page, widget::Widget};

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

// pub trait Pages {}

pub struct UiCtx<'a, Env> {
    pub env: Env,
    marker: core::marker::PhantomData<&'a Env>,
    // pub router: Router<'a, P, Env>,
}

impl<'a, Env> UiCtx<'a, Env> {
    pub fn new(env: Env) -> Self {
        Self {
            env,
            marker: core::marker::PhantomData,
        }
    }
}

// pub struct Router<'a, P: Pages, Env> {
//     pages: Vec<Box<dyn Page<'a, Env> + 'a>>,
//     location: Id,
// }

// impl<'a, Id: PageId, Env> Router<'a, Id, Env> {
//     pub fn new() -> Self {
//         Self {
//             pages: vec![],
//             location: 0,
//         }
//     }

//     pub fn add_page(mut self, page: impl Page<'a, Env> + 'a) -> Self {
//         let page = Box::new(page);
//         self
//     }

//     pub fn goto(&mut self, location: Id) {
//         self.location = location;
//     }
// }

// pub struct UI<'a, Id: PageId, Env> {
//     ctx: UiCtx<'a, Id, Env>,
// }

// impl<'a, Id: PageId, Env> UI<'a, Id, Env> {
//     pub fn new(env: Env, router: Router<'a, Id, Env>) -> Self {
//         Self {
//             ctx: { UiCtx { env, router } },
//         }
//     }
// }

#[derive(Clone, Copy)]
pub enum ContainerFocus {
    None,
    Child(usize),
}

#[derive(Clone, Copy)]
pub enum FocusResult {
    Ok,
    Out(i32),
}

pub struct Container<'a, Env> {
    focus: ContainerFocus,
    children: Vec<Box<dyn Widget<'a, Env>>>,
}

impl<'a, Env> Container<'a, Env> {
    pub fn new(children: impl IntoIterator<Item = &'b dyn Widget<'a, Env>>) -> Self {
        Self {
            focus: ContainerFocus::None,
            children: children.into_iter().collect(),
        }
    }

    pub fn offset_focus(&mut self, offset: i32) -> FocusResult {
        match self.focus {
            ContainerFocus::None => todo!(), // Error?
            ContainerFocus::Child(id) => {
                let focused_child = id as i32 + offset;
                if focused_child < 0 || focused_child >= self.children.len() as i32 {
                    FocusResult::Out(focused_child)
                } else {
                    FocusResult::Ok
                }
            }
        }
    }
}

impl<'a, Env> Widget<'a, Env> for Container<'a, Env> {
    fn update(&mut self, event: event::Event, ctx: &mut UiCtx<'a, Env>) -> event::EventResponse {
        match self.focus {
            ContainerFocus::None => Ok(event::EventIgnored::Ignored),
            ContainerFocus::Child(child_id) => self.children[child_id].update(event, ctx),
        }
    }
}
