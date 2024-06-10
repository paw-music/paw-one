use crate::micros;

pub struct FPS {
    last_micros: usize,
}

impl FPS {
    pub fn new() -> Self {
        Self { last_micros: 0 }
    }

    pub fn value(&mut self) -> f32 {
        let now = micros();
        let fps = 1_000_000.0 / (now - self.last_micros) as f32;

        self.last_micros = now;

        fps
    }
}
