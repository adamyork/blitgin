use crate::core::error::GameError;
use std::cmp;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
    pub fn left(&self) -> i32 {
        self.x
    }
    pub fn top(&self) -> i32 {
        self.y
    }
    pub fn right(&self) -> i32 {
        self.x + self.width
    }
    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }
    pub fn intersects(&self, rect: Rectangle) -> bool {
        let lateral = (&self.left() < &rect.left() && &rect.left() < &self.right())
            || (&self.left() < &rect.right() && &rect.right() < &self.right());
        let vertical = (&self.top() < &rect.top() && &rect.top() < &self.bottom())
            || (&self.top() < &rect.bottom() && &rect.bottom() < &self.bottom());
        lateral && vertical
    }
    pub fn intersection(&self, rect: Rectangle) -> Rectangle {
        Rectangle {
            x: cmp::max(rect.left(), self.left()),
            y: cmp::max(rect.top(), self.top()),
            width: cmp::min(rect.right(), self.right()),
            height: cmp::min(rect.bottom(), self.bottom()),
        }
    }
}
