use debouncr::{
    debounce_2, debounce_stateful_2, debounce_stateful_3, Debouncer, DebouncerStateful, Repeat2,
    Repeat3,
};
use defmt::{debug, unwrap};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal_1::delay::DelayNs;
use stm32f4xx_hal::timer::Delay;

use crate::iter::digits::{Digits, Edge};

const DEFAULT_DELAY_US: u32 = 1_000;
const MAX_FREQ: u32 = 400_000;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, defmt::Format)]
pub struct Keys(u16);

impl Keys {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn is_active(&self, index: u16) -> bool {
        assert!(index <= 15);
        self.0 & (1 << index) == 1
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn iter_active(&self) -> impl Iterator<Item = usize> {
        self.digits(2)
            .enumerate()
            .filter_map(|(key_index, key)| (key == 1).then_some(key_index))
    }

    pub fn get(&self, index: u16) -> Option<bool> {
        if index > 15 {
            None
        } else {
            Some(self.0 & (1 << index) == 1)
        }
    }
}

impl Digits for Keys {
    type Int = u16;

    fn digits(self, base: u16) -> crate::iter::digits::DigitsIter<u16> {
        self.0.digits(base)
    }
}

pub struct TTP229<SCL: OutputPin, SDO: InputPin> {
    scl: SCL,
    sdo: SDO,
    delay_us: u32,
    debouncers: [DebouncerStateful<u8, Repeat3>; 16],
}

impl<SCL: OutputPin, SDO: InputPin> TTP229<SCL, SDO>
where
    SCL::Error: defmt::Format,
    SDO::Error: defmt::Format,
{
    pub fn new(pins: (SCL, SDO)) -> Self {
        let init = Self {
            scl: pins.0,
            sdo: pins.1,
            delay_us: DEFAULT_DELAY_US,
            debouncers: core::array::from_fn(|_| debounce_stateful_3(false)),
        };

        init
    }

    pub fn freq(mut self, freq: u32) -> Self {
        let freq = freq.min(MAX_FREQ);
        // Set delay in micros rounding up
        self.delay_us = (1_000_000 - 1) / freq + 1;
        self
    }

    pub fn read<TIM: stm32f4xx_hal::timer::Instance, const FREQ: u32>(
        &mut self,
        delay: &mut Delay<TIM, FREQ>,
    ) -> Keys {
        let mut states = 0u16;

        unwrap!(self.scl.set_high());
        delay.delay_us(self.delay_us);

        for key_index in 0..16 {
            unwrap!(self.scl.set_low());
            delay.delay_us(self.delay_us);
            states |= (unwrap!(self.sdo.is_low()) as u16) << key_index as u16;
            unwrap!(self.scl.set_high());
            delay.delay_us(self.delay_us);
        }

        delay.delay_us(self.delay_us);

        Keys(states)
    }

    pub fn edges<TIM: stm32f4xx_hal::timer::Instance, const FREQ: u32>(
        &mut self,
        delay: &mut Delay<TIM, FREQ>,
    ) -> [Option<Edge>; 16] {
        let states = self.read(delay);
        let mut edges = [None; 16];
        for key_index in 0..16 {
            edges[key_index] = self.debouncers[key_index]
                .update(states.get(key_index as u16).unwrap())
                .map(Into::into);
        }

        edges
    }
}
