
use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2};
use actule::actule::*;

pub struct Hitbox {
    shape_handle: ShapeHandle2<Coord>,
}

impl Hitbox {
    pub fn new(shape_handle: ShapeHandle2<Coord>) -> Hitbox {
        Hitbox {
            shape_handle: shape_handle,
        }
    }

    pub fn get_shape(&self) -> ShapeHandle2<Coord> {
        self.shape_handle.clone()
    }
}
