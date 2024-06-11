use alloc::vec::Vec;
use btn::{Btn, BtnState, PullEdge};
use embedded_hal::digital::v2::InputPin;
use rotary_encoder_embedded::{angular_velocity::AngularVelocityMode, standard::StandardMode};

use crate::ui::Event;

use self::enc::{AccelEncoderState, Encoder, EncoderState};

pub mod btn;
pub mod enc;

#[derive(defmt::Format)]
pub struct ControlsStateChanged {
    pub main_enc: EncoderState,
    pub main_enc_btn: BtnState,
    pub red_enc: AccelEncoderState,
    pub green_enc: AccelEncoderState,
}

impl ControlsStateChanged {
    pub fn into_events(self) -> Vec<Event> {
        let mut events = vec![];

        if let EncoderState::Changed(main_enc) = self.main_enc {
            events.push(Event::MainEncChange(main_enc, 1.0));
        }

        if let BtnState::Up = self.main_enc_btn {
            events.push(Event::MainEncClickUp);
        }

        if let BtnState::Down = self.main_enc_btn {
            events.push(Event::MainEncClickDown);
        }

        events
    }
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
    main_enc_btn: BtnState,
    red_enc: AccelEncoderState,
    green_enc: AccelEncoderState,
}

impl ControlsStateBuilder {
    fn build(self) -> ControlsState {
        match (
            self.main_enc,
            self.main_enc_btn,
            self.red_enc,
            self.green_enc,
        ) {
            (
                EncoderState::None,
                BtnState::None,
                AccelEncoderState::None,
                AccelEncoderState::None,
            ) => ControlsState::None,
            _ => ControlsState::Changed(ControlsStateChanged {
                main_enc: self.main_enc,
                main_enc_btn: self.main_enc_btn,
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
    MainEncBtn: InputPin,
    MainEncBtnPull: PullEdge,
    RedEncDt: InputPin,
    RedEncClk: InputPin,
    GreenEncDt: InputPin,
    GreenEncClk: InputPin,
> {
    main_enc: Encoder<StandardMode, MainEncDt, MainEncClk>,
    main_enc_btn: Btn<MainEncBtn, MainEncBtnPull>,
    red_enc: Encoder<AngularVelocityMode, RedEncDt, RedEncClk>,
    green_enc: Encoder<AngularVelocityMode, GreenEncDt, GreenEncClk>,
}

impl<
        MainEncDt: InputPin,
        MainEncClk: InputPin,
        MainEncBtn: InputPin,
        MainEncBtnPull: PullEdge,
        RedEncDt: InputPin,
        RedEncClk: InputPin,
        GreenEncDt: InputPin,
        GreenEncClk: InputPin,
    >
    ControlPanel<
        MainEncDt,
        MainEncClk,
        MainEncBtn,
        MainEncBtnPull,
        RedEncDt,
        RedEncClk,
        GreenEncDt,
        GreenEncClk,
    >
{
    pub fn new(
        main_enc: (MainEncDt, MainEncClk),
        main_enc_btn: (MainEncBtn, MainEncBtnPull),
        red_enc: (RedEncDt, RedEncClk),
        green_enc: (GreenEncDt, GreenEncClk),
    ) -> Self {
        Self {
            main_enc: Encoder::new_standard(main_enc.0, main_enc.1),
            main_enc_btn: Btn::new(main_enc_btn.0, main_enc_btn.1),
            red_enc: Encoder::new(red_enc.0, red_enc.1),
            green_enc: Encoder::new(green_enc.0, green_enc.1),
        }
    }

    pub fn tick(&mut self, now_millis: u32) -> ControlsState {
        ControlsStateBuilder {
            main_enc: self.main_enc.tick(),
            main_enc_btn: self.main_enc_btn.tick(),
            red_enc: self.red_enc.tick(now_millis),
            green_enc: self.green_enc.tick(now_millis),
        }
        .build()
    }
}
