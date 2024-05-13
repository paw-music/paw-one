#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate paw_one;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    exti::ExtiInput,
    gpio::{AnyPin, Input, Level, Output, Pin, Speed},
    i2c,
    i2s::{self, I2S},
    peripherals::{self, SPI1},
    spi::{self, MckPin},
    time::Hertz,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Sender};
use embassy_time::{Duration, Instant, Timer};
use embassy_usb::class::cdc_acm::State;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    image::Image,
    mono_font::{ascii, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable},
    text::Text,
};
use embedded_layout::{layout::linear::LinearLayout, object_chain::Chain};
use embedded_text::TextBox;
use paw::audio::{osc::simple_form::SimpleFormSource, source::AudioSourceIter};
use paw_one::{
    board_info::DISPLAY_SIZE,
    control::{
        self,
        enc::{AccelEncoderState, EditByEncoder, EncoderState},
        ControlPanel, ControlsState,
    },
    heap::init_global_heap,
    sound::adsr::{ui::AdsrEdit, Adsr, AdsrCurveBend, AdsrStage, DurationSlope},
    ui::{
        kit::{button::Button, input::InputEl as _, select::SelectView},
        mono_icons::{icons_5x7::MonoIcons5x7, MonoIcons},
        page::{Page, PageFactory},
        text::{FONT_MEDIUM, FONT_SMALL},
        LOGO,
    },
};
use rotary_encoder_embedded::RotaryEncoder;
use ssd1306::{mode::DisplayConfig as _, prelude::Brightness};

use {defmt_rtt as _, panic_probe as _};

use embedded_graphics::Drawable as _;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

const SAMPLE_RATE: u32 = 48_000;

type ControlPanelChannel = embassy_sync::channel::Channel<ThreadModeRawMutex, ControlsState, 16>;
static CONTROL_PANEL_CHANNEL: ControlPanelChannel = embassy_sync::channel::Channel::new();

// #[derive(Clone, Copy)]
// struct Controls {
//     main_enc: i32,
// }

#[embassy_executor::task]
async fn status_led(pin: impl Pin) {
    let mut led = Output::new(pin, Level::High, Speed::Low);
    loop {
        info!("Okay");
        led.set_low();
        Timer::after_millis(1000).await;
        led.set_high();
        Timer::after_millis(10000).await;
    }
}

#[embassy_executor::task]
async fn control_panel(
    channel: Sender<'static, ThreadModeRawMutex, ControlsState, 16>,
    mut control_panel: ControlPanel<'static>,
) {
    loop {
        channel.send(control_panel.tick()).await;
        Timer::after_millis(1).await;
    }
}

// #[embassy_executor::task]
// async fn display(display: impl DrawTarget) {
//     loop {
//         let
//     }
// }

// #[embassy_executor::task]
// async fn status_led(pin: impl Pin) {}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Program entered");

    unsafe { init_global_heap() };
    {
        let mut vec = alloc::vec![1, 2, 3, 4, 5];
        vec.push(1);
        vec.pop();
        info!("HEAP Check with vector ran successfully");
    }

    let mut config = embassy_stm32::Config::default();

    {
        use embassy_stm32::rcc::*;

        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(25),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            // 16MHz HSI
            prediv: PllPreDiv::DIV25,
            mul: PllMul::MUL192,
            divp: Some(PllPDiv::DIV2),
            divq: Some(PllQDiv::DIV4),
            divr: Some(PllRDiv::DIV2),
        });
        config.rcc.plli2s = Some(Pll {
            prediv: PllPreDiv::DIV12,
            mul: PllMul::MUL50,
            divp: Some(PllPDiv::DIV2),
            divq: Some(PllQDiv::DIV4),
            divr: Some(PllRDiv::DIV2),
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2; // Must give <=50MHz
    }

    let p = embassy_stm32::init(config);

    info!(
        "SYS CLOCK FREQUENCY: {}",
        embassy_stm32::rcc::frequency::<peripherals::SYSCFG>()
    );

    spawner.spawn(status_led(p.PC13)).unwrap();
    spawner
        .spawn(control_panel(
            CONTROL_PANEL_CHANNEL.sender(),
            ControlPanel::new((p.PA9, p.PA8), (p.PA12, p.PA11), (p.PA1, p.PA0)),
        ))
        .unwrap();

    let mut display = {
        let display_i2c_cfg = i2c::Config::default();

        let display_i2c = i2c::I2c::new(
            p.I2C1,
            p.PB8,
            p.PB9,
            Irqs,
            p.DMA1_CH1,
            p.DMA1_CH5,
            // TODO: I think it only can do 400KHz
            Hertz::khz(400),
            display_i2c_cfg,
        );

        let di = ssd1306::I2CDisplayInterface::new(display_i2c);
        let mut display = ssd1306::Ssd1306::new(
            di,
            ssd1306::size::DisplaySize128x32,
            ssd1306::rotation::DisplayRotation::Rotate0,
        )
        .into_buffered_graphics_mode();
        display.init().unwrap();

        display.set_brightness(Brightness::NORMAL).unwrap();

        info!("Initialized SSD1306 display");

        display.clear(BinaryColor::Off).unwrap();

        Image::new(&LOGO, Point::zero()).draw(&mut display).unwrap();
        display.flush().unwrap();
        Timer::after_secs(1).await;

        display.clear(BinaryColor::Off).unwrap();
        display.flush().unwrap();

        display
    };

    // let mut spi3 = {
    //     let mut spi_cfg = spi::Config::default();
    //     let mut spi = spi::Spi::new(p.SPI3, p.PB3, p.PB5, p.PB4, p.DMA1_CH7, p.DMA1_CH2, spi_cfg);

    //     info!("Sending spi data");
    //     spi.write(&[1u16, 2, 3, 4, 5, 6, 7]).await.unwrap();
    // };

    // let mut i2s = {
    //     let mut i2s_config = i2s::Config::default();

    //     // i2s_config.mode = i2s::Mode::Master;
    //     // i2s_config.function = i2s::Function::Transmit;
    //     // i2s_config.standard = i2s::Standard::Philips;
    //     // i2s_config.format = i2s::Format::Data16Channel16;
    //     // i2s_config.clock_polarity = i2s::ClockPolarity::IdleLow;
    //     // i2s_config.master_clock = false;

    //     let i2s = I2S::new(
    //         p.SPI2,
    //         p.PB15,
    //         p.PB12,
    //         p.PB10,
    //         p.PA3,
    //         p.DMA1_CH4,
    //         p.DMA1_CH3,
    //         // Hertz::hz(SAMPLE_RATE),
    //         Hertz::mhz(1),
    //         i2s_config,
    //     );

    //     info!(
    //         "I2S clock frequency: {}",
    //         embassy_stm32::rcc::frequency::<peripherals::SPI2>()
    //     );

    //     i2s
    // };

    // let mut sound = SimpleFormSource::infinite_stereo(
    //     SAMPLE_RATE,
    //     paw::audio::osc::simple_form::WaveForm::Square,
    //     220.0,
    // );

    let adsr = Adsr {
        delay: Duration::from_secs(1).into(),
        attack: DurationSlope {
            duration: Duration::from_secs(1).into(),
            bend: AdsrCurveBend::new(0.0),
        },
        hold: Duration::from_secs(1).into(),
        decay: DurationSlope {
            duration: Duration::from_secs(1).into(),
            bend: AdsrCurveBend::new(0.0),
        },
        sustain: 1.0.into(),
        release: DurationSlope {
            duration: Duration::from_secs(1).into(),
            bend: AdsrCurveBend::new(0.0),
        },
    };

    // let mut adsr_edit = AdsrEdit {
    //     adsr,
    //     active: AdsrStage::Delay,
    // };

    let mut page = PageFactory.page(paw_one::ui::page::PageId::Preset);

    // let mut select = SelectView::new(
    //     &["abc", "bcd", "cde", "def", "efg"],
    //     0,
    //     Rectangle::new(Point::zero(), Size::new(42, 8)),
    //     MonoTextStyleBuilder::new()
    //         .font(&FONT_MEDIUM)
    //         .text_color(BinaryColor::On)
    //         .background_color(BinaryColor::Off)
    //         .build(),
    //     true,
    // );

    let btn1 = component!(Button {} @schema {color: BinaryColor});
    let btn2 = component!(Button {} @schema {color: BinaryColor});

    LinearLayout::horizontal(Chain::new(btn1).append(btn2));

    page.draw(&mut display).unwrap();
    display.flush().unwrap();

    info!("Starting main loop...");
    loop {
        // let mut buf = [0u16; 8];

        // for s in buf.iter_mut() {
        //     *s = (sound.next_sample() * i16::MAX as f32) as i16 as u16;
        // }

        // info!("Sending i2s data");
        // i2s.write(&buf).await.unwrap();

        if let ControlsState::Changed(state) = CONTROL_PANEL_CHANNEL.receive().await {
            info!("Changed");

            page.input(state).unwrap();
            page.draw(&mut display).unwrap();

            // if let EncoderState::Changed(mut main_encoder_state) = main_enc {
            //     if main_encoder_state < 0 {
            //         while main_encoder_state != 0 {
            //             main_encoder_state += 1;
            //             adsr_edit.active = adsr_edit.prev_stage();
            //         }
            //     } else {
            //         while main_encoder_state != 0 {
            //             main_encoder_state -= 1;
            //             adsr_edit.active = adsr_edit.next_stage();
            //         }
            //     }
            // }

            // if let AccelEncoderState::Changed((state, vel)) = red_enc {
            //     adsr_edit.edit_first_param(state, vel);
            // }

            // if let AccelEncoderState::Changed((state, vel)) = green_enc {
            //     adsr_edit.edit_second_param(state, vel);
            // }

            // // info!("ADSR: {}", adsr_edit);
            // adsr_edit.draw(&mut display).unwrap();

            display.flush().unwrap();
        }
    }
}
