use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};

use cortex_m::interrupt::Mutex;
use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use stm32f4xx_hal::{
    dma::Stream1,
    i2c::{
        self,
        dma::{I2CMasterDma, I2CMasterWriteDMA as _, NoDMA, TxDMA},
    },
    pac::{DMA1, I2C1},
};

const DISPLAY_BUFFER_SIZE: usize = 128 * 32 / 8 + 1;
const COMMAND_BUFFER_SIZE: usize = 8;
const I2C_ADDRESS: u8 = 0x3C;

pub static DISPLAY_I2C: Mutex<RefCell<Option<I2c1Handle>>> = Mutex::new(RefCell::new(None));
static COMMAND_SEND: AtomicBool = AtomicBool::new(false);
static DRAWING: AtomicBool = AtomicBool::new(false);

pub type I2c1Handle = I2CMasterDma<
    I2C1,                          // Instance of I2C
    TxDMA<I2C1, Stream1<DMA1>, 0>, // Stream and channel used for Tx. First parameter must be same Instance as first generic parameter of I2CMasterDma
    NoDMA,                         // This example don't need Rx
>;

pub struct DisplayI2cDma {
    display_buffer: [u8; DISPLAY_BUFFER_SIZE],
    command_buffer: [u8; COMMAND_BUFFER_SIZE],
}

impl DisplayI2cDma {
    pub fn new() -> Self {
        Self {
            display_buffer: [0x40; DISPLAY_BUFFER_SIZE],
            command_buffer: [0x0; COMMAND_BUFFER_SIZE],
        }
    }
}

impl WriteOnlyDataCommand for DisplayI2cDma {
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError> {
        while COMMAND_SEND.load(Ordering::SeqCst) {
            core::hint::spin_loop()
        }

        match cmd {
            DataFormat::U8(slice) => {
                self.command_buffer[1..=slice.len()].copy_from_slice(&slice[0..slice.len()]);

                COMMAND_SEND.store(true, Ordering::SeqCst);
                nb::block!(cortex_m::interrupt::free(|cs| unsafe {
                    DISPLAY_I2C
                        .borrow(cs)
                        .borrow_mut()
                        .as_mut()
                        .unwrap()
                        .write_dma(
                            I2C_ADDRESS,
                            &self.command_buffer[..=slice.len()],
                            Some(|_| {
                                COMMAND_SEND.store(false, Ordering::SeqCst);
                            }),
                        )
                }))
                .ok(); // Ignore errors, Callback will handle it

                Ok(())
            }
            _ => Err(DisplayError::DataFormatNotImplemented),
        }
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError> {
        while DRAWING.load(Ordering::SeqCst) {
            core::hint::spin_loop()
        }

        match buf {
            DataFormat::U8(slice) => {
                self.display_buffer[1..=slice.len()].copy_from_slice(&slice[0..slice.len()]);

                DRAWING.store(true, Ordering::SeqCst);
                nb::block!(cortex_m::interrupt::free(|cs| unsafe {
                    DISPLAY_I2C
                        .borrow(cs)
                        .borrow_mut()
                        .as_mut()
                        .unwrap()
                        .write_dma(
                            I2C_ADDRESS,
                            &self.display_buffer[..=slice.len()],
                            Some(|_| DRAWING.store(false, Ordering::SeqCst)),
                        )
                }))
                .ok(); // Ignore errors, Callback will handle it

                Ok(())
            }
            _ => Err(DisplayError::DataFormatNotImplemented),
        }
    }
}
