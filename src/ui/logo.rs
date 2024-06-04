use embedded_graphics::{image::ImageRaw, pixelcolor::BinaryColor};

pub const LOGO: ImageRaw<'static, BinaryColor> = ImageRaw::new(
    &[
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00011100, 0b00001111, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b11110000, 0b00111000, 0b00000000, 0b00000000, 0b00111111, 0b00111111,
        0b11000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000011, 0b11111100, 0b11111100, 0b00000000, 0b00000000,
        0b11111111, 0b11111111, 0b11000000, 0b00000111, 0b11111110, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000011, 0b11111111, 0b11111111,
        0b00000000, 0b00000001, 0b11111111, 0b11111111, 0b11000000, 0b00000100, 0b00000001,
        0b10000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000011,
        0b11111111, 0b11111111, 0b10000000, 0b00000001, 0b11111111, 0b11111111, 0b11100000,
        0b00000100, 0b00000000, 0b01000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000111, 0b11111111, 0b11111111, 0b10000000, 0b00000001, 0b11111001,
        0b11110001, 0b11100000, 0b00000100, 0b00000000, 0b00100000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000111, 0b10001111, 0b10011111, 0b10000000,
        0b00001101, 0b11110000, 0b11100000, 0b11100000, 0b00000100, 0b00000000, 0b00100000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000111, 0b00000111,
        0b00001111, 0b10110000, 0b00011111, 0b11100000, 0b01100000, 0b11101100, 0b00000100,
        0b00000000, 0b00010000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00110111, 0b00000110, 0b00000111, 0b11111000, 0b00011111, 0b11100000, 0b01100000,
        0b11111111, 0b00000100, 0b00000000, 0b00010000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b11111111, 0b00000110, 0b00000111, 0b11111000, 0b00111111,
        0b11100000, 0b01100000, 0b11111111, 0b00000100, 0b00000000, 0b00010000, 0b00000000,
        0b00000000, 0b00000000, 0b00001110, 0b00000000, 0b11111111, 0b00000110, 0b00000111,
        0b11111100, 0b00111111, 0b11110000, 0b11110001, 0b11111111, 0b00000100, 0b00000000,
        0b00010000, 0b00000000, 0b00000000, 0b00000000, 0b00000010, 0b00000000, 0b11111111,
        0b10001111, 0b00001111, 0b11111100, 0b01111111, 0b11111001, 0b11111111, 0b11111111,
        0b10000100, 0b00000000, 0b00010000, 0b00000000, 0b00000000, 0b00000000, 0b00000010,
        0b00000001, 0b11111111, 0b11111111, 0b10011111, 0b11111110, 0b01111100, 0b01111111,
        0b11111111, 0b11001111, 0b10000100, 0b00000000, 0b00100000, 0b00000000, 0b00000000,
        0b00000000, 0b00000010, 0b00000001, 0b11110011, 0b11111111, 0b11111110, 0b00111110,
        0b01111000, 0b00111111, 0b11111111, 0b10000111, 0b11000100, 0b00000000, 0b00100000,
        0b00000000, 0b00000000, 0b00000000, 0b00000010, 0b00000011, 0b11100001, 0b11111111,
        0b11111100, 0b00011110, 0b01111000, 0b00111111, 0b11111111, 0b00000011, 0b11000100,
        0b00000000, 0b01000000, 0b00000000, 0b00000000, 0b00000000, 0b00000010, 0b00000011,
        0b11000000, 0b11111111, 0b11111100, 0b00011110, 0b01111000, 0b00111111, 0b11111111,
        0b00000011, 0b11000100, 0b00000001, 0b10000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000010, 0b00000011, 0b11000000, 0b11111111, 0b11111100, 0b00011110, 0b01111100,
        0b00111111, 0b00001111, 0b00000011, 0b10000111, 0b11111110, 0b00000011, 0b11110000,
        0b00000000, 0b00000000, 0b00001111, 0b10000001, 0b11000000, 0b11110000, 0b11111100,
        0b00111110, 0b00111110, 0b01111100, 0b00000111, 0b10000111, 0b10000100, 0b00000000,
        0b00000100, 0b00001000, 0b00010000, 0b00000000, 0b00100000, 0b00000001, 0b11100001,
        0b11100000, 0b00111110, 0b01111100, 0b00011111, 0b11111000, 0b00000011, 0b11001111,
        0b10000100, 0b00000000, 0b00001000, 0b00000110, 0b00010000, 0b00000000, 0b00100000,
        0b00000001, 0b11110011, 0b11000000, 0b00011111, 0b11111000, 0b00001111, 0b11110000,
        0b00000001, 0b11111111, 0b00000100, 0b00000000, 0b00010000, 0b00000010, 0b00010000,
        0b00000000, 0b00100000, 0b00000000, 0b11111111, 0b10000000, 0b00001111, 0b11110000,
        0b00001111, 0b11110000, 0b00000001, 0b11111111, 0b00000100, 0b00000000, 0b00010000,
        0b00000010, 0b00010000, 0b00000000, 0b00100000, 0b00000000, 0b11111111, 0b10000000,
        0b00001111, 0b11110000, 0b00001111, 0b11110000, 0b00000001, 0b11111111, 0b00000100,
        0b00000000, 0b00010000, 0b00000010, 0b00010000, 0b00000000, 0b00100000, 0b00000000,
        0b11111111, 0b10000000, 0b00001111, 0b11110000, 0b00000111, 0b11110000, 0b00000001,
        0b11111110, 0b00000100, 0b00000000, 0b00010000, 0b00000010, 0b00010000, 0b00000000,
        0b00100000, 0b00000000, 0b01111111, 0b10000000, 0b00001111, 0b11100000, 0b00000111,
        0b11111000, 0b00000011, 0b11111110, 0b00000100, 0b00000000, 0b00010000, 0b00000010,
        0b00010000, 0b00000000, 0b00100000, 0b00000000, 0b01111111, 0b11000000, 0b00011111,
        0b11100000, 0b00000011, 0b11111111, 0b11111111, 0b11111100, 0b00000100, 0b00000000,
        0b00010000, 0b00000110, 0b00001000, 0b01111000, 0b01000000, 0b00000000, 0b00111111,
        0b11111111, 0b11111111, 0b11000000, 0b00000001, 0b11111111, 0b11111111, 0b11111000,
        0b00000100, 0b00000000, 0b00001000, 0b00011010, 0b00001000, 0b01001000, 0b01000000,
        0b00000000, 0b00011111, 0b11111111, 0b11111111, 0b10000000, 0b00000000, 0b11111111,
        0b11111111, 0b11100000, 0b00000100, 0b00000000, 0b00000100, 0b00100010, 0b00000100,
        0b10000100, 0b10000000, 0b00000000, 0b00000111, 0b11111111, 0b11111111, 0b00000000,
        0b00000000, 0b01111111, 0b11111111, 0b11000000, 0b00000100, 0b00000000, 0b00000011,
        0b11000010, 0b00000011, 0b00000011, 0b00000000, 0b00000000, 0b00000011, 0b11111111,
        0b11111110, 0b00000000, 0b00000000, 0b00011111, 0b11111111, 0b10000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000001, 0b11111111, 0b11111000, 0b00000000, 0b00000000, 0b00000011, 0b11111000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00011111, 0b11000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000,
    ],
    128,
);
