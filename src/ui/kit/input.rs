use crate::ui::Focus;

pub trait InputEl: Focus {
    type Value;

    fn value(&self) -> &Self::Value;
}
