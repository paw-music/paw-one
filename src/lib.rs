#![no_std]
#![no_main]

pub mod sound;
pub mod ui;

pub mod control;
pub mod heap;
pub mod synth;

#[macro_use]
extern crate alloc;

use cortex_m_semihosting::debug;
// use panic_halt as _;
// use panic_semihosting as _;
use panic_probe as _;

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
