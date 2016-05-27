
use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2, Cuboid};
use actule::nalgebra::{Vector2};
use actule::actule::*;

pub struct Hitbox {
    shape_handle: ShapeHandle2<Coord>,
}

impl Hitbox {
    pub fn new() -> Hitbox {
        Hitbox {
            shape_handle: ShapeHandle2::new(Cuboid::new(Vector2::new(1.0, 1.0))),
        }
    }

    pub fn get_shape(&self) -> ShapeHandle2<Coord> {
        self.shape_handle.clone()
    }
}
