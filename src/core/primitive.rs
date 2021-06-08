use crate::core::error::GameError;

pub struct State {
    persistent: bool,
    duration: i32,
    row: i32,
    frame_buffer: f64,
    id: Box<str>,
    is_cancellable: bool,
}

impl State {
    pub fn new(
        &self,
        duration: i32,
        row: i32,
        persistent: bool,
        id: Box<str>,
        frame_buffer: f64,
    ) -> State {
        State {
            persistent,
            duration,
            row,
            frame_buffer,
            id,
            is_cancellable: false,
        }
    }
    pub fn set_frame_buffer(&mut self, buffer: f64) {
        if buffer > 0.9 {
            self.frame_buffer = 0.9;
            GameError::warn(&"A frame buffer greater .9 will result in the frame always being 0.");
            return;
        }
        self.frame_buffer = buffer
    }
}
