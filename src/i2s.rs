// // use core::marker::PhantomData;

// // use embassy_stm32::{gpio::Output, Peripheral};

// // pub struct I2S<'a, SPI> {
// //     marker: PhantomData<&'a SPI>,
// // }

// // pub struct WsPin<'a> {
// //     output: Output<'a>,
// // }

// // impl<'a> stm32_i2s_v12x::WsPin for WsPin<'a> {
// //     fn is_low(&self) -> bool {
// //         self.output.is_set_low()
// //     }

// //     fn is_high(&self) -> bool {
// //         self.output.is_set_high()
// //     }
// // }

// // unsafe impl<'a, SPI: embassy_stm32::spi::Instance> stm32_i2s_v12x::I2sPeripheral for I2S<'a, SPI> {
// //     type WsPin = WsPin<'a>;

// //     const REGISTERS: *const () = SPI;

// //     fn i2s_freq(&self) -> u32 {
// //         todo!()
// //     }

// //     fn ws_pin(&self) -> &Self::WsPin {
// //         todo!()
// //     }

// //     fn ws_pin_mut(&mut self) -> &mut Self::WsPin {
// //         todo!()
// //     }

// //     fn rcc_reset(&mut self) {
// //         todo!()
// //     }
// // }

// use stm32f4xx_hal::dma::CurrentBuffer;

// use crate::{DmaAudioBuffer, DMA_AUDIO_BUFFER_SIZE};

// pub struct I2sDoubleBuffer {
//     buffer1: &'static mut DmaAudioBuffer,
//     buffer2: &'static mut DmaAudioBuffer,
//     active: CurrentBuffer,
//     pointer: usize,
// }

// impl I2sDoubleBuffer {
//     pub fn new() -> Self {
//         let buffer1 = cortex_m::singleton!(: DmaAudioBuffer = [0; DMA_AUDIO_BUFFER_SIZE]).unwrap();
//         let buffer2 = cortex_m::singleton!(: DmaAudioBuffer = [0; DMA_AUDIO_BUFFER_SIZE]).unwrap();

//         Self {
//             buffer1,
//             buffer2,
//             active: CurrentBuffer::FirstBuffer,
//             pointer: 0,
//         }
//     }

//     fn sample_to_words(sample: i32) -> [u16; 2] {
//         let mut words = [0u16; 2];
//         let bytes = sample.to_be_bytes();

//         words[0] = u16::from_be_bytes([bytes[0], bytes[1]]);
//         words[1] = u16::from_be_bytes([bytes[2], bytes[3]]);

//         words
//     }

//     fn buffer(&self) -> &DmaAudioBuffer {
//         match self.active {
//             CurrentBuffer::FirstBuffer => self.buffer1,
//             CurrentBuffer::SecondBuffer => self.buffer2,
//         }
//     }

//     fn buffer_mut(&mut self) -> &mut DmaAudioBuffer {
//         match self.active {
//             CurrentBuffer::FirstBuffer => self.buffer1,
//             CurrentBuffer::SecondBuffer => self.buffer2,
//         }
//     }

//     fn full(&self) -> bool {
//         self.pointer >= self.buffer().len()
//     }

//     pub fn take_buffer(&mut self) -> &mut DmaAudioBuffer {
//         let buf = match self.active {
//             CurrentBuffer::FirstBuffer => &mut self.buffer1,
//             CurrentBuffer::SecondBuffer => &mut self.buffer2,
//         };
//         self.active = !self.active;
//         self.pointer = 0;
//         buf
//     }

//     pub fn when_unfilled<F: Fn() -> (i32, i32)>(&mut self, frame: F) {
//         if !self.full() {
//             let frame = frame();
//             let left_slice = self.pointer..self.pointer + 2;
//             let right_slice = self.pointer + 2..self.pointer + 4;

//             self.buffer_mut()[left_slice].copy_from_slice(&Self::sample_to_words(frame.0));
//             self.buffer_mut()[right_slice].copy_from_slice(&Self::sample_to_words(frame.1));

//             self.pointer += 4;
//         }
//     }
// }
