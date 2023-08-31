pub trait Monitor {
    fn monitor(&mut self);
}

pub trait MonitorState {
    fn print_state(&self);
}
