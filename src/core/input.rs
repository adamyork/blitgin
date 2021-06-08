use std::collections::hash_map::RandomState;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    pub waits: HashMap<String, i32, RandomState>,
    pub disabled: bool,
    pub direction: i32,
    pub v_direction: i32,
    pub jump: i32,
    pub jump_lock: bool,
    pub custom_key: i32,
}

impl Input {
    pub fn new() -> Input {
        Input {
            waits: HashMap::new(),
            disabled: false,
            direction: 0,
            v_direction: 0,
            jump: 0,
            jump_lock: false,
            custom_key: 0,
        }
    }
    pub fn has_waits_for(&self, key: &str) -> bool {
        self.waits.contains_key(key)
    }
    pub fn add_wait_for_action(&mut self, key: String, duration: i32) {
        &self.waits.insert(key, duration);
        return;
    }
}

#[derive(Debug)]
pub struct Keyboard {}

impl Keyboard {
    const KEY_UP: &'static str = "keyup";
    const KEY_DOWN: &'static str = "keydown";
    const LEFT: i32 = 65;
    const RIGHT: i32 = 68;
    const SPACE: i32 = 32;
}
