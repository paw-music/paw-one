use defmt::info;
use embassy_stm32::{
    gpio::{Input, Pin},
    time::Hertz,
    Peripheral,
};
use embassy_time::Instant;
use rotary_encoder_embedded::{
    angular_velocity::AngularVelocityMode, standard::StandardMode, RotaryEncoder,
};

const UPDATE_FREQUENCY: u64 = 10;
const VELOCITY_DEC_FREQUENCY: u64 = 10;

#[derive(Clone, Copy, Default)]
pub enum EncoderState {
    #[default]
    None,
    Changed(i32),
}

pub struct Encoder<'a, MODE> {
    enc: RotaryEncoder<MODE, Input<'a>, Input<'a>>,
    state: i32,
    last_update: Instant,
}

impl<'a> Encoder<'a, StandardMode> {
    pub fn new_standard(
        dt: impl Peripheral<P = impl Pin> + 'a,
        clk: impl Peripheral<P = impl Pin> + 'a,
    ) -> Self {
        Self {
            enc: RotaryEncoder::new(
                Input::new(dt, embassy_stm32::gpio::Pull::Up),
                Input::new(clk, embassy_stm32::gpio::Pull::Up),
            )
            .into_standard_mode(),
            state: 0,
            last_update: Instant::now(),
        }
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

#[derive(Clone, Copy, Default)]
pub enum AccelEncoderState {
    #[default]
    None,
    Changed((i32, f32)),
}

impl<'a> Encoder<'a, AngularVelocityMode> {
    pub fn new(
        dt: impl Peripheral<P = impl Pin> + 'a,
        clk: impl Peripheral<P = impl Pin> + 'a,
    ) -> Self {
        let mut enc = RotaryEncoder::new(
            Input::new(dt, embassy_stm32::gpio::Pull::Up),
            Input::new(clk, embassy_stm32::gpio::Pull::Up),
        )
        .into_angular_velocity_mode();

        enc.set_velocity_action_ms(5);
        enc.set_velocity_dec_factor(0.01);
        enc.set_velocity_inc_factor(0.1);

        Self {
            enc,
            state: 0,
            last_update: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> AccelEncoderState {
        let now = Instant::now();

        let elapsed = self.last_update.elapsed().as_millis();

        let mut dec_times = elapsed / VELOCITY_DEC_FREQUENCY;
        while dec_times > 0 {
            self.enc.decay_velocity();
            dec_times -= 1;
        }

        self.enc.update(now.as_millis());

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
        self.last_update = now;

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
