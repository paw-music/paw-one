use alloc::vec::Vec;
use btn::{AnyBtn, Btn, BtnState, PullEdge};
use embedded_hal::{digital::v2::InputPin, Qei};
use qei_enc::{QeiEnc, QeiEncTimExt};
use rotary_encoder_embedded::{angular_velocity::AngularVelocityMode, standard::StandardMode};

use crate::ui::Event;

use self::enc::EncState;

pub mod btn;
pub mod enc;
pub mod qei_enc;

#[derive(defmt::Format)]
pub struct ControlsStateChanged {
    pub main_enc: EncState,
    pub main_enc_btn: BtnState,
    pub red_enc: EncState,
    pub green_enc: EncState,
}

impl ControlsStateChanged {
    pub fn into_events(self) -> Vec<Event> {
        let mut events = vec![];

        if let EncState::Changed(main_enc) = self.main_enc {
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
    main_enc: EncState,
    main_enc_btn: BtnState,
    red_enc: EncState,
    green_enc: EncState,
}

impl ControlsStateBuilder {
    fn build(self) -> ControlsState {
        match (
            self.main_enc,
            self.main_enc_btn,
            self.red_enc,
            self.green_enc,
        ) {
            (EncState::None, BtnState::None, EncState::None, EncState::None) => ControlsState::None,
            _ => ControlsState::Changed(ControlsStateChanged {
                main_enc: self.main_enc,
                main_enc_btn: self.main_enc_btn,
                red_enc: self.red_enc,
                green_enc: self.green_enc,
            }),
        }
    }
}

pub struct ControlPanel<
    MainEnc: QeiEncTimExt,
    MainEncBtn: AnyBtn,
    RedEnc: QeiEncTimExt,
    GreenEnc: QeiEncTimExt,
> {
    main_enc: QeiEnc<MainEnc>,
    main_enc_btn: MainEncBtn,
    red_enc: QeiEnc<RedEnc>,
    green_enc: QeiEnc<GreenEnc>,
}

impl<MainEnc: QeiEncTimExt, MainEncBtn: AnyBtn, RedEnc: QeiEncTimExt, GreenEnc: QeiEncTimExt>
    ControlPanel<MainEnc, MainEncBtn, RedEnc, GreenEnc>
{
    pub fn new(
        main_enc: QeiEnc<MainEnc>,
        main_enc_btn: MainEncBtn,
        red_enc: QeiEnc<RedEnc>,
        green_enc: QeiEnc<GreenEnc>,
    ) -> Self {
        Self {
            main_enc,
            main_enc_btn,
            red_enc,
            green_enc,
        }
    }

    pub fn tick(&mut self, now_millis: u32) -> ControlsState {
        ControlsStateBuilder {
            main_enc: self.main_enc.tick(now_millis),
            main_enc_btn: self.main_enc_btn.tick(),
            red_enc: self.red_enc.tick(now_millis),
            green_enc: self.green_enc.tick(now_millis),
        }
        .build()
    }
}
