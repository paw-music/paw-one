use embassy_stm32::{gpio::Pin, Peripheral};
use rotary_encoder_embedded::{angular_velocity::AngularVelocityMode, standard::StandardMode};

use self::enc::{AccelEncoderState, Encoder, EncoderState};

pub mod enc;

pub struct ControlsStateChanged {
    pub main_enc: EncoderState,
    pub red_enc: AccelEncoderState,
    pub green_enc: AccelEncoderState,
}

#[derive(Default)]
pub enum ControlsState {
    #[default]
    None,
    Changed(ControlsStateChanged),
}

#[derive(Default)]
pub struct ControlsStateBuilder {
    main_enc: EncoderState,
    red_enc: AccelEncoderState,
    green_enc: AccelEncoderState,
}

impl ControlsStateBuilder {
    fn build(self) -> ControlsState {
        match (self.main_enc, self.red_enc, self.green_enc) {
            (EncoderState::None, AccelEncoderState::None, AccelEncoderState::None) => {
                ControlsState::None
            }
            _ => ControlsState::Changed(ControlsStateChanged {
                main_enc: self.main_enc,
                red_enc: self.red_enc,
                green_enc: self.green_enc,
            }),
        }
    }
}

pub struct ControlPanel<'a> {
    main_enc: Encoder<'a, StandardMode>,
    red_enc: Encoder<'a, AngularVelocityMode>,
    green_enc: Encoder<'a, AngularVelocityMode>,
}

impl<'a> ControlPanel<'a> {
    pub fn new(
        main_enc: (
            impl Peripheral<P = impl Pin> + 'a,
            impl Peripheral<P = impl Pin> + 'a,
        ),
        red_enc: (
            impl Peripheral<P = impl Pin> + 'a,
            impl Peripheral<P = impl Pin> + 'a,
        ),
        green_enc: (
            impl Peripheral<P = impl Pin> + 'a,
            impl Peripheral<P = impl Pin> + 'a,
        ),
    ) -> Self {
        Self {
            main_enc: Encoder::new_standard(main_enc.0, main_enc.1),
            red_enc: Encoder::new(red_enc.0, red_enc.1),
            green_enc: Encoder::new(green_enc.0, green_enc.1),
        }
    }

    pub fn tick(&mut self) -> ControlsState {
        ControlsStateBuilder {
            main_enc: self.main_enc.tick(),
            red_enc: self.red_enc.tick(),
            green_enc: self.green_enc.tick(),
        }
        .build()
    }
}
