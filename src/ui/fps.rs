use crate::millis;

pub struct FPS {
    last_millis: u32,
}

impl FPS {
    pub fn new() -> Self {
        Self { last_millis: 0 }
    }

    pub fn value(&mut self) -> f32 {
        let now = millis();
        let fps = 1_000.0 / (now - self.last_millis) as f32;

        self.last_millis = now;

        fps
    }
}
