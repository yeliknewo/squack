use actule::ncollide::ncollide_geometry::query::{Contact};
use actule::nalgebra::{Point2, Vector2, dot, Translation};
use actule::actule::*;

use utils::*;

pub struct PhysicsObj {
    mass: Mass,
    i_mass: Mass,
    velocity: Vector2<Coord>,
    bounciness: f64,
}

impl PhysicsObj {
    pub fn new(mass: Mass, velocity: Vector2<Coord>, bounciness: f64) -> PhysicsObj {
        PhysicsObj {
            mass: mass,
            i_mass: 1.0 / mass,
            velocity: velocity,
            bounciness: bounciness,
        }
    }

    pub fn collision(&mut self, other: &mut Box<PhysicsObj>, contact: Contact<Point2<Coord>>) {
        let rv = other.velocity - self.velocity;

        let vel_along_normal = dot(&rv, &contact.normal);

        if vel_along_normal > 0.0 {
            return;
        }

        let e = self.bounciness.min(other.bounciness);

        let j = (
            -(1.0 + e) * vel_along_normal
        ) / (
            self.i_mass + other.i_mass
        );

        let impulse = j * contact.normal;

        self.velocity -= self.i_mass * impulse;
        other.velocity += other.i_mass * impulse;
    }

    pub fn tick(&mut self, transform: &mut Box<Transform>, dt: f64) {
        transform.get_mut_isometry().append_translation_mut(&(self.velocity * dt));
    }
}
