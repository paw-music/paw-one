use super::{
    event::{self, Event, EventResponse},
    UiCtx,
};

pub trait Widget<'a, Env> {
    fn update(&mut self, event: Event, ctx: &mut UiCtx<'a, Env>) -> EventResponse;
}
