use crate::ui::focus::Focus;

pub trait InputEl: Focus {
    type Value;

    fn value(&self) -> &Self::Value;
}
