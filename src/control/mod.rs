use embedded_hal::digital::v2::InputPin;
use rotary_encoder_embedded::{angular_velocity::AngularVelocityMode, standard::StandardMode};

use self::enc::{AccelEncoderState, Encoder, EncoderState};

pub mod enc;

#[derive(defmt::Format)]
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

// pub trait ControlPanelPins {
//     type MainEncDt: InputPin;
//     type MainEncClk: InputPin;

//     type RedEncDt: InputPin;
//     type RedEncClk: InputPin;

//     type GreenEncDt: InputPin;
//     type GreenEncClk: InputPin;
// }

pub struct ControlPanel<
    MainEncDt: InputPin,
    MainEncClk: InputPin,
    RedEncDt: InputPin,
    RedEncClk: InputPin,
    GreenEncDt: InputPin,
    GreenEncClk: InputPin,
> {
    main_enc: Encoder<StandardMode, MainEncDt, MainEncClk>,
    red_enc: Encoder<AngularVelocityMode, RedEncDt, RedEncClk>,
    green_enc: Encoder<AngularVelocityMode, GreenEncDt, GreenEncClk>,
}

impl<
        MainEncDt: InputPin,
        MainEncClk: InputPin,
        RedEncDt: InputPin,
        RedEncClk: InputPin,
        GreenEncDt: InputPin,
        GreenEncClk: InputPin,
    > ControlPanel<MainEncDt, MainEncClk, RedEncDt, RedEncClk, GreenEncDt, GreenEncClk>
{
    pub fn new(
        main_enc: (MainEncDt, MainEncClk),
        red_enc: (RedEncDt, RedEncClk),
        green_enc: (GreenEncDt, GreenEncClk),
    ) -> Self {
        Self {
            main_enc: Encoder::new_standard(main_enc.0, main_enc.1),
            red_enc: Encoder::new(red_enc.0, red_enc.1),
            green_enc: Encoder::new(green_enc.0, green_enc.1),
        }
    }

    pub fn tick(&mut self, now_millis: u64) -> ControlsState {
        ControlsStateBuilder {
            main_enc: self.main_enc.tick(),
            red_enc: self.red_enc.tick(now_millis),
            green_enc: self.green_enc.tick(now_millis),
        }
        .build()
    }
}
