use actule::ncollide::ncollide_geometry::query::{Contact};
use actule::nalgebra::{Point2, Vector2, dot, Translation};
use actule::actule::*;

use utils::redefines::*;

pub struct PhysicsObj {
    mass: Mass,
    i_mass: Mass,
    velocity: Vector2<Coord>,
    bounciness: f64,
    pos_correction: Option<(Contact<Point2<Coord>>, Id)>,
}

impl PhysicsObj {
    pub fn new(mass: Mass, velocity: Vector2<Coord>, bounciness: f64) -> PhysicsObj {
        if mass == 0.0 {
            PhysicsObj {
                mass: 0.0,
                i_mass: 0.0,
                velocity: velocity,
                bounciness: bounciness,
                pos_correction: None,
            }
        } else {
            PhysicsObj {
                mass: mass,
                i_mass: 1.0 / mass,
                velocity: velocity,
                bounciness: bounciness,
                pos_correction: None,
            }
        }
    }

    pub fn set_velocity(&mut self, velocity: Vector2<Coord>) {
        self.velocity = velocity;
    }
    pub fn clone_velocity(&self) -> Vector2<Coord> {
        self.velocity.clone()
    }

    pub fn collision(&mut self, other_id: Id, other: &mut Box<PhysicsObj>, contact: Contact<Point2<Coord>>) {
        let vel_along_normal = dot(&(other.velocity - self.velocity), &contact.normal);

        if vel_along_normal > 0.0 {
            return;
        }

        let impulse = (
            -(1.0 + self.bounciness.min(other.bounciness)) * vel_along_normal
        ) / (
            self.i_mass + other.i_mass
        ) * contact.normal;

        let mass_sum = self.mass + other.mass;

        self.velocity -= self.mass / mass_sum * impulse;
        other.velocity += other.mass / mass_sum * impulse;

        self.pos_correction = Some((contact, other_id));
    }

    pub fn tick(&mut self, world: &mut SWorld, transform: &mut Box<Transform>, dt: f64) {
        match self.pos_correction.take() {
            Some((contact, other_id)) =>  {
                let percent = 0.2;
                let slop = 0.01;
                let mut other = world.get_mut_entity_by_id(other_id).expect("Other entity was none");
                let other_i_mass = {
                    let other_physics_obj = other.get_physics_obj().expect("Physics Obj was none");
                     other_physics_obj.i_mass
                };
                let correction = 0.0f64.max(contact.depth - slop) / (self.i_mass + other_i_mass) * percent * contact.normal;
                transform.get_mut_isometry().append_translation_mut(&(self.i_mass * correction * -1.0));
                other.get_mut_transform().expect("Other transform was none").get_mut_isometry().append_translation_mut(&(other_i_mass * correction));
            },
            None => (),
        }
        transform.get_mut_isometry().append_translation_mut(&(self.velocity * dt));
    }
}
