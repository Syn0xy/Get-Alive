use std::time::Duration;

#[derive(Debug)]
pub enum DisplayEvent {
    RedrawRequested { elapsed_time: Duration },
}
