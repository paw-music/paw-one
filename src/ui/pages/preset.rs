use embedded_graphics::{
    geometry::{Point, Size},
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::Drawable,
    primitives::{
        Primitive, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle,
        StyledDimensions,
    },
};
use embedded_text::TextBox;

use crate::{
    control::{enc::EncoderState, ControlsStateChanged},
    ui::{
        builder::component,
        kit::button::Button,
        page::{make_page, Page, PageError, PageEvent},
        text::FONT_SMALL,
        Focus,
    },
};

make_page! {
    pub PresetPage {
        @draw

        edit_btn: Button {
            text: "EDIT";
            font: &FONT_SMALL;
            width: 22;
            height: 11;
            border_radius: 3;
        }

        save_btn: Button {
            text: "SAVE";
            font: &FONT_SMALL;
            width: 22;
            height: 11;
        }
    }
    focus edit_btn,
    @schema {
        color: BinaryColor
    }
}

impl<'a> Page for PresetPage<'a> {
    fn input(&mut self, control_panel: ControlsStateChanged) -> Result<PageEvent, PageError> {
        if let EncoderState::Changed(main_enc) = control_panel.main_enc {
            self.focus = self.focus + main_enc;
        }

        Ok(PageEvent::None)
    }
}

// impl Drawable for PresetPage {
//     type Color = BinaryColor;
//     type Output = ();

//     fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
//     where
//         D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
//     {
//         // self.edit_btn.draw(target);
//         // self.save_btn.d

//         Rectangle::new(Point::zero(), Size::zero())
//             .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
//         Ok(())
//     }
// }
