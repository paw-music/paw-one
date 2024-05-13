pub mod ui;

use defmt::info;
use embassy_time::Duration;

use micromath::F32Ext;

use crate::control::enc::EditByEncoder;

// ADSR (DAHDSR) stages //

pub mod values {
    // // 1e0, 1e1, 1e2, 1e3 (millis), 1e4, 1e5, 1e6 (seconds)
    // pub const DURATION_POWER_EDIT_OFFSET: [i32; 7] = [1, 5, 10, 100, , 50, 250];

    // Millis offset when editing for powers:
    // 1e0, 1e1, 1e2, 1e3 (secs).
    // Note: These values are maximums because acceleration is used on encoder.
    //  So these values are multiplied by 0-1.0
    pub const DURATION_POWER_EDIT_OFFSET: [i32; 4] = [5, 10, 25, 250];

    pub const DELAY_DURATION_RANGE: core::ops::Range<i32> = 0..10_000;
    pub const ATTACK_DURATION_RANGE: core::ops::Range<i32> = 0..10_000;
    pub const HOLD_DURATION_RANGE: core::ops::Range<i32> = 0..10_000;
    pub const DECAY_DURATION_RANGE: core::ops::Range<i32> = 0..10_000;
    pub const RELEASE_DURATION_RANGE: core::ops::Range<i32> = 0..10_000;

    pub const AMPLITUDE_MAX_STEP: f32 = 0.05;

    pub const CURVE_BEND_MAX_TENSION: f32 = 20.0;
    pub const CURVE_BEND_ZERO_ERROR: f32 = 0.01;
    pub const CURVE_BEND_EDIT_MAX_STEP: f32 = 0.1;
    pub const CURVE_BEND_RANGE: core::ops::RangeInclusive<f32> = -1.0..=1.0;
}

#[derive(Clone, Copy, defmt::Format)]
pub struct AdsrAmplitude(pub f32);

impl EditByEncoder for AdsrAmplitude {
    type Meta = ();

    fn edit_by_encoder(&mut self, offset: i32, vel: f32, _: Self::Meta) -> &mut Self {
        let offset_sign = offset.signum() as f32;
        let offset = (offset as f32 * vel * values::AMPLITUDE_MAX_STEP * 100.0).round() / 100.0;

        let offset = if offset == 0.0 {
            offset_sign * 0.01
        } else {
            offset
        };

        self.0 = (self.0 as f32 + offset).clamp(0.0, 1.0);

        self
    }
}

impl From<f32> for AdsrAmplitude {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for AdsrAmplitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // if self.0 < 0.01 {
        //     write!(f, "{:.2}%", self.0 * 100.0)
        // } else if self.0 < 0.1 {
        //     write!(f, "{:.1}%", self.0 * 100.0)
        // } else {
        //     write!(f, "{:.0}%", self.0 * 100.0)
        // }

        if self.0 < 0.01 {
            write!(f, "{:.1}%", self.0 * 100.0)
        } else {
            write!(f, "{:.0}%", self.0 * 100.0)
        }
    }
}

#[derive(Clone, Copy, defmt::Format)]
pub struct AdsrDuration(pub Duration);

impl EditByEncoder for AdsrDuration {
    type Meta = AdsrStage;

    fn edit_by_encoder(&mut self, offset: i32, vel: f32, stage: Self::Meta) -> &mut Self {
        let range = match stage {
            AdsrStage::Delay => values::DELAY_DURATION_RANGE,
            AdsrStage::Attack => values::ATTACK_DURATION_RANGE,
            AdsrStage::Hold => values::HOLD_DURATION_RANGE,
            AdsrStage::Decay => values::DECAY_DURATION_RANGE,
            // Note: Does not have duration
            AdsrStage::Sustain => return self,
            AdsrStage::Release => values::RELEASE_DURATION_RANGE,
        };

        let current_value = self.0.as_millis();

        let offset_sign = offset.signum();
        let offset = {
            let val_pow: usize = if current_value == 0 {
                0
            } else {
                current_value.ilog10() as usize
            };
            (offset as f32
                * if val_pow >= values::DURATION_POWER_EDIT_OFFSET.len() {
                    values::DURATION_POWER_EDIT_OFFSET.last().copied().unwrap()
                } else {
                    values::DURATION_POWER_EDIT_OFFSET[val_pow]
                } as f32
                * vel) as i32
        };

        let offset = if offset == 0 { offset_sign } else { offset };

        self.0 = Duration::from_millis(
            (current_value as i32 + offset).clamp(range.start, range.end) as u64,
        );

        self
    }
}

impl From<Duration> for AdsrDuration {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

// TODO: Pretty print decimal points
impl core::fmt::Display for AdsrDuration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let micros = self.0.as_micros();

        if micros == 0 {
            return write!(f, "0");
        }

        if micros <= 999 {
            return write!(f, "{micros}us");
        }

        let millis = micros / 1_000;
        if millis <= 999 {
            return write!(f, "{millis}ms");
        }

        let secs = millis as f32 / 1_000.0;

        if secs < 10.0 {
            write!(f, "{secs:.1}s")
        } else {
            write!(f, "{secs:.0}s")
        }
    }
}

#[derive(Clone, Copy, defmt::Format)]
pub struct AdsrCurveBend(f32);

impl AdsrCurveBend {
    pub fn new(bend: f32) -> Self {
        assert!(values::CURVE_BEND_RANGE.contains(&bend));
        Self(bend)
    }

    pub fn fun(&self, x: f32) -> f32 {
        assert!(x >= 0.0 && x <= 1.0);

        //
        if x.abs() < values::CURVE_BEND_ZERO_ERROR {
            0.0
        } else if self.0.abs() < values::CURVE_BEND_ZERO_ERROR {
            // If x is near zero -- go linear
            x
        } else {
            (1.0 - (x.abs() * self.0 * values::CURVE_BEND_MAX_TENSION).exp())
                / (1.0 - (self.0 * values::CURVE_BEND_MAX_TENSION).exp())
                * x.signum()
        }
    }
}

impl EditByEncoder for AdsrCurveBend {
    type Meta = ();

    fn edit_by_encoder(&mut self, offset: i32, vel: f32, meta: Self::Meta) -> &mut Self {
        let offset = offset as f32 * values::CURVE_BEND_EDIT_MAX_STEP * vel;

        self.0 = (self.0 + offset).clamp(
            *values::CURVE_BEND_RANGE.start(),
            *values::CURVE_BEND_RANGE.end(),
        );

        self
    }
}

impl core::fmt::Display for AdsrCurveBend {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, defmt::Format)]
pub struct DurationSlope {
    pub duration: AdsrDuration,
    pub bend: AdsrCurveBend,
}

impl core::fmt::Display for DurationSlope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.duration)
    }
}

#[derive(Clone, Copy, PartialEq, defmt::Format)]
pub enum AdsrStage {
    Delay,
    Attack,
    Hold,
    Decay,
    Sustain,
    Release,
}

impl AdsrStage {
    pub fn index(&self) -> usize {
        match self {
            AdsrStage::Delay => 0,
            AdsrStage::Attack => 1,
            AdsrStage::Hold => 2,
            AdsrStage::Decay => 3,
            AdsrStage::Sustain => 4,
            AdsrStage::Release => 5,
        }
    }

    pub fn each() -> impl Iterator<Item = Self> {
        [
            Self::Delay,
            Self::Attack,
            Self::Hold,
            Self::Decay,
            Self::Sustain,
            Self::Release,
        ]
        .iter()
        .copied()
    }
}

#[derive(Clone, Copy, defmt::Format)]
pub enum AdsrStageValue {
    Duration(AdsrDuration),
    DurationSlope(DurationSlope),
    Amplitude(AdsrAmplitude),
}

impl From<AdsrDuration> for AdsrStageValue {
    fn from(value: AdsrDuration) -> Self {
        Self::Duration(value)
    }
}

impl From<DurationSlope> for AdsrStageValue {
    fn from(value: DurationSlope) -> Self {
        Self::DurationSlope(value)
    }
}

impl From<AdsrAmplitude> for AdsrStageValue {
    fn from(value: AdsrAmplitude) -> Self {
        Self::Amplitude(value)
    }
}

impl core::fmt::Display for AdsrStageValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AdsrStageValue::Duration(dur) => write!(f, "{dur}"),
            AdsrStageValue::DurationSlope(ds) => write!(f, "{ds}"),
            AdsrStageValue::Amplitude(amp) => write!(f, "{amp}"),
        }
    }
}

#[derive(defmt::Format)]
pub struct Adsr {
    pub delay: AdsrDuration,
    pub attack: DurationSlope,
    pub hold: AdsrDuration,
    pub decay: DurationSlope,
    pub sustain: AdsrAmplitude,
    pub release: DurationSlope,
}

impl Adsr {
    pub fn stage(&self, stage: AdsrStage) -> AdsrStageValue {
        match stage {
            AdsrStage::Delay => AdsrStageValue::Duration(self.delay),
            AdsrStage::Attack => AdsrStageValue::DurationSlope(self.attack),
            AdsrStage::Hold => AdsrStageValue::Duration(self.hold),
            AdsrStage::Decay => AdsrStageValue::DurationSlope(self.decay),
            AdsrStage::Sustain => AdsrStageValue::Amplitude(self.sustain),
            AdsrStage::Release => AdsrStageValue::DurationSlope(self.attack),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = AdsrStageValue> + '_ {
        AdsrStage::each().map(|stage| self.stage(stage))
    }
}

#[derive(Clone, Copy)]
pub struct AdsrStages<T> {
    delay: T,
    attack: T,
    hold: T,
    decay: T,
    sustain: T,
    release: T,
}

impl<T> AdsrStages<T> {
    pub fn stage(&self, stage: AdsrStage) -> &T {
        match stage {
            AdsrStage::Delay => &self.delay,
            AdsrStage::Attack => &self.attack,
            AdsrStage::Hold => &self.hold,
            AdsrStage::Decay => &self.decay,
            AdsrStage::Sustain => &self.sustain,
            AdsrStage::Release => &self.release,
        }
    }
}

impl<T> AdsrStages<T>
where
    T: Copy,
{
    pub fn iter(&self) -> impl Iterator<Item = T> {
        [
            self.delay,
            self.attack,
            self.hold,
            self.decay,
            self.sustain,
            self.release,
        ]
        .into_iter()
    }
}
