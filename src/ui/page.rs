use super::{
    event::{Event, EventResponse},
    widget::Widget,
};

// #[derive(Clone, Copy)]
// pub enum Pages {
//     Preset,
//     Osc,
// }

pub trait Page<'a, Env>: Widget<'a, Env> {}
