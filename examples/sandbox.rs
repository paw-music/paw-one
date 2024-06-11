#![no_main]
#![no_std]

extern crate alloc;
extern crate paw_one;

use core::{cell::RefCell, sync::atomic::AtomicUsize};

use cortex_m::interrupt::Mutex;
use defmt::*;
use stm32f4xx_hal::{
    gpio::GpioExt,
    pac::{Peripherals, TIM5},
    prelude::*,
    timer::{CounterHz},
};
use {defmt_rtt as _, panic_probe as _};

static COMMON_TIMER: Mutex<RefCell<Option<CounterHz<TIM5>>>> = Mutex::new(RefCell::new(None));
static ELAPSED_US: AtomicUsize = AtomicUsize::new(0);

fn micros() -> usize {
    ELAPSED_US.load(core::sync::atomic::Ordering::Relaxed)
}

fn millis() -> usize {
    micros() / 1_000
}

// #[cortex_m_rt::interrupt]
// fn TIM5() {
//     cortex_m::interrupt::free(|cs| {
//         COMMON_TIMER
//             .borrow(cs)
//             .borrow_mut()
//             .as_mut()
//             .unwrap()
//             .clear_flags(Flag::Update);
//     });

//     ELAPSED_US.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
// }

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Sandbox entered");

    let dp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(96.MHz())
        .hclk(96.MHz())
        .i2s_apb1_clk(61440.kHz())
        // .pclk1(48.MHz())
        // .pclk2(96.MHz())
        .freeze();

    let gpiob = dp.GPIOB.split();

    let mut led = gpiob.pb2.into_push_pull_output();

    // {
    //     let mut common_timer = dp.TIM5.counter_hz(&clocks);
    //     common_timer
    //         .start(1.MHz())
    //         .expect("Failed to start common timer TIM5");
    //     common_timer.listen(Event::Update);

    //     cortex_m::interrupt::free(|cs| {
    //         COMMON_TIMER.borrow(cs).borrow_mut().replace(common_timer);
    //     });

    //     unsafe {
    //         NVIC::unmask(interrupt::TIM5);
    //     }
    // }

    // let prev_time = micros();

    loop {
        // let now = micros();

        // if now - prev_time > 1_000_000 {
        //     info!("SECOND");
        // }
        for _ in 0..1_000 {
            cortex_m::asm::nop();
        }
        led.toggle();
    }
}
