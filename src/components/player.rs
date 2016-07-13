use actule::actule::*;
use actule::piston_window::*;

use components::*;

const JUMP_WAIT: f64 = 1.0;

#[derive(Debug)]
pub struct Player {
    can_jump: bool,
    jump_timer: f64
}

impl Player {
    pub fn new() -> Player {
        Player {
            can_jump: true,
            jump_timer: 0.0,
        }
    }

    pub fn tick(&mut self, minput: &Minput, physics_obj: &mut Box<PhysicsObj>, dt: f64) {

        self.jump_timer += dt;

        let mut velocity = physics_obj.clone_velocity();

        if minput.get_key(Key::D) == KeyState::Pressed {
            velocity.x = 200.0;
        }
        else if minput.get_key(Key::A) == KeyState::Pressed {
            velocity.x = -200.0;
        }
        else {
            velocity.x = 0.0;
        }


        if minput.get_key(Key::S) == KeyState::Pressed && velocity.y <= 100.0 {
            velocity.y = 50.0;
        }
        if minput.get_key(Key::W) == KeyState::Pressed {
            if self.jump_timer >= JUMP_WAIT {
                self.can_jump = true;
                self.jump_timer = 0.0;
            }

            if self.can_jump {
                velocity.y = -400.0;
                self.can_jump = false;
            }
        }
        velocity.y += 10.0;

        physics_obj.set_velocity(velocity);

    }
}
