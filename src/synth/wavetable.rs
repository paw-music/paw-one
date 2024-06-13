pub const WAVETABLE_SIZE: usize = 1024;

pub struct Wavetable {
    samples: [f32; WAVETABLE_SIZE],
}

impl Wavetable {
    pub fn gen(f: impl Fn(f32) -> f32) -> Self {
        let mut samples = [0.0; WAVETABLE_SIZE];

        for (i, s) in samples.iter_mut().enumerate() {
            *s = f(i as f32 / WAVETABLE_SIZE as f32);
        }

        Self { samples }
    }
}
