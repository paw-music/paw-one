#![no_std]
#![no_main]

pub mod sound;

pub mod control;
pub mod display_dma;
pub mod heap;
pub mod i2s;
pub mod synth;
pub mod ui;

#[macro_use]
extern crate alloc;

use core::{
    cell::RefCell,
    sync::atomic::{AtomicU32, AtomicUsize},
};

use cortex_m::interrupt::Mutex;
use cortex_m_semihosting::debug;
use display_dma::DisplayI2cDma;
// use panic_halt as _;
// use panic_semihosting as _;
use panic_probe as _;
use stm32_i2s_v12x::{
    marker::{Data32Channel32, Master, Philips, Transmit},
    transfer::I2sTransfer,
};
use stm32f4xx_hal::{i2c::I2c, i2s::I2s3};

#[inline(never)]
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}

pub mod board_info {
    use embedded_graphics::geometry::Size;

    pub const DISPLAY_SIZE: Size = Size::new(128, 32);
}

pub const SAMPLE_RATE: u32 = 48_000;
pub const AUDIO_BUFFER_SIZE: usize = 1024;
pub const DMA_AUDIO_BUFFER_SIZE: usize = AUDIO_BUFFER_SIZE * 2 * 2;
pub type DmaAudioBuffer = [u16; DMA_AUDIO_BUFFER_SIZE];
pub type MainI2s = I2sTransfer<I2s3, Master, Transmit, Philips, Data32Channel32>;
pub type Display = ssd1306::Ssd1306<
    DisplayI2cDma,
    ssd1306::prelude::DisplaySize128x32,
    ssd1306::mode::BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x32>,
>;
pub static ELAPSED_MS: AtomicU32 = AtomicU32::new(0);

pub static AUDIO_BUFFER: Mutex<RefCell<heapless::Deque<(i32, i32), AUDIO_BUFFER_SIZE>>> =
    Mutex::new(RefCell::new(heapless::Deque::new()));

// pub fn micros() -> u32 {
//     ELAPSED_US.load(core::sync::atomic::Ordering::Relaxed)
// }

pub fn millis() -> u32 {
    ELAPSED_MS.load(core::sync::atomic::Ordering::Relaxed)
    // micros() / 1_000
}
