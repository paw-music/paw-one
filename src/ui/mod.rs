use embedded_ui::event::CommonEvent;

pub mod fps;
pub mod logo;

#[derive(Clone)]
pub enum Message {}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    MainEncChange(i32, f32),
    MainEncClickDown,
    MainEncClickUp,
    RedEncChange(i32, f32),
    GreenEncChange(i32, f32),
}

impl embedded_ui::event::Event for Event {
    fn as_common(&self) -> Option<CommonEvent> {
        match self {
            &Event::MainEncChange(offset, _) => Some(CommonEvent::FocusMove(offset)),
            &Event::RedEncChange(_, _) => None,
            Event::MainEncClickDown => Some(CommonEvent::FocusClickDown),
            Event::MainEncClickUp => Some(CommonEvent::FocusClickUp),
            &Event::GreenEncChange(_, _) => None,
        }
    }

    fn as_select_shift(&self) -> Option<i32> {
        match self {
            &Event::MainEncChange(offset, _) => Some(offset),
            _ => None,
        }
    }

    fn as_slider_shift(&self) -> Option<i32> {
        match self {
            &Event::MainEncChange(offset, _) => Some(offset),
            _ => None,
        }
    }

    fn as_knob_rotation(&self) -> Option<i32> {
        match self {
            &Event::MainEncChange(offset, _) => Some(offset),
            _ => None,
        }
    }

    fn as_input_letter_scroll(&self) -> Option<i32> {
        match self {
            &Event::MainEncChange(offset, _) => Some(offset),
            _ => None,
        }
    }
}

impl From<embedded_ui::event::CommonEvent> for Event {
    fn from(value: embedded_ui::event::CommonEvent) -> Self {
        match value {
            CommonEvent::FocusMove(offset) => Self::MainEncChange(offset, 1.0),
            CommonEvent::FocusClickDown => Self::MainEncClickDown,
            CommonEvent::FocusClickUp => Self::MainEncClickUp,
        }
    }
}
