// use embassy_time::Instant;
use embedded_hal::digital::v2::InputPin;
use rotary_encoder_embedded::{
    angular_velocity::AngularVelocityMode, standard::StandardMode, RotaryEncoder,
};
use stm32f4xx_hal::gpio::{ExtiPin, PinExt};

const UPDATE_FREQUENCY: u32 = 10;
const VELOCITY_DEC_FREQUENCY: u32 = 10;

#[derive(Clone, Copy, Default, defmt::Format)]
pub enum EncoderState {
    #[default]
    None,
    Changed(i32),
}

pub struct Encoder<MODE, DT, CLK> {
    enc: RotaryEncoder<MODE, DT, CLK>,
    state: i32,
    last_update: u32,
}

impl<DT, CLK> Encoder<StandardMode, DT, CLK>
where
    DT: InputPin + PinExt,
    CLK: InputPin + ExtiPin,
{
    pub fn new_standard(dt: DT, clk: CLK) -> Self {
        Self {
            enc: RotaryEncoder::new(dt, clk).into_standard_mode(),
            state: 0,
            last_update: 0,
        }
    }

    pub fn pins_mut(&mut self) -> (&mut DT, &mut CLK) {
        self.enc.pins_mut()
    }

    pub fn tick(&mut self) -> EncoderState {
        self.enc.update();

        match self.enc.direction() {
            rotary_encoder_embedded::Direction::None => {}
            rotary_encoder_embedded::Direction::Clockwise => {
                self.state += 1;
            }
            rotary_encoder_embedded::Direction::Anticlockwise => {
                self.state -= 1;
            }
        }

        let state = self.state;
        self.state = 0;
        match state {
            0 => EncoderState::None,
            _ => EncoderState::Changed(state),
        }
    }
}

#[derive(Clone, Copy, Default, defmt::Format)]
pub enum AccelEncoderState {
    #[default]
    None,
    Changed((i32, f32)),
}

impl<DT, CLK> Encoder<AngularVelocityMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    pub fn new(dt: DT, clk: CLK) -> Self {
        let mut enc = RotaryEncoder::new(dt, clk).into_angular_velocity_mode();

        enc.set_velocity_action_ms(5);
        enc.set_velocity_dec_factor(0.01);
        enc.set_velocity_inc_factor(0.1);

        Self {
            enc,
            state: 0,
            last_update: 0,
        }
    }

    pub fn pins_mut(&mut self) -> (&mut DT, &mut CLK) {
        self.enc.pins_mut()
    }

    pub fn tick(&mut self, now_millis: u32) -> AccelEncoderState {
        let elapsed = now_millis - self.last_update;

        let mut dec_times = elapsed / VELOCITY_DEC_FREQUENCY;
        while dec_times > 0 {
            self.enc.decay_velocity();
            dec_times -= 1;
        }

        self.enc.update(now_millis as u64);

        match self.enc.direction() {
            rotary_encoder_embedded::Direction::None => {}
            rotary_encoder_embedded::Direction::Clockwise => {
                self.state += 1;
            }
            rotary_encoder_embedded::Direction::Anticlockwise => {
                self.state -= 1;
            }
        }

        if elapsed < UPDATE_FREQUENCY {
            return AccelEncoderState::None;
        }

        let state = self.state;

        self.state = 0;
        self.last_update = now_millis;

        match state {
            0 => AccelEncoderState::None,
            _ => AccelEncoderState::Changed((state, self.enc.velocity())),
        }
    }
}

// pub trait EncoderId {}

// pub struct RedEnc;
// impl EncoderId for RedEnc {}

// pub struct GreenEnc;
// impl EncoderId for GreenEnc {}

pub trait EditByEncoder {
    type Meta;

    fn edit_by_encoder(&mut self, offset: i32, vel: f32, meta: Self::Meta) -> &mut Self;
}
