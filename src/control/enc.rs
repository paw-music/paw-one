// use embassy_time::Instant;
use embedded_hal::digital::v2::InputPin;
use rotary_encoder_embedded::{
    angular_velocity::AngularVelocityMode, standard::StandardMode, RotaryEncoder,
};

const UPDATE_FREQUENCY: u32 = 10;
const VELOCITY_DEC_FREQUENCY: u32 = 10;

#[derive(Clone, Copy, Default, defmt::Format)]
pub enum EncState {
    #[default]
    None,
    Changed(i32),
}

pub struct Enc<MODE, DT, CLK> {
    enc: RotaryEncoder<MODE, DT, CLK>,
    state: i32,
    last_update: u32,
}

impl<DT, CLK> Enc<StandardMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
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

    pub fn tick(&mut self, now_millis: u32) -> EncState {
        // if now_millis - self.last_update <  {
        //     return EncoderState::None;
        // }

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
        self.last_update = now_millis;
        match state {
            0 => EncState::None,
            _ => EncState::Changed(state),
        }
    }
}

#[derive(Clone, Copy, Default, defmt::Format)]
pub enum AccelEncState {
    #[default]
    None,
    Changed((i32, f32)),
}

impl<DT, CLK> Enc<AngularVelocityMode, DT, CLK>
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

    pub fn tick(&mut self, now_millis: u32) -> AccelEncState {
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
            return AccelEncState::None;
        }

        let state = self.state;

        self.state = 0;
        self.last_update = now_millis;

        match state {
            0 => AccelEncState::None,
            _ => AccelEncState::Changed((state, self.enc.velocity())),
        }
    }
}
