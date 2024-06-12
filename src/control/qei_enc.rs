use core::marker::PhantomData;

use stm32f4xx_hal::{gpio::PushPull, timer::CPin};

use super::enc::EncState;
use stm32f4xx_hal::pac;

/// My encoder setup applied after `stm32f4xx_hal::qei::setup_qei`
pub trait QeiEncTimExt: stm32f4xx_hal::qei::Instance + Sized
// where
// stm32f4xx_hal::qei::Qei<Self>: embedded_hal::Qei<Count = u16>,
// Self::WI
{
    fn qei_enc_tim_setup();
}

macro_rules! impl_qei_enc_tim_ext {
    ($($tim: ty),*) => {
        $(impl QeiEncTimExt for $tim {
            fn qei_enc_tim_setup() {
                unsafe {
                    (*Self::ptr()).smcr.write(|w| w.sms().encoder_mode_1());
                }
            }
        })*
    };
}

// Don't implement for 32-bit timers, we assume usage of u16
impl_qei_enc_tim_ext!(
    pac::TIM1,
    // pac::TIM2,
    pac::TIM3,
    pac::TIM4,
    // pac::TIM5,
    pac::TIM8
);

pub enum Dir {
    Original,
    Inverted,
}

impl Dir {
    fn factor(&self) -> i32 {
        match self {
            Dir::Original => 1,
            Dir::Inverted => -1,
        }
    }

    fn inverted(self) -> Self {
        match self {
            Dir::Original => Dir::Inverted,
            Dir::Inverted => Dir::Original,
        }
    }
}

pub struct QeiEnc<TIM: QeiEncTimExt> {
    qei: stm32f4xx_hal::qei::Qei<TIM>,
    prev_count: u16,
    dir: Dir,
}

impl<TIM: QeiEncTimExt> QeiEnc<TIM> {
    pub fn new(
        tim: TIM,
        pins: (
            impl Into<<TIM as CPin<0>>::Ch<PushPull>>,
            impl Into<<TIM as CPin<1>>::Ch<PushPull>>,
        ),
    ) -> Self {
        let qei = stm32f4xx_hal::qei::Qei::new(tim, pins);
        TIM::qei_enc_tim_setup();

        let prev_count = embedded_hal::Qei::count(&qei).into() as u16;
        Self {
            qei,
            prev_count,
            dir: Dir::Original,
        }
    }

    pub fn inverted(mut self) -> Self {
        self.dir = self.dir.inverted();
        self
    }

    // Thanks to https://github.com/tonarino/panel-firmware/blob/main/src%2Fcounter.rs
    pub fn tick(&mut self, _now_millis: u32) -> EncState {
        let count = embedded_hal::Qei::count(&self.qei).into() as u16;
        let diff = count.wrapping_sub(self.prev_count) as i16 as i32;

        if diff.abs() >= 2 {
            // let offset = count as i32 - self.prev_count as i32;

            self.prev_count = count;

            let offset = diff / 2 * self.dir.factor();

            EncState::Changed(offset)
        } else {
            EncState::None
        }
    }
}
