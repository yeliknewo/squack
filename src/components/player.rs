use actule::actule::*;
use actule::piston_window::*;

#[derive(Debug)]
pub struct Player {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
    fn set_left(&mut self, state: bool) {
        self.left = state;
    }
    fn set_right(&mut self, state: bool) {
        self.right = state;
    }
    fn set_up(&mut self, state: bool) {
        self.up = state;
    }
    fn set_down(&mut self, state: bool) {
        self.down = state;
    }

    pub fn get_left(&self) -> bool {
        self.left
    }
    pub fn get_right(&self) -> bool {
        self.right
    }
    pub fn get_up(&self) -> bool {
        self.up
    }
    pub fn get_down(&self) -> bool {
        self.down
    }

    pub fn tick(&mut self, minput: &Minput) {
        println!("{:?}", self);
        match minput.get_key(Key::W) {
            KeyState::Pressed => self.set_up(true),
            KeyState::Released => self.set_up(false),
        }
        match minput.get_key(Key::A) {
            KeyState::Pressed => self.set_left(true),
            KeyState::Released => self.set_left(false),
        }
        match minput.get_key(Key::S) {
            KeyState::Pressed => self.set_down(true),
            KeyState::Released => self.set_down(false),
        }
        match minput.get_key(Key::D) {
            KeyState::Pressed => self.set_right(true),
            KeyState::Released => self.set_right(false),
        }
    }
}
