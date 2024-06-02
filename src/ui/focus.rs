pub trait Focus {
    fn focused(&self) -> bool;
    fn set_focus(&mut self, focus: bool);

    fn focus(&mut self) {
        self.set_focus(true)
    }

    fn blur(&mut self) {
        self.set_focus(false)
    }
}
