#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate paw_one;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Pin, Speed},
    i2c,
    i2s::{self, I2S},
    peripherals,
    spi::{CkPin, MckPin, MosiPin, WsPin},
    time::Hertz,
    Peripheral,
};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Receiver, Sender},
};
use embassy_time::Timer;
use embedded_graphics::{
    draw_target::DrawTarget, geometry::Point, image::Image, pixelcolor::BinaryColor, Drawable,
};
use paw::audio::{osc::simple_form::SimpleFormSource, source::AudioSourceIter};
use paw_one::{
    control::{ControlPanel, ControlsState},
    heap::init_global_heap,
    ui::logo::LOGO,
};
use ssd1306::{mode::DisplayConfig as _, prelude::Brightness};

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

const SAMPLE_RATE: u32 = 48_000;

// type ControlPanelChannel = embassy_sync::channel::Channel<ThreadModeRawMutex, ControlsState, 16>;
// static CONTROL_PANEL_CHANNEL: ControlPanelChannel = embassy_sync::channel::Channel::new();

const SOUND_BUFFER_SIZE: usize = 1000;
type SoundBuffer = [u16; SOUND_BUFFER_SIZE];
type SoundChannel = embassy_sync::channel::Channel<ThreadModeRawMutex, SoundBuffer, 8>;
static SOUND_CHANNEL: SoundChannel = embassy_sync::channel::Channel::new();

#[embassy_executor::task]
async fn status_led(pin: impl Pin) {
    info!("Status LED running...");

    let mut led = Output::new(pin, Level::High, Speed::Low);
    loop {
        info!("Okay");
        led.set_low();
        Timer::after_millis(10000).await;
        led.set_high();
        Timer::after_millis(1000).await;
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

#[embassy_executor::task]
async fn playback(
    channel: Receiver<'static, ThreadModeRawMutex, SoundBuffer, 8>,
    mut i2s: I2S<'static>,
) {
    loop {
        let buf = channel.receive().await;
        i2s.write(&buf).await.unwrap();
        Timer::after_micros(10).await;
        info!("Sent to i2s");
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Program entered");

    let mut config = embassy_stm32::Config::default();

    // { stm32f411
    //     use embassy_stm32::rcc::*;

    //     config.rcc.sys = Sysclk::PLL1_P;
    //     config.rcc.hse = Some(Hse {
    //         freq: Hertz::mhz(25),
    //         mode: HseMode::Oscillator,
    //     });
    //     config.rcc.pll_src = PllSource::HSE;
    //     config.rcc.pll = Some(Pll {
    //         // 16MHz HSI
    //         prediv: PllPreDiv::DIV25,
    //         mul: PllMul::MUL192,
    //         divp: Some(PllPDiv::DIV2),
    //         divq: Some(PllQDiv::DIV4),
    //         divr: Some(PllRDiv::DIV2),
    //     });
    //     config.rcc.plli2s = Some(Pll {
    //         prediv: PllPreDiv::DIV12,
    //         mul: PllMul::MUL50,
    //         divp: Some(PllPDiv::DIV2),
    //         divq: Some(PllQDiv::DIV4),
    //         divr: Some(PllRDiv::DIV2),
    //     });
    //     config.rcc.ahb_pre = AHBPrescaler::DIV1;
    //     config.rcc.apb1_pre = APBPrescaler::DIV2; // Must give <=50MHz
    // }

    {
        // Stm32f412re
        use embassy_stm32::rcc::*;

        config.rcc.hsi = true;
        // config.rcc.hse = Some(Hse {
        //     freq: Hertz::mhz(8),
        //     mode: HseMode::Oscillator,
        // });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.pll_src = PllSource::HSI;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV16,
            mul: PllMul::MUL192,
            divp: Some(PllPDiv::DIV2),
            divq: Some(PllQDiv::DIV2),
            divr: Some(PllRDiv::DIV2),
        });
        config.rcc.plli2s = Some(Pll {
            prediv: PllPreDiv::DIV16,
            mul: PllMul::MUL384,
            divp: Some(PllPDiv::DIV2),
            divq: Some(PllQDiv::DIV2),
            divr: Some(PllRDiv::DIV2),
        });
        config.rcc.apb1_pre = APBPrescaler::DIV2;
    }

    let p = embassy_stm32::init(config);

    unsafe { init_global_heap() };
    {
        let mut vec = alloc::vec![1, 2, 3, 4, 5];
        vec.push(1);
        vec.pop();
        info!("HEAP Check with vector ran successfully");
    }

    info!(
        "SYS CLOCK FREQUENCY: {}",
        embassy_stm32::rcc::frequency::<peripherals::SYSCFG>()
    );

    // spawner
    //     .spawn(control_panel(
    //         CONTROL_PANEL_CHANNEL.sender(),
    //         ControlPanel::new((p.PA9, p.PA8), (p.PA12, p.PA11), (p.PA1, p.PA0)),
    //     ))
    //     .unwrap();

    spawner.spawn(status_led(p.PB2)).unwrap();

    let mut i2s = {
        let mut i2s_config = i2s::Config::default();

        i2s_config.mode = i2s::Mode::Master;
        i2s_config.standard = i2s::Standard::Philips;
        i2s_config.format = i2s::Format::Data16Channel16;
        i2s_config.clock_polarity = i2s::ClockPolarity::IdleLow;
        i2s_config.master_clock = true;

        let i2s = I2S::new_txonly(
            p.SPI3,
            p.PB5,
            p.PA4,
            p.PB3,
            p.PC7,
            p.DMA1_CH7,
            Hertz::hz(SAMPLE_RATE * 32),
            i2s_config,
        );

        info!(
            "I2S clock frequency: {}",
            embassy_stm32::rcc::frequency::<peripherals::SPI3>()
        );

        i2s
    };

    spawner
        .spawn(playback(SOUND_CHANNEL.receiver(), i2s))
        .unwrap();

    let mut display = {
        let display_i2c_cfg = i2c::Config::default();

        let display_i2c = i2c::I2c::new(
            p.I2C1,
            p.PB6,
            p.PB7,
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

        // display.clear(BinaryColor::Off).unwrap();
        // display.flush().unwrap();

        display
    };

    let mut sound = SimpleFormSource::infinite_stereo(
        SAMPLE_RATE,
        paw::audio::osc::simple_form::WaveForm::Sine,
        480.0,
    );

    let mut buf = [0u16; SOUND_BUFFER_SIZE];

    for s in buf.iter_mut() {
        *s = (sound.next_sample() * i16::MAX as f32) as i16 as u16;
    }

    info!("Starting main loop...");
    loop {
        SOUND_CHANNEL.send(buf).await;
        info!("Sent to playback");
    }
}
