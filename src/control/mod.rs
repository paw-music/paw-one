use alloc::vec::Vec;
use defmt::info;
use embedded_hal::digital::v2::InputPin;
use rotary_encoder_embedded::{angular_velocity::AngularVelocityMode, standard::StandardMode};
use stm32f4xx_hal::{
    gpio::{ExtiPin, Pin, PinExt},
    interrupt,
    pac::{EXTI, NVIC},
    syscfg::SysCfg,
};

use crate::ui::Event;

use self::enc::{AccelEncoderState, Encoder, EncoderState};

pub mod enc;

#[derive(defmt::Format)]
pub struct ControlsStateChanged {
    pub main_enc: EncoderState,
    pub red_enc: AccelEncoderState,
    pub green_enc: AccelEncoderState,
}

impl ControlsStateChanged {
    pub fn into_events(self) -> Vec<Event> {
        let mut events = vec![];

        if let EncoderState::Changed(main_enc) = self.main_enc {
            events.push(Event::MainEncChange(main_enc, 1.0));
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
        MainEncDt: InputPin + PinExt + ExtiPin,
        MainEncClk: InputPin + PinExt + ExtiPin,
        RedEncDt: InputPin + PinExt + ExtiPin,
        RedEncClk: InputPin + PinExt + ExtiPin,
        GreenEncDt: InputPin + PinExt + ExtiPin,
        GreenEncClk: InputPin + PinExt + ExtiPin,
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

    // pub fn tick(&mut self, now_millis: u32) -> ControlsState {
    //     ControlsStateBuilder {
    //         main_enc: self.main_enc.tick(),
    //         red_enc: self.red_enc.tick(now_millis),
    //         green_enc: self.green_enc.tick(now_millis),
    //     }
    //     .build()
    // }

    // pub fn configure_interrupts(&mut self, syscfg: &mut SysCfg, exti: &mut EXTI) {
    //     Self::configure_pin_interrupts(self.main_enc.pins_mut().0, syscfg, exti);
    //     Self::configure_pin_interrupts(self.main_enc.pins_mut().1, syscfg, exti);
    //     Self::configure_pin_interrupts(self.red_enc.pins_mut().0, syscfg, exti);
    //     Self::configure_pin_interrupts(self.green_enc.pins_mut().0, syscfg, exti);
    // }

    // fn configure_pin_interrupts<P: ExtiPin + PinExt>(
    //     pin: &mut P,
    //     syscfg: &mut SysCfg,
    //     exti: &mut EXTI,
    // ) {
    //     pin.make_interrupt_source(syscfg);
    //     pin.enable_interrupt(exti);
    //     pin.trigger_on_edge(exti, stm32f4xx_hal::gpio::Edge::Falling);

    //     // Note: Copy of `stm32f4xx_hal::Pin::interrupt`
    //     let pin_int = match pin.pin_id() {
    //         0 => interrupt::EXTI0,
    //         1 => interrupt::EXTI1,
    //         2 => interrupt::EXTI2,
    //         3 => interrupt::EXTI3,
    //         4 => interrupt::EXTI4,
    //         5..=9 => interrupt::EXTI9_5,
    //         10..=15 => interrupt::EXTI15_10,
    //         _ => panic!("Unsupported pin number"),
    //     };

    //     unsafe {
    //         NVIC::unmask(pin_int);
    //     }
    // }

    // pub fn handle_exti(&mut self) {
    //     let (main_enc_dt, main_enc_clk) = self.main_enc.pins_mut();
    //     let (red_enc_dt, red_enc_clk) = self.red_enc.pins_mut();
    //     let (green_enc_dt, green_enc_clk) = self.green_enc.pins_mut();

    //     main_enc_dt.clear_interrupt_pending_bit();
    //     main_enc_clk.clear_interrupt_pending_bit();
    //     red_enc_dt.clear_interrupt_pending_bit();
    //     red_enc_clk.clear_interrupt_pending_bit();
    //     green_enc_dt.clear_interrupt_pending_bit();
    //     green_enc_clk.clear_interrupt_pending_bit();

    //     info!("Control panel interrupt");
    // }
}
