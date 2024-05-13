use embedded_graphics::{image::ImageRaw, pixelcolor::BinaryColor};

use super::{create_icons, Icon, MonoIcons};

create_icons! {
    pub MonoIcons5x7 / InvMonoIcons5x7: 5 * 7 {
        ARROW_LEFT = &[
            0b00001000,
            0b00010000,
            0b00100000,
            0b01000000,
            0b00100000,
            0b00010000,
            0b00001000,
        ],
        ARROW_RIGHT = &[
            0b01000000,
            0b00100000,
            0b00010000,
            0b00001000,
            0b00010000,
            0b00100000,
            0b01000000,
        ],
    }
}
