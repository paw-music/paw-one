use defmt::unwrap;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct TTP229<SCL: OutputPin, SDO: InputPin> {
    scl: SCL,
    sdo: SDO,
}

impl<SCL: OutputPin, SDO: InputPin> TTP229<SCL, SDO>
where
    SCL::Error: defmt::Format,
    SDO::Error: defmt::Format,
{
    pub fn new(pins: (SCL, SDO)) -> Self {
        let mut init = Self {
            scl: pins.0,
            sdo: pins.1,
        };

        unwrap!(init.scl.set_high());

        init
    }

    pub fn read(&mut self) -> u16 {
        let mut states = 0u16;
        for key_index in 0..16 {
            unwrap!(self.scl.set_low());
            states |= (unwrap!(self.sdo.is_low()) as u16) << key_index as u16;
            unwrap!(self.scl.set_high());
        }
        states
    }
}
