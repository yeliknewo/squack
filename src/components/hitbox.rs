use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2};
use actule::actule::*;

use utils::*;

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

    pub fn tick(&self, id: Id, world: &mut SWorld, transform: &Transform) {
        world.get_mut_entity_by_name(WATCHER_NAME).expect("Watcher was none").get_mut_hit_watcher().expect("Entity had no hit watcher").update_hitbox(id, transform.get_isometry().clone());
    }
}
