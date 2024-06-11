#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate paw_one;

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use alloc::format;
use cortex_m::interrupt::Mutex;
use defmt::*;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    image::Image,
    mono_font::{ascii::FONT_4X6, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    primitives::Rectangle,
    Drawable,
};
use embedded_text::TextBox;
use embedded_ui::{col, helpers::select, ui::UI};
use micromath::F32Ext;
use paw_one::{
    control::{btn::PullUp, ControlPanel, ControlsState},
    display_dma::{DisplayI2cDma, DISPLAY_I2C},
    heap::init_global_heap,
    millis,
    synth::Synth,
    ui::{fps::FPS, logo::LOGO, Message},
    DmaAudioBuffer, AUDIO_BUFFER, AUDIO_BUFFER_SIZE, ELAPSED_MS, SAMPLE_RATE,
};
use ssd1306::{mode::DisplayConfig as _, prelude::Brightness};
use stm32_i2s_v12x::{
    driver::{I2sDriver, I2sDriverConfig},
    marker::{Master, Philips, Transmit},
};
use stm32f4xx_hal::{
    dma::{config::DmaConfig, DmaFlag, MemoryToPeripheral, Stream5, StreamsTuple, Transfer},
    i2s::I2s3,
    pac::{DMA1, TIM3},
    prelude::*,
    timer::{CounterHz, Event, Flag},
};
use stm32f4xx_hal::{
    gpio::GpioExt,
    i2c::{self, I2c},
    i2s::I2s,
    interrupt,
    pac::{Peripherals, NVIC, TIM5},
    prelude::_fugit_RateExtU32,
    rcc::RccExt,
    timer::TimerExt,
};
use {defmt_rtt as _, panic_probe as _};

// static I2S_TIMER: Mutex<RefCell<Option<CounterHz<TIM2>>>> = Mutex::new(RefCell::new(None));
// static UI_TIMER: Mutex<RefCell<Option<CounterHz<TIM4>>>> = Mutex::new(RefCell::new(None));
// static DISPLAY: Mutex<RefCell<Option<Display>>> = Mutex::new(RefCell::new(None));
static SYNTH_TIMER: Mutex<RefCell<Option<CounterHz<TIM3>>>> = Mutex::new(RefCell::new(None));
static SYNTH: Mutex<RefCell<Option<Synth>>> = Mutex::new(RefCell::new(None));
// static I2S: Mutex<RefCell<Option<I2sDriver<I2s<SPI3>, Master, Transmit, Philips>>>> =
//     Mutex::new(RefCell::new(None));
// static I2S: Mutex<
//     RefCell<Option<I2sTransfer<I2s<SPI3>, Master, Transmit, Philips, Data32Channel32>>>,
// > = Mutex::new(RefCell::new(None));
static AUDIO_BUFFER_UNDERRUN_COUNT: AtomicUsize = AtomicUsize::new(0);
static COMMON_TIMER: Mutex<RefCell<Option<CounterHz<TIM5>>>> = Mutex::new(RefCell::new(None));
// static CONTROLS_STATE: Mutex<RefCell<Option<ControlsState>>> = Mutex::new(RefCell::new(None));
// static CONTROL_PANEL: Mutex<
//     RefCell<
//         Option<
//             ControlPanel<
//                 stm32f4xx_hal::gpio::Pin<'A', 0>,
//                 stm32f4xx_hal::gpio::Pin<'A', 1>,
//                 stm32f4xx_hal::gpio::Pin<'A', 3>,
//                 stm32f4xx_hal::gpio::Pin<'A', 5>,
//                 stm32f4xx_hal::gpio::Pin<'C', 0>,
//                 stm32f4xx_hal::gpio::Pin<'C', 1>,
//             >,
//         >,
//     >,
// > = Mutex::new(RefCell::new(None));

type I2sDmaTransfer = Transfer<
    Stream5<DMA1>,
    0,
    I2sDriver<I2s3, Master, Transmit, Philips>,
    MemoryToPeripheral,
    &'static mut DmaAudioBuffer,
>;
static I2S_DMA_TRANSFER: Mutex<RefCell<Option<I2sDmaTransfer>>> = Mutex::new(RefCell::new(None));

// #[interrupt]
// fn TIM2() {
//     cortex_m::interrupt::free(|cs| {
//         I2S_TIMER
//             .borrow(cs)
//             .borrow_mut()
//             .as_mut()
//             .unwrap()
//             .clear_flags(Flag::Update);

//         // timer
//         //     .borrow_mut()
//         //     .as_mut()
//         //     .unwrap()
//         //     .start(SAMPLE_RATE.Hz())
//         //     .unwrap();

//         let frame = AUDIO_BUFFER.borrow(cs).borrow_mut().pop_front();
//         // let frame = frame.unwrap_or_else(|| {
//         //     // AUDIO_BUFFER_UNDERRUN_COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
//         //     (0, 0)
//         // });
//         // let frame = frame.expect("Audio buffer underrun");
//         // block!(I2S.borrow(cs).borrow_mut().as_mut().unwrap().write(frame)).unwrap();
//         // info!("Sent frame");
//     });
// }

#[interrupt]
fn TIM5() {
    cortex_m::interrupt::free(|cs| {
        COMMON_TIMER
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_flags(Flag::Update);
    });

    ELAPSED_MS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
}

#[interrupt]
fn DMA1_STREAM1() {
    cortex_m::interrupt::free(|cs| {
        DISPLAY_I2C
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .handle_dma_interrupt()
    });
}

#[interrupt]
fn I2C1_ER() {
    cortex_m::interrupt::free(|cs| {
        DISPLAY_I2C
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .handle_error_interrupt()
    });
}

// #[interrupt]
// fn DMA1_STREAM5() {
//     info!("DMA1_STREAM5");
// }

#[interrupt]
fn TIM3() {
    cortex_m::interrupt::free(|cs| {
        SYNTH_TIMER
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_flags(Flag::Update);

        SYNTH.borrow(cs).borrow_mut().as_mut().unwrap().tick();
    });
}

// #[interrupt]
// fn TIM4() {
//     cortex_m::interrupt::free(|cs| {
//         UI_TIMER
//             .borrow(cs)
//             .borrow_mut()
//             .as_mut()
//             .unwrap()
//             .clear_flags(Flag::Update);

//         DISPLAY
//             .borrow(cs)
//             .borrow_mut()
//             .as_mut()
//             .unwrap()
//             .flush()
//             .unwrap();
//     });
// }

#[interrupt]
fn DMA1_STREAM5() {
    static mut TRANSFER: Option<I2sDmaTransfer> = None;
    let transfer = TRANSFER.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| I2S_DMA_TRANSFER.borrow(cs).replace(None).unwrap())
    });

    let flags = transfer.flags();

    transfer.clear_flags(DmaFlag::FifoError);

    if flags.is_transfer_complete() {
        cortex_m::interrupt::free(|cs| {
            unsafe {
                transfer
                    .next_transfer_with(|buffer, _active| {
                        let mut pointer = 0;
                        while let Some(frame) = AUDIO_BUFFER.borrow(cs).borrow_mut().pop_front() {
                            let mut data = [0; 4];
                            let left = frame.0.to_be_bytes();
                            let right = frame.1.to_be_bytes();

                            data[0..2].copy_from_slice(&[
                                u16::from_be_bytes([left[0], left[1]]),
                                u16::from_be_bytes([left[2], left[3]]),
                            ]);

                            data[2..4].copy_from_slice(&[
                                u16::from_be_bytes([right[0], right[1]]),
                                u16::from_be_bytes([right[2], right[3]]),
                            ]);

                            buffer[pointer..pointer + 4].copy_from_slice(&data);
                            pointer += 4;
                        }

                        if pointer < buffer.len() {
                            // defmt::panic!("Underrun");
                            AUDIO_BUFFER_UNDERRUN_COUNT
                                .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                        }
                        (buffer, ())
                    })
                    .unwrap();
            }
        });
    }
    if flags.is_fifo_error() {
        warn!("I2S DMA Stream FIFO Error!");
    }
    if flags.is_transfer_error() {
        warn!("I2S DMA Stream Transfer Error!");
    }
    if flags.is_empty() {
        warn!("Audio buffer is empty!");
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Program entered");

    unsafe { init_global_heap() };
    {
        let mut vec = alloc::vec![1, 2, 3, 4, 5];
        vec.push(1);
        vec.pop();
        info!("HEAP Check with vector ran successfully");
    }

    let dp = Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let syscfg = dp.SYSCFG.constrain();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    let rcc = dp.RCC.constrain();

    let dma1 = StreamsTuple::new(dp.DMA1);

    // let clocks = rcc
    //     .cfgr
    //     // .use_hse(8u32.MHz())
    //     .sysclk(96u32.MHz())
    //     // .hclk(96u32.MHz())
    //     .i2s_apb1_clk(61440u32.kHz())
    //     // .i2s_apb1_clk(61440u32.kHz())
    //     // .pclk1(48u32.MHz())
    //     // .pclk2(96u32.MHz())
    //     .freeze();

    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(96.MHz())
        .hclk(96.MHz())
        .i2s_apb1_clk(61440.kHz())
        // .pclk1(48.MHz())
        // .pclk2(96.MHz())
        .freeze();

    let mut display = {
        unsafe {
            NVIC::unmask(interrupt::DMA1_STREAM1);
            NVIC::unmask(interrupt::I2C1_ER);
            NVIC::unmask(interrupt::I2C1_EV);
        }

        let display_i2c = I2c::new(
            dp.I2C1,
            (gpiob.pb6, gpiob.pb7),
            i2c::Mode::Fast {
                frequency: 400u32.kHz(),
                duty_cycle: i2c::DutyCycle::Ratio2to1,
            },
            &clocks,
        )
        .use_dma_tx(dma1.1);

        cortex_m::interrupt::free(|cs| {
            DISPLAY_I2C.borrow(cs).borrow_mut().replace(display_i2c);
        });

        let display_i2c_dma = DisplayI2cDma::new();

        let mut display = ssd1306::Ssd1306::new(
            display_i2c_dma,
            ssd1306::size::DisplaySize128x32,
            ssd1306::rotation::DisplayRotation::Rotate0,
        )
        .into_buffered_graphics_mode();
        display.init().unwrap();
        display.flush().unwrap();

        display.set_brightness(Brightness::NORMAL).unwrap();

        info!("Initialized SSD1306 display");

        display.clear(BinaryColor::Off).unwrap();

        Image::new(&LOGO, Point::zero()).draw(&mut display).unwrap();
        display.flush().unwrap();

        display
    };

    // Note: Attempt to reload UI by interrupt
    // cortex_m::interrupt::free(|cs| {
    //     *DISPLAY.borrow(cs).borrow_mut() = Some(display);
    // });

    // {
    //     let mut ui_timer = dp.TIM4.counter_hz(&clocks);
    //     ui_timer
    //         .start(1.Hz())
    //         .expect("Failed to start UI timer TIM4");
    //     ui_timer.listen(Event::Update);

    //     cortex_m::interrupt::free(|cs| {
    //         UI_TIMER.borrow(cs).borrow_mut().replace(ui_timer);
    //     });

    //     unsafe {
    //         NVIC::unmask(interrupt::TIM4);
    //     }
    // }

    let synth = Synth::new();

    cortex_m::interrupt::free(|cs| {
        SYNTH.borrow(cs).borrow_mut().replace(synth);
    });

    {
        let mut synth_timer = dp.TIM3.counter_hz(&clocks);
        synth_timer
            .start((SAMPLE_RATE * 4).Hz())
            .expect("Failed to initialize Synth timer TIM3");
        synth_timer.listen(Event::Update);

        cortex_m::interrupt::free(|cs| {
            *SYNTH_TIMER.borrow(cs).borrow_mut() = Some(synth_timer);
        });

        unsafe {
            NVIC::unmask(interrupt::TIM3);
        }
    }

    // {
    //     let mut i2s_timer = dp.TIM2.counter_hz(&clocks);

    //     i2s_timer
    //         .start(SAMPLE_RATE.Hz())
    //         .expect("Failed to start I2S timer TIM2");

    //     i2s_timer.listen(Event::Update);

    //     cortex_m::interrupt::free(|cs| {
    //         I2S_TIMER.borrow(cs).borrow_mut().replace(i2s_timer);
    //     });

    //     unsafe {
    //         // cp.NVIC.set_priority(interrupt::TIM2, 0);
    //         NVIC::unmask(interrupt::TIM2);
    //     }
    // }

    // {
    //     let mut ui_timer = dp.TIM4.counter_hz(&clocks);
    //     ui_timer
    //         .start(50.Hz())
    //         .expect("Failed to start UI timer TIM4");
    //     ui_timer.listen(Event::Update);

    //     cortex_m::interrupt::free(|cs| {
    //         UI_TIMER.borrow(cs).borrow_mut().replace(ui_timer);
    //     });

    //     unsafe {
    //         NVIC::unmask(interrupt::TIM4);
    //     }
    // }

    {
        let mut common_timer = dp.TIM5.counter_hz(&clocks);
        common_timer
            .start(1.kHz())
            .expect("Failed to start common timer TIM5");
        common_timer.listen(Event::Update);

        cortex_m::interrupt::free(|cs| {
            COMMON_TIMER.borrow(cs).borrow_mut().replace(common_timer);
        });

        unsafe {
            NVIC::unmask(interrupt::TIM5);
        }
    }

    let i2s = {
        let pins = (gpioa.pa4, gpiob.pb12, gpioc.pc7, gpiob.pb5);
        let i2s = I2s::new(dp.SPI3, pins, &clocks);

        let i2s_driver_config = I2sDriverConfig::new_master()
            .transmit()
            .data_format(stm32_i2s_v12x::driver::DataFormat::Data32Channel32)
            .require_frequency(SAMPLE_RATE)
            .master_clock(true)
            .standard(Philips);
        let mut i2s_driver = I2sDriver::new(i2s, i2s_driver_config);

        i2s_driver.set_tx_dma(true);
        i2s_driver.set_tx_interrupt(true);

        i2s_driver
    };

    {
        let dma1ch5 = dma1.5;

        let i2s_buf1 =
        cortex_m::singleton!(: [u16; AUDIO_BUFFER_SIZE * 2 * 2] = [0; AUDIO_BUFFER_SIZE * 2 * 2])
            .unwrap();
        let i2s_buf2 =
        cortex_m::singleton!(: [u16; AUDIO_BUFFER_SIZE * 2 * 2] = [0; AUDIO_BUFFER_SIZE * 2 * 2])
            .unwrap();

        let mut i2s_dma_transfer = Transfer::init_memory_to_peripheral(
            dma1ch5,
            i2s,
            i2s_buf1,
            Some(i2s_buf2),
            DmaConfig::default()
                .double_buffer(true)
                // .fifo_enable(true)
                // .fifo_threshold(stm32f4xx_hal::dma::config::FifoThreshold::ThreeQuarterFull)
                .transfer_complete_interrupt(true)
                // .fifo_error_interrupt(true)
                // .half_transfer_interrupt(true)
                .priority(stm32f4xx_hal::dma::config::Priority::VeryHigh)
                .transfer_error_interrupt(true)
                .memory_increment(true),
        );

        i2s_dma_transfer.start(|i2s| i2s.enable());

        cortex_m::interrupt::free(|cs| {
            *I2S_DMA_TRANSFER.borrow(cs).borrow_mut() = Some(i2s_dma_transfer);
        });

        unsafe {
            NVIC::unmask(interrupt::DMA1_STREAM5);
        }
    }

    let mut ui = {
        let root = col![
            select(["55", "110", "220", "440", "880", "1760"]).on_change(|new| {
                info!("Select changed to {}", new);
                Message::None
            })
        ];
        let mut ui = UI::new(root, display.bounding_box().size.into()).monochrome();

        ui.auto_focus();

        ui
    };

    let mut control_panel = {
        let main_enc = (gpioa.pa0, gpioa.pa1);
        let main_enc_btn = (gpioa.pa2, PullUp);
        let red_enc = (gpioa.pa3, gpioa.pa5);
        let green_enc = (gpioc.pc0, gpioc.pc1);

        ControlPanel::new(main_enc, main_enc_btn, red_enc, green_enc)
    };

    // cortex_m::interrupt::free(|cs| {
    //     *CONTROL_PANEL.borrow(cs).borrow_mut() = Some(control_panel);
    // });

    // cortex_m::interrupt::free(|cs| {
    //     CONTROL_PANEL
    //         .borrow(cs)
    //         .borrow_mut()
    //         .as_mut()
    //         .unwrap()
    //         .configure_interrupts(&mut syscfg, &mut dp.EXTI);
    // });

    let mut last_frame_ms = millis();

    const FIXED_FPS: u32 = 25;
    const FPS_MS_PERIOD: u32 = 1_000 / FIXED_FPS;

    let mut fps = FPS::new();

    info!("Starting main loop...");
    loop {
        let now = millis();

        // synth.tick();

        if let ControlsState::Changed(changed) = control_panel.tick(now) {
            info!("Changed {}", changed);
            ui.tick(changed.into_events().into_iter());
        }

        if now - last_frame_ms > FPS_MS_PERIOD {
            ui.draw(&mut display);

            // Text::new(format!("{}FPS", ), Point::new(x, y), character_style)
            TextBox::new(
                &format!("{}FPS", fps.value().round() as u32),
                Rectangle::new(Point::new(0, 0), Size::new(24, 7)),
                MonoTextStyleBuilder::new()
                    .font(&FONT_4X6)
                    .text_color(BinaryColor::On)
                    .background_color(BinaryColor::Off)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

            TextBox::new(
                &format!(
                    "UDR: {}",
                    AUDIO_BUFFER_UNDERRUN_COUNT.load(core::sync::atomic::Ordering::Relaxed)
                ),
                Rectangle::new(Point::new(0, 25), Size::new(64, 6)),
                MonoTextStyleBuilder::new()
                    .font(&FONT_4X6)
                    .text_color(BinaryColor::On)
                    .background_color(BinaryColor::Off)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

            display.flush().unwrap();

            last_frame_ms = now;
        }

        // if main_timer
        //     .now()
        //     .checked_duration_since(last_frame_ms)
        //     .unwrap()
        //     .to_millis()
        //     > 1_000
        // {
        //     info!("Second");
        //     last_frame_ms = main_timer.now();
        // }

        // if main_timer.now().duration_since_epoch(). {
        //     ui.draw(&mut display);
        //     display.flush().unwrap();
        // }

        // let now_micros = timer.now().duration_since_epoch().to_micros();

        // info!("NOW {}us", now);

        // SOUND_CHANNEL.send(buf).await;

        // match control_panel.tick(timer.now().duration_since_epoch().to_millis() as u64) {
        //     ControlsState::None => {}
        //     ControlsState::Changed(changed) => {
        //         info!("State changed to {}", changed);
        //     }
        // }

        // // ui.tick(core::iter::once(EventStub));

        // if !buffer.is_full() {
        //     let sample = (sound.next_sample() * 0.4 * i32::MAX as f32) as i32;
        //     buffer.push_back((sample, sample)).ok();
        // } else {
        //     // info!("Buffer is full!");
        //     // asm::nop();
        // }

        // for _ in 0..2 {
        //     let frame = sending_frame.or_else(|| buffer.pop_front());

        //     if let Some(frame) = frame {
        //         match i2s.write(frame) {
        //             Ok(()) => {
        //                 sending_frame = None;
        //             }
        //             Err(err) => match err {
        //                 nb::Error::Other(e) => defmt::unreachable!(),
        //                 nb::Error::WouldBlock => {
        //                     sending_frame = Some(frame);
        //                 }
        //             },
        //         }
        //         // let dur = dwt.measure(|| {
        //         // block!(i2s.write(frame)).unwrap();
        //         // });
        //         // info!("Sent sample {} ticks", dur.as_ticks());
        //     }
        // }

        // // 25FPS
        // if sending_frame.is_none() && now_micros - last_frame_us > 1_000_000 {
        //     // info!("Update display; now={}", now);

        //     last_frame_us = now_micros;
        // }
    }
}
