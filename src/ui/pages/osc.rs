use core::marker::PhantomData;

use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
    primitives::{Primitive as _, PrimitiveStyle, Rectangle},
    Drawable,
};

use crate::{
    control::ControlsStateChanged,
    ui::page::{Page, PageId},
};

pub struct OscPage<'a> {
    marker: PhantomData<&'a str>,
}

impl<'a> OscPage<'a> {
    pub fn new() -> Self {
        Self {
            marker: Default::default(),
        }
    }
}

impl<'a> Page for OscPage<'a> {
    fn input(
        &mut self,
        control_panel: ControlsStateChanged,
    ) -> Result<crate::ui::page::PageEvent, crate::ui::page::PageError> {
        todo!()
    }
}

impl<'a> Drawable for OscPage<'a> {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        Rectangle::new(Point::zero(), Size::zero())
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
        Ok(())
    }
}
