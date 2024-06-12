use debouncr::debounce_2;
use embedded_hal::digital::v2::InputPin;

#[derive(Clone, Copy, Debug, Default, defmt::Format)]
pub enum BtnState {
    #[default]
    None,
    Up,
    Down,
}

pub struct PullUp;
pub struct PullDown;

pub trait PullEdge {
    fn to_state(&self, edge: debouncr::Edge) -> BtnState;
}

impl PullEdge for PullUp {
    fn to_state(&self, edge: debouncr::Edge) -> BtnState {
        match edge {
            debouncr::Edge::Rising => BtnState::Up,
            debouncr::Edge::Falling => BtnState::Down,
        }
    }
}

impl PullEdge for PullDown {
    fn to_state(&self, edge: debouncr::Edge) -> BtnState {
        match edge {
            debouncr::Edge::Rising => BtnState::Down,
            debouncr::Edge::Falling => BtnState::Up,
        }
    }
}

pub struct Btn<P: InputPin, Pull: PullEdge> {
    pin: P,
    debouncer: debouncr::Debouncer<u8, debouncr::Repeat2>,
    pull: Pull,
}

impl<P: InputPin, Pull: PullEdge> Btn<P, Pull> {
    pub fn new(pin: P, pull: Pull) -> Self {
        Self {
            pin,
            debouncer: debounce_2(false),
            pull,
        }
    }
}

pub trait AnyBtn {
    fn tick(&mut self) -> BtnState;
}

impl<P: InputPin, Pull: PullEdge> AnyBtn for Btn<P, Pull> {
    fn tick(&mut self) -> BtnState {
        match self.debouncer.update(self.pin.is_high().ok().unwrap()) {
            Some(edge) => self.pull.to_state(edge),
            None => BtnState::None,
        }
    }
}
