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
    pac::RCC,
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

const SOUND_BUFFER_SIZE: usize = 1024;
type SoundBuffer = [u16; SOUND_BUFFER_SIZE];
type SoundChannel = embassy_sync::channel::Channel<ThreadModeRawMutex, SoundBuffer, 4>;
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
        // Timer::after_millis(1).await;
    }
}

#[embassy_executor::task]
async fn playback(
    channel: Receiver<'static, ThreadModeRawMutex, SoundBuffer, 4>,
    mut i2s: I2S<'static>,
) {
    loop {
        let buf = channel.receive().await;
        i2s.write(&buf).await.unwrap();

        // info!("Sent to i2s");

        // Timer::after_micros(10).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Program entered");

    /*
     * Need this:
     * RCC: Clocks { hclk: 96000000 Hz, pclk1: 48000000 Hz, pclk2: 96000000 Hz, timclk1: 96000000 Hz, timclk2: 96000000 Hz, sysclk: 96000000 Hz, pll48clk: None, i2s_apb1_clk: Some(61440000 Hz), i2s_apb2_clk: Some(61440000 Hz) }
     *
     * { hclk1: Some(Hertz(96000000)), hclk2: Some(Hertz(96000000)), hclk3: Some(Hertz(96000000)), pclk1: Some(Hertz(48000000)), pclk1_tim: Some(Hertz(96000000)), pclk2: Some(Hertz(96000000)), pclk2_tim: Some(Hertz(96000000)), pll1_q: Some(Hertz(192000000)), pllsai1_q: None, rtc: Some(Hertz(32000)), sys: Some(Hertz(96000000)) }
     */
    let config = {
        let mut config = embassy_stm32::Config::default();

        // Stm32f412re
        use embassy_stm32::rcc::*;

        config.rcc.hsi = true;
        config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(8),
            mode: HseMode::Oscillator,
        });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV8,
            mul: PllMul::MUL192,
            divp: Some(PllPDiv::DIV2),
            divq: Some(PllQDiv::DIV2),
            divr: Some(PllRDiv::DIV2),
        });
        config.rcc.plli2s = Some(Pll {
            prediv: PllPreDiv::DIV5,
            mul: PllMul::MUL192,
            divp: Some(PllPDiv::DIV4),
            divq: Some(PllQDiv::DIV2),
            divr: Some(PllRDiv::DIV5),
        });
        config.rcc.apb1_pre = APBPrescaler::DIV2;

        // embassy_stm32::pac::RCC.plli2scfgr().modify(|w| {
        //     w.set_plli2ssrc(embassy_stm32::pac::rcc::vals::Plli2ssrc::HSE_HSI);
        // });

        config
    };

    let p = embassy_stm32::init(config);

    // embassy_stm32::pac::RCC.dckcfgr().modify(|w| {
    //     w.set_i2s1src(embassy_stm32::pac::rcc::vals::I2s1src::PLLI2SR);
    // });

    // RCC.cr().modify(|w| w.set_plli2son(false));
    // while RCC.cr().read().plli2srdy() != false {}

    // embassy_stm32::pac::RCC.plli2scfgr().write(|w| {
    //     use embassy_stm32::rcc::*;
    //     w.set_plli2ssrc(embassy_stm32::pac::rcc::vals::Plli2ssrc::HSE_HSI);
    //     w.set_pllsrc(PllSource::HSE);
    //     w.set_pllm(PllPreDiv::DIV5);
    //     w.set_plln(PllMul::MUL192);
    //     w.set_pllp(PllPDiv::DIV4);
    //     w.set_pllq(PllQDiv::DIV2);
    //     w.set_pllr(PllRDiv::DIV5);
    // });

    // RCC.cr().modify(|w| w.set_plli2son(true));
    // while RCC.cr().read().plli2srdy() != true {}

    let plli2scfgr = embassy_stm32::pac::RCC.plli2scfgr().read();

    info!(
        "I2S PLL: pllm={}, plln={}, pllp={}, pllq={}, pllr={}",
        plli2scfgr.pllm().to_bits(),
        plli2scfgr.plln().to_bits(),
        plli2scfgr.pllp().to_bits(),
        plli2scfgr.pllq().to_bits(),
        plli2scfgr.pllr().to_bits()
    );

    info!(
        "I2S clock: {}",
        8_000_000u32 / Into::<u8>::into(plli2scfgr.pllm()) as u32
            * Into::<u16>::into(plli2scfgr.plln()) as u32
            / Into::<u8>::into(plli2scfgr.pllr()) as u32
    );

    let dckcfgr = embassy_stm32::pac::RCC.dckcfgr().read();

    info!(
        "dckcfgr: {}",
        (
            dckcfgr.i2s1src() as u8,
            dckcfgr.i2s2src() as u8,
            dckcfgr.i2ssrc() as u8,
            dckcfgr.plli2sdivq() as u8,
            dckcfgr.plli2sdivr() as u8,
        )
    );

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

    info!(
        "SPI clocks: {}",
        (
            embassy_stm32::rcc::frequency::<peripherals::SPI1>(),
            embassy_stm32::rcc::frequency::<peripherals::SPI2>(),
            embassy_stm32::rcc::frequency::<peripherals::SPI3>(),
            embassy_stm32::rcc::frequency::<peripherals::SPI4>(),
            embassy_stm32::rcc::frequency::<peripherals::SPI5>(),
        )
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
        i2s_config.format = i2s::Format::Data32Channel32;
        i2s_config.clock_polarity = i2s::ClockPolarity::IdleLow;
        i2s_config.master_clock = true;

        let i2s = I2S::new_txonly(
            p.SPI3,
            p.PB5,
            p.PA4,
            p.PB3,
            p.PC7,
            p.DMA1_CH7,
            Hertz::hz(SAMPLE_RATE),
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
        *s = (sound.next_sample() * 0.1 * i16::MAX as f32) as i16 as u16;
    }

    info!("Starting main loop...");
    loop {
        SOUND_CHANNEL.send(buf).await;
        // info!("Sent to playback");
    }
}
