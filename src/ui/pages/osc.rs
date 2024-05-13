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
