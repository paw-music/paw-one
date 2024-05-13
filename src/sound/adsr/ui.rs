use alloc::vec::Vec;
use defmt::debug;
use embassy_time::Duration;
use embedded_graphics::{
    geometry::{Point, Size},
    mono_font::{ascii::FONT_4X6, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, PixelColor},
    primitives::{
        Circle, Line, Polyline, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable,
    },
    text::renderer::CharacterStyle,
    Drawable, Pixel,
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*};
use embedded_text::TextBox;
use micromath::F32Ext;

use crate::{
    board_info::DISPLAY_SIZE,
    control::enc::EditByEncoder,
    ui::{
        text::{FONT_MEDIUM, FONT_SMALL},
        ToHeaplessString,
    },
};
use core::{
    fmt::{Display, Write},
    write,
};

use super::{Adsr, AdsrDuration, AdsrStage, AdsrStageValue, AdsrStages, DurationSlope};

const HALF_HEIGHT: u32 = DISPLAY_SIZE.height / 2;

const CHAR_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new()
    .background_color(BinaryColor::Off)
    .text_color(BinaryColor::On)
    .font(&FONT_MEDIUM)
    .build();

const CURVE_STYLE: PrimitiveStyle<BinaryColor> = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(BinaryColor::On)
    .build();

#[derive(Clone, Copy)]
struct AdsrStageUi {
    prev_stage_end: Point,
    amps: AdsrStages<f32>,
    width: u32,
    stage: AdsrStage,
    value: AdsrStageValue,
    active: bool,
    // Note: Curve needs to be redrawn only if itself or its siblings changed
    need_curve_redraw: bool,
}

impl Drawable for AdsrStageUi {
    type Color = BinaryColor;
    type Output = Point;

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        // TODO: Show curvature value when editing for some time, then switch to duration
        let mut text = heapless::String::<10>::new();
        write!(
            text,
            "{}{}{}\n{}",
            if self.active { "[" } else { "" },
            match self.stage {
                AdsrStage::Delay => "D",
                AdsrStage::Attack => "A",
                AdsrStage::Hold => "H",
                AdsrStage::Decay => "D",
                AdsrStage::Sustain => "S",
                AdsrStage::Release => "R",
            },
            if self.active { "]" } else { "" },
            self.value
        )
        .unwrap();

        let x_offset = self.prev_stage_end.x + 1;

        let text_bounds =
            Rectangle::new(Point::new(x_offset, 0), Size::new(self.width, HALF_HEIGHT));

        text_bounds.draw_styled(
            &PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::Off)
                .build(),
            target,
        )?;

        TextBox::with_alignment(
            &text,
            text_bounds,
            CHAR_STYLE,
            embedded_text::alignment::HorizontalAlignment::Center,
        )
        .draw(target)?;

        // TODO: Curvature

        // if self.need_curve_redraw {

        let prev_amp = match self.stage {
            AdsrStage::Delay => 0.0,
            AdsrStage::Attack => self.amps.delay,
            AdsrStage::Hold => self.amps.attack,
            AdsrStage::Decay => self.amps.hold,
            AdsrStage::Sustain => self.amps.decay,
            AdsrStage::Release => self.amps.sustain,
        };

        let prev_amp_y =
            DISPLAY_SIZE.height as i32 - (prev_amp * HALF_HEIGHT as f32).floor() as i32 - 1;

        let amp = self.amps.stage(self.stage);

        let amp_y = DISPLAY_SIZE.height as i32 - (amp * HALF_HEIGHT as f32).floor() as i32 - 1;

        Rectangle::new(
            Point::new(x_offset, HALF_HEIGHT as i32),
            Size::new(self.width, HALF_HEIGHT),
        )
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::Off)
                .build(),
            target,
        )?;

        match self.value {
            AdsrStageValue::Duration(_) | AdsrStageValue::Amplitude(_) => {
                Line::new(
                    self.prev_stage_end,
                    Point::new(x_offset + self.width as i32, amp_y),
                )
                .draw_styled(&CURVE_STYLE, target)?;
            }
            AdsrStageValue::DurationSlope(DurationSlope { bend, duration: _ }) => {
                let vertices = (0..self.width)
                    .map(|x| {
                        let x_norm = x as f32 / self.width as f32;
                        let y =
                            (bend.fun(x_norm) * (amp_y as f32 - prev_amp_y as f32 - 1.0)) as i32;
                        Point::new(self.prev_stage_end.x + x as i32, y + prev_amp_y)
                    })
                    .collect::<Vec<_>>();

                Polyline::new(&vertices).draw_styled(
                    &PrimitiveStyleBuilder::new()
                        .stroke_color(BinaryColor::On)
                        .stroke_width(1)
                        .build(),
                    target,
                )?;
            }
        }

        Ok(Point::new(self.width as i32 + self.prev_stage_end.x, amp_y))
    }
}

#[derive(defmt::Format)]
pub struct AdsrEdit {
    pub adsr: Adsr,
    pub active: AdsrStage,
}

impl AdsrEdit {
    pub fn prev_stage(&self) -> AdsrStage {
        match self.active {
            AdsrStage::Delay => AdsrStage::Release,
            AdsrStage::Attack => AdsrStage::Delay,
            AdsrStage::Hold => AdsrStage::Attack,
            AdsrStage::Decay => AdsrStage::Hold,
            AdsrStage::Sustain => AdsrStage::Decay,
            AdsrStage::Release => AdsrStage::Sustain,
        }
    }

    pub fn next_stage(&self) -> AdsrStage {
        match self.active {
            AdsrStage::Delay => AdsrStage::Attack,
            AdsrStage::Attack => AdsrStage::Hold,
            AdsrStage::Hold => AdsrStage::Decay,
            AdsrStage::Decay => AdsrStage::Sustain,
            AdsrStage::Sustain => AdsrStage::Release,
            AdsrStage::Release => AdsrStage::Delay,
        }
    }

    pub fn edit_first_param(&mut self, offset: i32, vel: f32) {
        match self.active {
            AdsrStage::Delay => {
                self.adsr
                    .delay
                    .edit_by_encoder(offset, vel, AdsrStage::Delay);
            }
            AdsrStage::Attack => {
                self.adsr
                    .attack
                    .duration
                    .edit_by_encoder(offset, vel, AdsrStage::Attack);
            }
            AdsrStage::Hold => {
                self.adsr.hold.edit_by_encoder(offset, vel, AdsrStage::Hold);
            }
            AdsrStage::Decay => {
                self.adsr
                    .decay
                    .duration
                    .edit_by_encoder(offset, vel, AdsrStage::Decay);
            }
            AdsrStage::Sustain => {
                self.adsr.sustain.edit_by_encoder(offset, vel, ());
            }
            AdsrStage::Release => {
                self.adsr
                    .release
                    .duration
                    .edit_by_encoder(offset, vel, AdsrStage::Release);
            }
        }
    }

    pub fn edit_second_param(&mut self, offset: i32, vel: f32) {
        match self.active {
            AdsrStage::Delay => {
                debug!("ADSR: No second param editing for DELAY");
            }
            AdsrStage::Attack => {
                self.adsr.attack.bend.edit_by_encoder(offset, vel, ());
            }
            AdsrStage::Hold => {
                debug!("ADSR: No second param editing for HOLD");
            }
            AdsrStage::Decay => {
                self.adsr.decay.bend.edit_by_encoder(offset, vel, ());
            }
            AdsrStage::Sustain => {
                debug!("ADSR: No second param editing for SUSTAIN");
            }
            AdsrStage::Release => {
                self.adsr.release.bend.edit_by_encoder(offset, vel, ());
            }
        }
    }
}

impl Drawable for AdsrEdit {
    type Color = BinaryColor;

    // TODO: Project draw results may be useful?
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        // LinearLayout::horizontal(Chain::new(TextBox::with_alignment(
        //     &self.delay.to_heapless_string::<5>(),
        //     Rectangle::new(Point::zero(), Size::new(D, height)),
        //     character_style,
        //     alignment,
        // )));

        // TODO: Draw all on first or remote changes

        // The point each stage's curve approaches
        // Delay is always 0
        let amps = AdsrStages {
            delay: 0.0,
            attack: 1.0,
            hold: 1.0,
            decay: self.adsr.sustain.0,
            sustain: self.adsr.sustain.0,
            release: 0.0,
        };

        let prev_stage_end = AdsrStageUi {
            prev_stage_end: Point::new(0, DISPLAY_SIZE.height as i32 - 1),
            amps,
            width: 20,
            stage: AdsrStage::Delay,
            value: self.adsr.delay.into(),
            active: self.active == AdsrStage::Delay,
            need_curve_redraw: matches!(self.active, AdsrStage::Delay | AdsrStage::Attack),
        }
        .draw(target)?;

        let prev_stage_end = AdsrStageUi {
            prev_stage_end,
            amps,
            width: 22,
            stage: AdsrStage::Attack,
            value: self.adsr.attack.into(),
            active: self.active == AdsrStage::Attack,
            need_curve_redraw: matches!(
                self.active,
                AdsrStage::Delay | AdsrStage::Attack | AdsrStage::Hold
            ),
        }
        .draw(target)?;

        let prev_stage_end = AdsrStageUi {
            prev_stage_end,
            amps,
            width: 22,
            stage: AdsrStage::Hold,
            value: self.adsr.hold.into(),
            active: self.active == AdsrStage::Hold,
            need_curve_redraw: matches!(
                self.active,
                AdsrStage::Attack | AdsrStage::Hold | AdsrStage::Decay
            ),
        }
        .draw(target)?;

        let prev_stage_end = AdsrStageUi {
            prev_stage_end,
            amps,
            width: 22,
            stage: AdsrStage::Decay,
            value: self.adsr.decay.into(),
            active: self.active == AdsrStage::Decay,
            need_curve_redraw: matches!(
                self.active,
                AdsrStage::Hold | AdsrStage::Decay | AdsrStage::Sustain
            ),
        }
        .draw(target)?;

        let prev_stage_end = AdsrStageUi {
            prev_stage_end,
            amps,
            width: 20,
            stage: AdsrStage::Sustain,
            value: self.adsr.sustain.into(),
            active: self.active == AdsrStage::Sustain,
            need_curve_redraw: matches!(
                self.active,
                AdsrStage::Decay | AdsrStage::Sustain | AdsrStage::Release
            ),
        }
        .draw(target)?;

        AdsrStageUi {
            prev_stage_end,
            amps,
            width: 22,
            stage: AdsrStage::Release,
            value: self.adsr.release.into(),
            active: self.active == AdsrStage::Release,
            need_curve_redraw: matches!(self.active, AdsrStage::Sustain | AdsrStage::Release),
        }
        .draw(target)?;

        Ok(())
    }
}
