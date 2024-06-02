pub enum Encoder {
    Main,
    Red,
    Green,
}

pub enum ButtonEvent {
    Press,
}

pub enum Event {
    Encoder(Encoder, i32, f32),
    EncoderButton(Encoder, ButtonEvent),
}

pub enum EventCaptured {
    Captured,
}

pub enum EventIgnored {
    Ignored,
}

pub type EventResponse = Result<EventIgnored, EventCaptured>;
