use actule::ncollide::ncollide_geometry::query::{Contact};
use actule::nalgebra::{Point2, Vector2};
use actule::actule::*;

use utils::*;

pub struct PhysicsObj {
    mass: Mass,
    velocity: Vector2<Coord>,
}

impl PhysicsObj {
    pub fn new(mass: Mass, velocity: Vector2<Coord>) -> PhysicsObj {
        PhysicsObj {
            mass: mass,
            velocity: velocity,
        }
    }

    pub fn collision(&mut self, other: &mut Box<PhysicsObj>, contact: Contact<Point2<Coord>>) {

    }
}
